pub mod links {
    use diesel;
    use diesel::prelude::*;
    use models::links::{CreateLink,Link};
    use schema::links::table;
    use url::{Url};


    pub fn insert_link(conn: &PgConnection, link: &mut CreateLink, try: u8) -> Result<Link, Box<::std::error::Error>> {
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

    pub fn create_link(conn: &PgConnection, url: String, key: Option<String>, title: Option<String>) -> Result<Link, Box<::std::error::Error>> {
        let url = Url::parse(url.as_str())?;
        let mut link = CreateLink::new(url);

        if let Some(key) = key {
            // The user wants to use a custom key.
            link.customize(key);
        }
        if let Some(title) = title {
            // The link also has a title!.
            link.set_title(title);
        }
        insert_link(conn, &mut link, 0)
    }

    pub fn get_link(conn: &PgConnection, shortened_key: String) -> Result<Link, Box<::std::error::Error>> {
        use schema::links::dsl::*;

        match links.filter(key.eq(shortened_key))
            .first::<Link>(conn) {
                Ok(link) => Ok(link),
                Err(e) => Err(Box::new(e))
            }
    }
}
