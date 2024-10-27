use crate::errors::JmbgError;
use crate::utils::{determine_gender, map_region};
use chrono::NaiveDate;

/// Structure that represents a person's data based on the JMBG
pub struct Person {
    pub date_of_birth: NaiveDate, // NaiveDate for date of birth
    pub region: String,           // Region code
    pub unique_number: u32,       // Triple-digit number that defines the unique number
    pub control_digit: u32,       // Control digit
    pub gender: String,           // Gender: "Male" or "Female"
}

/// Function to parse the JMBG and return a `Person` structure
pub fn parse_jmbg(jmbg: &str) -> Result<Person, JmbgError> {
    // Check if JMBG has exactly 13 digits
    if jmbg.len() != 13 {
        return Err(JmbgError::InvalidLength);
    }

    // Parse the JMBG digits
    let day: u32 = jmbg[0..2].parse().map_err(|_| JmbgError::InvalidDate)?;
    let month: u32 = jmbg[2..4].parse().map_err(|_| JmbgError::InvalidDate)?;
    let year_part: u32 = jmbg[4..7].parse().map_err(|_| JmbgError::InvalidDate)?;
    let region: u32 = jmbg[7..9].parse().map_err(|_| JmbgError::UnknownRegion)?;
    let unique_number: u32 = jmbg[9..12]
        .parse()
        .map_err(|_| JmbgError::InvalidUniqueNumber)?;
    let control_digit: u32 = jmbg[12..13]
        .parse()
        .map_err(|_| JmbgError::InvalidControlDigit)?;

    let year = 1000 + year_part;

    // Create the date of birth using NaiveDate
    let date_of_birth =
        NaiveDate::from_ymd_opt(year as i32, month, day).ok_or(JmbgError::InvalidDate)?;

    // Determine gender based on the unique number
    let gender = determine_gender(unique_number);

    let region = map_region(region).to_string();
    // Return an instance of the `Person` structure
    Ok(Person {
        date_of_birth,
        region,
        unique_number,
        control_digit,
        gender,
    })
}
