// This is set so that `infer_schema` done by `src/schema.rs` does not fail.
#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;

pub mod schema;
pub mod models;
pub mod handlers;
pub mod db;
