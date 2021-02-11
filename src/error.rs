use std::{error::Error, fmt::Display};
#[derive(Serialize,Debug)]
pub enum StratError {
    //Database Errors
    DbFailed,
    //User Errors
    UserNotFound,
    EmailInUse,
    UniqueExists,
    NameExists,
    Unknown,
    BadLogin,
    //Auth Errors
    AuthFailed,
    UnknownToken,
    TokenExpired,
    AuthExpired,
    UnknownRefresh,
}
impl Display for StratError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StratError::DbFailed => write!(f, "A server error has occured! Please try again later."),
            StratError::UserNotFound => write!(f, "The requested user could not be found."),
            StratError::EmailInUse => write!(f, "The requested email is already in use!"),
            StratError::UniqueExists => write!(f, "An issue with the database has occured, please try again."),
            StratError::NameExists => write!(f, "The requested username is already in use."),
            StratError::Unknown => write!(f, "An unknown error has occured!"),
            StratError::BadLogin => write!(f, "The Email or Password submitted is invalid!"),
            StratError::UnknownToken => write!(f, "The Token provided could not be linked to a session!"),
            StratError::TokenExpired => write!(f, "The Token provided has expired!"),
            StratError::AuthExpired => write!(f, "The Authorization linked to this token has expired."),
            StratError::UnknownRefresh => write!(f, "The Refresh provided could not be linked to a session!"),
            StratError::AuthFailed => write!(f, "An Error occured while verifying authentication.")
        }
    }
}
impl Error for StratError{

}