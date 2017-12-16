use db;
use error::AppError;
use handlers::links;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::{Url,status};
use router::Router;
use params::{Params, FromValue};
use middleware::diesel::DieselReqExt;
use url;

/// Respond with a PONG when called. To be used as a health check.
fn pong (_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "PONG")))
}

fn get_link (req: &mut Request) -> IronResult<Response> {
    let key = req.extensions.get::<Router>().unwrap().find("key").unwrap();
    let conn = req.db_conn();
    let link = links::get_link(&conn, key.to_owned())?;
    let url = Url::parse(&link.url).map_err(AppError::from)?;
    Ok(Response::with((status::Found, Redirect(url))))
}

fn create_link (req: &mut Request) -> IronResult<Response> {
    let map = req.get::<Params>().unwrap();
    let conn = req.db_conn();
    let url: String = map.find(&["url"])
        .and_then(FromValue::from_value)
        .map(Ok)
        .unwrap_or(Err(AppError::BadRequest("Param 'url' is required."
                                            .to_owned())))?;
    let title = map.find(&["title"]).and_then(FromValue::from_value);
    let custom_key = map.find(&["custom-key"]).and_then(FromValue::from_value);
    let link = links::create_link(&conn, url.to_owned(), title, custom_key)?;

    let mut short_url: url::Url = req.url.clone().into();
    short_url.set_query(None);
    short_url.set_path(format!("/{}", link.key).as_str());

    Ok(Response::with((status::Ok, short_url.as_str())))
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
