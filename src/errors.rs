use thiserror::Error;

#[derive(Error, Debug)]
pub enum JmbgError {
    #[error("JMBG must have exactly 13 digits")]
    InvalidLength,

    #[error("Control digit is not valid")]
    InvalidControlDigit,

    #[error("Invalid date of birth")]
    InvalidDate,

    #[error("Error converting the unique number to a number")]
    InvalidUniqueNumber,

    #[error("Unknown region")]
    UnknownRegion,
}
