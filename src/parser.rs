use crate::errors::JmbgError;
use crate::utils::{Gender, Region};
use chrono::NaiveDate;
use serde::Serialize;
/// Structure that represents a person's data based on the JMBG
///
#[derive(Debug, Serialize)]
pub struct Person {
    pub date_of_birth: NaiveDate, // NaiveDate for date of birth
    pub region: Region,           // Region code
    pub unique_number: u32,       // Triple-digit number that defines the unique number
    pub gender: Gender,           // Gender: "Male" or "Female"
    jmbg: String,                 // Store the JMBG for calculating the control digit
}

impl Person {
    /// Method to calculate the control digit for this person
    pub fn calculate_control_digit(&self) -> Result<u32, JmbgError> {
        let jmbg = &self.jmbg;
        let digits: Vec<u32> = jmbg
            .chars()
            .take(12) // Take only the first 12 digits
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();

        if digits.len() != 12 {
            return Err(JmbgError::InvalidLength);
        }

        let s = 7 * digits[0]
            + 6 * digits[1]
            + 5 * digits[2]
            + 4 * digits[3]
            + 3 * digits[4]
            + 2 * digits[5]
            + 7 * digits[6]
            + 6 * digits[7]
            + 5 * digits[8]
            + 4 * digits[9]
            + 3 * digits[10]
            + 2 * digits[11];

        let m = s % 11;
        let k = match m {
            0 => 0,
            1 => return Err(JmbgError::InvalidControlDigit),
            _ => 11 - m,
        };

        Ok(k)
    }
}

/// Function for validating the entire JMBG
pub fn validate_jmbg(jmbg: &str) -> Result<(), JmbgError> {
    // Check if the JMBG is exactly 13 digits long
    if jmbg.len() != 13 {
        return Err(JmbgError::InvalidLength);
    }

    // Parse the JMBG into a `Person` instance
    let person = parse_jmbg(jmbg)?;

    // Parse the last (13th) digit as the control digit from the JMBG
    let control_digit = jmbg
        .chars()
        .nth(12)
        .ok_or(JmbgError::InvalidControlDigit)?
        .to_digit(10)
        .ok_or(JmbgError::InvalidControlDigit)?;

    // Calculate the control digit using the `Person` instance's method
    let calculated_digit = person.calculate_control_digit()?;

    // Compare the control digit
    if control_digit == calculated_digit {
        Ok(())
    } else {
        Err(JmbgError::InvalidControlDigit)
    }
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

    let year: u32 = if year_part > 25 {
        1000 + year_part
    } else {
        2000 + year_part
    };

    // Create the date of birth using NaiveDate
    let date_of_birth =
        NaiveDate::from_ymd_opt(year as i32, month, day).ok_or(JmbgError::InvalidDate)?;

    // Determine gender based on the unique number

    let gender = match unique_number.try_into() {
        Ok(gender) => gender,
        Err(_) => unreachable!(),
    };

    let region = Region::try_from(region).map_err(|_| JmbgError::UnknownRegion)?;

    Ok(Person {
        date_of_birth,
        region,
        unique_number,
        gender,
        jmbg: jmbg.to_string(),
    })
}
