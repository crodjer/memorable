extern crate rand;
extern crate url;

pub mod links {
    use schema::links;
    use super::rand::{thread_rng, Rng};
    use super::url::{Url};

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

        pub fn generate_key(&mut self) -> &mut CreateLink {
            self.key = generate_key();
            self
        }

        pub fn customize(&mut self, key: String) -> &mut CreateLink {
            self.key = key;
            self.is_custom = true;
            self
        }

        pub fn set_title(&mut self, title: String) -> &mut CreateLink {
            self.title = title;
            self
        }
    }
}
