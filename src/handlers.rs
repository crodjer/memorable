extern crate diesel;

use super::models;
use super::schema;

pub mod links {
    use diesel;
    use diesel::prelude::*;
    use models::links::{CreateLink,Link};
    use schema::links::table;

    pub fn create_link<'a>(conn: &PgConnection, url: &'a str,
                           key: Option<&'a str>, title: Option<&'a str>)
        -> Link {

        let mut link = CreateLink::new(url);
        if let Some(key) = key {
            // The user wants to use a custom key.
            link.customize(key);
        }
        if let Some(title) = title {
            // The link also has a title!.
            link.set_title(title);
        }

        diesel::insert(&link).into(table)
            .get_result(conn)
            .expect("Error inserting link")
    }
}
