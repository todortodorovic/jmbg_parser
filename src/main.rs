mod errors;
mod parser;
mod utils;
use clap::Parser;
use parser::parse_jmbg;
use parser::validate_jmbg;
use std::process;

#[derive(Parser)]
#[command(name = "JMBG Validator")]
#[command(about = "Program for validating JMBG using the Clap library", long_about = None)]
struct Cli {
    #[arg(short, long)]
    jmbg: String,
}

fn main() {
    let args = Cli::parse();

    let jmbg = &args.jmbg;

    let jmbg = jmbg.trim();

    match validate_jmbg(jmbg) {
        Ok(()) => println!("JMBG is valid."),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }

    match parse_jmbg(jmbg) {
        Ok(person) => {
            println!("Date of birth: {}", person.date_of_birth);
            println!("Region: {:?}", person.region);
            println!("Unique number: {}", person.unique_number);
            println!("Gender: {:?}", person.gender);
        }
        Err(e) => {
            println!("Error parsing JMBG: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{Gender, Region};

    #[test]
    fn test_valid_jmbg() {
        let jmbg = "2708005798914";
        assert!(validate_jmbg(jmbg).is_ok());
    }

    #[test]
    fn test_parse_jmbg() {
        let jmbg = "2708005798914";
        let person = parse_jmbg(jmbg).expect("Expected valid JMBG");

        // Check date of birth
        assert_eq!(person.date_of_birth.to_string(), "2005-08-27");

        // Check region
        assert_eq!(person.region, Region::Uzice);

        // Check unique number
        assert_eq!(person.unique_number, 891);

        // Check calculated control digit
        assert_eq!(person.calculate_control_digit().unwrap(), 4);

        // Check gender
        assert_eq!(person.gender, Gender::Female);
    }

    #[test]
    fn test_invalid_control_digit() {
        let jmbg = "2708005798913"; // Incorrect control digit
        assert!(validate_jmbg(jmbg).is_err());
    }

    #[test]
    fn test_invalid_jmbg_length() {
        let jmbg = "270800579891"; // Missing control digit
        assert!(validate_jmbg(jmbg).is_err());
    }

    #[test]
    fn test_invalid_date() {
        let jmbg = "3102005798914"; // Invalid date (31st February)
        assert!(parse_jmbg(jmbg).is_err());
    }
}
