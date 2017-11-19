use super::schema;

pub mod links {
    use schema::links;

    #[derive(Queryable)]
    #[derive(Debug)]
    pub struct Link {
        pub id: i32,
        pub key: String,
        pub url: String,
        pub domain: String,
        pub title: String,
        pub is_custom: bool,
    }


    #[derive(Insertable)]
    #[derive(Debug)]
    #[table_name="links"]
    pub struct CreateLink<'a> {
        pub key: &'a str,
        pub url: &'a str,
        pub domain: &'a str,
        pub title: &'a str,
        pub is_custom: bool
    }

    impl <'a> CreateLink <'a> {
        pub fn new(url: &'a str) -> CreateLink {
            CreateLink {
                url: url,
                key: "",
                domain: "",
                title: "",
                is_custom: false
            }
        }

        pub fn customize<'b>(&'b mut self, key: &'a str)
            -> &'b mut CreateLink<'a> {
            self.key = key;
            self.is_custom = true;
            self
        }

        pub fn set_title<'b>(&'b mut self, title: &'a str)
            -> &'b mut CreateLink<'a> {
            self.title = title;
            self
        }
    }
}
