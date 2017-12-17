//! Models to abstract database storage.

/// The models used for link storage.
pub mod links {
    use std::fmt;
    use schema::links;
    use rand::{thread_rng, Rng};
    use url::{Url};

    /// Link structure that directly maps to the database entries.
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

    impl fmt::Display for Link {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}\t{}", self.url, self.key)?;
            if !self.title.is_empty() {
                write!(f, "\t{} ", self.title)
            } else {
                write!(f, "")
            }
        }
    }


    /// Structure used to construct a link, to be inserted in the database.
    #[derive(Insertable)]
    #[derive(Debug)]
    #[table_name="links"]
    pub struct CreateLink {
        pub key: String,
        pub url: String,
        pub domain: String,
        pub title: String,
        pub is_custom: bool
    }

    // Valid characters to be used for key generation.
    // https://en.wikipedia.org/wiki/Base32#RFC_4648_Base32_alphabet
    static VALID_CHARS: [char; 32] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                                      'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                                      'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
                                      'y', 'z', '2', '3', '4', '5', '6', '7'];

    static KEY_SIZE: usize = 7;

    fn generate_key() -> String {
        let mut rng = thread_rng();
        (0..KEY_SIZE).map(|_| {
            VALID_CHARS[rng.gen::<usize>() % 32]
        }).collect()
    }

    impl CreateLink {
        /// Given a URL, build a new create link object.
        pub fn new(url: Url) -> CreateLink {
            let host = url.host_str().unwrap_or("").to_owned();

            CreateLink {
                url: url.as_str().to_owned(),
                key: generate_key(),
                domain: host,
                title: "".to_owned(),
                is_custom: false
            }
        }

        /// Generate a new key and update the link object.
        pub fn generate_key(&mut self) -> &mut CreateLink {
            self.key = generate_key();
            self
        }

        /// Customise the key for the object, instead of using auto generated
        /// key.
        pub fn customize(&mut self, key: String) -> &mut CreateLink {
            self.key = key;
            self.is_custom = true;
            self
        }

        /// Set the title for the object when provided.
        pub fn set_title(&mut self, title: String) -> &mut CreateLink {
            self.title = title;
            self
        }
    }
}
