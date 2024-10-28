# JMBG Parser

A Rust-based tool for parsing and validating **Jedinstveni Matični Broj Građana (JMBG)**, commonly used in several Balkan countries as a unique identification number. The JMBG Parser extracts valuable information such as date of birth, region, and gender, while also verifying the validity of the number using its control digit.

## Features

- **Extracts Information**: Retrieves date of birth, region, and gender from a JMBG.
- **Validates JMBG**: Ensures the JMBG is correct by checking its length and control digit.
- **Enum-based Regions and Genders**: Uses Rust enums for better type safety.

## Usage

### Prerequisites

- **Rust**: Install Rust and Cargo by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Clone and Build

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/jmbg_parser.git
   cd jmbg_parser
   ```
2. Build the project:

  ```bash
  cargo build
  ```
3. Run the project:
  
  ```bash
  cargo run -- --jmbg 2708005798914
  ```
Example Output
For a valid JMBG like 2708005798914:

```json
{
  "date_of_birth": "2005-08-27",
  "region": "Uzice",
  "unique_number": 891,
  "gender": "Female",
  "jmbg": "2708005798914"
}
```

