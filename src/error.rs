use std::{error::Error, fmt::Display};
#[derive(Serialize, Debug)]
pub enum StratError {
    // Database Errors
    DbFailed,
    // User Errors
    UserNotFound,
    EmailInUse,
    UniqueExists,
    NameExists,
    Unknown,
    BadLogin,
    // Auth Errors
    AuthFailed,
    UnknownToken,
    TokenExpired,
    AuthExpired,
    UnknownRefresh,
    InvalidToken,
    InvalidRefresh,
    // Multipart
    BadMulti,
    OversizedField(String, u64),
    MediaUnsupported,
    // Post Errors
    UnknownPost,
    NoPermission,
    NeedsContent,
    // This Error is for testing only!
    Custom(String),
}
impl Display for StratError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StratError::DbFailed => {
                write!(f, "A server error has occured! Please try again later.")
            }
            StratError::UserNotFound => write!(f, "The requested user could not be found."),
            StratError::EmailInUse => write!(f, "The requested email is already in use!"),
            StratError::UniqueExists => write!(
                f,
                "An issue with the database has occured, please try again."
            ),
            StratError::NameExists => write!(f, "The requested username is already in use."),
            StratError::Unknown => write!(f, "An unknown error has occured!"),
            StratError::BadLogin => write!(f, "The Email or Password submitted is invalid!"),
            StratError::UnknownToken => {
                write!(f, "The Token provided could not be linked to a session!")
            }
            StratError::TokenExpired => write!(f, "The Token provided has expired!"),
            StratError::AuthExpired => write!(
                f,
                "The Authorization Token linked to this token has expired."
            ),
            StratError::UnknownRefresh => write!(
                f,
                "The Refresh Token provided could not be linked to a session!"
            ),
            StratError::AuthFailed => write!(f, "An Error occured while verifying authentication."),
            StratError::InvalidToken => write!(
                f,
                "The Authorization Token provided is malformed or missing."
            ),
            StratError::InvalidRefresh => {
                write!(f, "The Refresh Token provided is malformed or missing.")
            }
            StratError::BadMulti => {
                write!(f, "This request must be a valid Multipart Request")
            }
            StratError::OversizedField(name, size) => {
                write!(
                    f,
                    "The Field: {} excees the maximum size of: {}",
                    name, size
                )
            }
            StratError::MediaUnsupported => {
                write!(
                    f,
                    "The Multipart Request contains an unsupported media type!"
                )
            }
            StratError::UnknownPost => {
                write!(f, "The requested Post could not be found.")
            }
            StratError::NoPermission => {
                write!(f, "The Authenticated User is not the owner of this post.")
            }
            StratError::Custom(val) => {
                write!(f, "{}", val)
            }
            StratError::NeedsContent => {
                write!(f, "The post submitted contains no text!")
            }
        }
    }
}
impl Error for StratError {}
