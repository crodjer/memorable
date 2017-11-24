use db;
use handlers::links;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::{Url,status};
use router::Router;
use params::{Params, FromValue, Value};
use middleware::diesel::DieselReqExt;

/// Respond with a PONG when called. To be used as a health check.
fn pong (_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "PONG")))
}

fn get_link (req: &mut Request) -> IronResult<Response> {
    if let Some(key) = req.extensions.get::<Router>().unwrap().find("key") {
        let conn = req.db_conn();

        if let Ok(link) = links::get_link(&conn, key.to_owned()) {
            let url = Url::parse(&link.url).unwrap();
            Ok(Response::with((status::Found, Redirect(url))))
        } else {
            Ok(Response::with((status::NotFound)))
        }
    } else {
        Ok(Response::with((status::BadRequest, "Key to lookup is required")))
    }
}

fn create_link (req: &mut Request) -> IronResult<Response> {
    let map = req.get::<Params>().unwrap();
    let conn = req.db_conn();

    match map.find(&["url"]) {
        Some(&Value::String(ref url))  => {
            let title = map.find(&["title"])
                .and_then(FromValue::from_value);
            let custom_key = map.find(&["custom-key"])
                .and_then(FromValue::from_value);

            match links::create_link(&conn, url.to_owned(), title, custom_key) {
                Ok(link) => Ok(Response::with((status::Ok, link.key))),
                // This right now captures our errors as well.
                Err(_) => Ok(Response::with((status::BadRequest,
                                             "URL is invalid.")))
            }
        },
        _ => Ok(Response::with((status::BadRequest, "URL is required.")))
    }

}

pub fn run () {
    let mut router = Router::new();

    router.get("/ping", pong, "pong");
    router.post("/shorten", create_link, "create-link");
    router.get("/:key", get_link, "get-link");

    let mut chain = Chain::new(router);
    chain.link_before(db::diesel_middleware());

    Iron::new(chain).http("localhost:3000").unwrap();
}
