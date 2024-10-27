mod errors;
mod parser;
mod utils;
use clap::Parser;
use parser::parse_jmbg;
use std::process;
use utils::validate_jmbg;

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
            println!("Region: {}", person.region);
            println!("Unique number: {}", person.unique_number);
            println!("Control digit: {}", person.control_digit);
            println!("Gender: {}", person.gender);
        }
        Err(e) => {
            println!("Error parsing JMBG: {}", e);
        }
    }
}
