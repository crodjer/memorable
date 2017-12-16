use diesel::result::Error as DieselError;
use iron::error::IronError;
use std::env;
use std::error::Error;
use std::fmt;
use url::ParseError;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    AlreadyExists(String),
    DB(DieselError),
    UrlParseError(ParseError),
    String(String)
}

impl AppError {
    fn new(msg: &str) -> AppError {
        AppError::String(msg.to_string())
    }
}

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

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<env::VarError> for AppError {
    fn from(err: env::VarError) -> Self {
        AppError::new(err.description())
    }
}

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

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::String(err)
    }
}

impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        AppError::UrlParseError(err)
    }
}

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
