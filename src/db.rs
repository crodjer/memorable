//! Database utilities: Configuration and connection.
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use middleware::diesel::DieselMiddleware;
use std::env;


/// Extract the database URL from the environment. Uses either the SHELL
/// environment variable `DATABASE_URL` or reads the `.env` file.
pub fn database_url () -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")

}

/// Create a new DieselMiddleware using the database URL extracted via the
/// function `database_url`.
pub fn diesel_middleware() -> DieselMiddleware<PgConnection>{
    DieselMiddleware::new(&database_url())
        .expect("Couldn't create Diesel Middleware")
}

/// Establish connection to the database URL from enviornment. Used by the CLI
/// utility. Creates a single connection, unlike the middleware above.
pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&database_url())
        .expect(&format!("Error connecting to {}", &database_url()))
}
