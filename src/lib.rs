// This is set so that `infer_schema` done by `src/schema.rs` does not fail.
#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate iron;
extern crate params;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate router;
extern crate url;

pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod server;
