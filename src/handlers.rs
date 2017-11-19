extern crate diesel;
extern crate url;

pub mod links {
    use diesel;
    use diesel::prelude::*;
    use models::links::{CreateLink,Link};
    use schema::links::table;
    use super::url::{Url};


    pub fn insert_link<'a>(conn: &PgConnection, link: &mut CreateLink, try: u8) -> Result<Link, Box<::std::error::Error>> {
        match diesel::insert(link).into(table).get_result(conn) {
            Ok(link) => Ok(link),
            Err(e) => {
                if link.is_custom || try > 2 {
                    Err(Box::new(e))
                } else {
                    link.generate_key();
                    insert_link(conn, link, try + 1)
                }
            }
        }
    }

    pub fn create_link<'a>(conn: &PgConnection, url: &'a str, key: Option<&'a str>, title: Option<&'a str>) -> Result<Link, Box<::std::error::Error>> {
        let url = Url::parse(url)?;
        let mut link = CreateLink::new(url);

        if let Some(key) = key {
            // The user wants to use a custom key.
            link.customize(key.to_owned());
        }
        if let Some(title) = title {
            // The link also has a title!.
            link.set_title(title.to_owned());
        }
        insert_link(conn, &mut link, 0)
    }

    pub fn get_link<'a>(conn: &PgConnection, shortened_key: &'a str) -> Result<Link, Box<::std::error::Error>> {
        use schema::links::dsl::*;

        match links.filter(key.eq(shortened_key))
            .first::<Link>(conn) {
                Ok(link) => Ok(link),
                Err(e) => Err(Box::new(e))
            }
    }
}
