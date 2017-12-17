//! Application specific error handling.
use diesel::result::Error as DieselError;
use iron::error::IronError;
use std::env;
use std::error::Error;
use std::fmt;
use url::ParseError;

/// An error specific to Memorable. Encapsulates various possible errors within
/// the application or from the libraries.
#[derive(Debug)]
pub enum AppError {
    /// Document not found in database. Results in a 404.
    NotFound,

    /// Server received a bad request because of the reason (the String
    /// argument). Results in a 402.
    BadRequest(String),

    /// The key used already exists in the database. Results in a 402.
    AlreadyExists(String),

    /// General databases errors from Diesel. Result in 500x
    DB(DieselError),

    /// Failed to parse the provided URL. Results in a 402.
    UrlParseError(ParseError),

    /// General errors, specified as String. Results in a 500.
    String(String)
}

/// Implement the Error trait so that it works conveniently with Box.
impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::NotFound              => "Document Not Found",
            AppError::BadRequest(ref e)     => e.as_str(),
            AppError::AlreadyExists(ref e)  => e.as_str(),
            AppError::DB(ref e)             => e.description(),
            AppError::UrlParseError(ref e)  => e.description(),
            AppError::String(ref e)         => e.as_str()
        }
    }
}

/// Render an AppError.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Define conversion of var errors to AppError.
impl From<env::VarError> for AppError {
    fn from(err: env::VarError) -> Self {
        AppError::String(err.description().to_owned())
    }
}

/// Define conversion of diesel errors to AppError. Handle NotFound and
/// UniqueViolation specially as they are used in the business logic.
impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        use diesel::result::DatabaseErrorKind::*;

        match err {
            DieselError::NotFound                          => AppError::NotFound,
            DieselError::DatabaseError(UniqueViolation, e) => {
                AppError::AlreadyExists(e.message().to_owned())
            },
            err                                            => AppError::DB(err)
        }
    }
}

/// Define conversion of string error (from iron::Url) to AppError.
impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::String(err)
    }
}

/// Define conversion of ParseError to AppError.
impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        AppError::UrlParseError(err)
    }
}

/// Define conversion of AppError to IronError. This is so that our server can
/// transparently generate the correct status code given an error.
impl From<AppError> for IronError {
    fn from(err: AppError) -> IronError {
        use iron::status;

        let modifier = match err {
            AppError::NotFound          => status::NotFound,
            AppError::BadRequest(_)     => status::BadRequest,
            AppError::AlreadyExists(_)  => status::Conflict,
            AppError::DB(_)             => status::InternalServerError,
            AppError::UrlParseError(_)  => status::BadRequest,
            AppError::String(_)         => status::InternalServerError
        };

        IronError::new(err, modifier)
    }
}
