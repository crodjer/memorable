use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use middleware::diesel::DieselMiddleware;
use std::env;


pub fn database_url () -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")

}

pub fn diesel_middleware() -> DieselMiddleware<PgConnection>{
    DieselMiddleware::new(&database_url())
        .expect("Couldn't create Diesel Middleware")
}

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&database_url())
        .expect(&format!("Error connecting to {}", &database_url()))
}
