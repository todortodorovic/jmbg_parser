use thiserror::Error;

#[derive(Error, Debug)]
pub enum JmbgError {
    #[error("JMBG mora imati tačno 13 cifara")]
    InvalidLength,

    #[error("Kontrolna cifra nije validna")]
    InvalidControlDigit,

    #[error("Nevažeći datum rođenja")]
    InvalidDate,

    #[error("Greška pri konvertovanju jedinstvenog broja u broj")]
    InvalidUniqueNumber,

    #[error("Nepoznat region")]
    UnknownRegion,
}
