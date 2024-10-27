use crate::errors::JmbgError;

/// Function to determine gender based on the unique number (BBB)
pub fn determine_gender(unique_number: u32) -> String {
    if (0..=499).contains(&unique_number) {
        "Male".to_string()
    } else {
        "Female".to_string()
    }
}

pub fn map_region(region_code: u32) -> &'static str {
    match region_code {
        1 => "Stranci u BiH",
        2 => "Stranci u Crnoj Gori",
        3 => "Stranci u Hrvatskoj",
        4 => "Stranci u Severnoj Makedoniji",
        5 => "Stranci u Sloveniji",
        7 => "Beograd",
        8 => "Vojvodina",
        9 => "Kosovo",
        10 => "Bihać, Bosna i Hercegovina",
        11 => "Doboj, Bosna i Hercegovina",
        12 => "Goražde, Bosna i Hercegovina",
        13 => "Livno, Bosna i Hercegovina",
        14 => "Mostar, Bosna i Hercegovina",
        15 => "Prijedor, Bosna i Hercegovina",
        16 => "Sarajevo, Bosna i Hercegovina",
        17 => "Tuzla, Bosna i Hercegovina",
        18 => "Zenica, Bosna i Hercegovina",
        20 => "Podgorica, Crna Gora",
        21 => "Bar, Crna Gora",
        22 => "Budva, Crna Gora",
        23 => "Herceg Novi, Crna Gora",
        24 => "Cetinje, Crna Gora",
        25 => "Nikšić, Crna Gora",
        26 => "Berane, Crna Gora",
        27 => "Bijelo Polje, Crna Gora",
        28 => "Pljevlja, Crna Gora",
        30 => "Osijek, Hrvatska",
        31 => "Varaždin, Hrvatska",
        32 => "Zagreb, Hrvatska",
        33 => "Karlovac, Hrvatska",
        34 => "Lika, Hrvatska",
        35 => "Sisak, Hrvatska",
        36 => "Dalmacija, Hrvatska",
        40 => "Bitolj, Severna Makedonija",
        41 => "Kumanovo, Severna Makedonija",
        42 => "Ohrid, Severna Makedonija",
        43 => "Prilep, Severna Makedonija",
        44 => "Skoplje, Severna Makedonija",
        45 => "Strumica, Severna Makedonija",
        46 => "Tetovo, Severna Makedonija",
        47 => "Veles, Severna Makedonija",
        48 => "Štip, Severna Makedonija",
        50 => "Slovenija",
        70 => "Beograd",
        71 => "Beograd",
        72 => "Kragujevac",
        73 => "Niš",
        74 => "Leskovac",
        75 => "Zaječar",
        76 => "Smederevo",
        77 => "Mačva",
        78 => "Čačak",
        79 => "Užice",
        80 => "Novi Sad",
        81 => "Sombor",
        82 => "Subotica",
        83 => "Vrbas",
        84 => "Kikinda",
        85 => "Zrenjanin",
        86 => "Pančevo",
        87 => "Vršac",
        88 => "Sremska Mitrovica",
        90 => "Priština, Kosovo",
        91 => "Kosovska Mitrovica, Kosovo",
        92 => "Peć, Kosovo",
        93 => "Đakovica, Kosovo",
        94 => "Prizren, Kosovo",
        95 => "Kosovska Kamenica, Kosovo",
        _ => "Nepoznat region",
    }
}

pub fn calculate_control_digit(jmbg: &str) -> Result<u32, JmbgError> {
    // Parse the first 12 digits of the JMBG into an array of numbers
    let digits: Vec<u32> = jmbg
        .chars()
        .take(12) // Take only the first 12 digits
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();

    if digits.len() != 12 {
        return Err(JmbgError::InvalidLength);
    }

    // Calculate the sum S using the formula:
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

    // Remainder when S is divided by 11
    let m = s % 11;

    // Calculate the control digit based on the value of m
    let k = match m {
        0 => 0,                                          // If the remainder is 0, the control digit is 0
        1 => return Err(JmbgError::InvalidControlDigit), // m=1 means the JMBG is invalid
        _ => 11 - m,                                     // For m > 1, the control digit is 11 - m
    };

    Ok(k)
}

/// Function for validating the entire JMBG
pub fn validate_jmbg(jmbg: &str) -> Result<(), JmbgError> {
    // Check if the JMBG is exactly 13 digits long
    if jmbg.len() != 13 {
        return Err(JmbgError::InvalidLength);
    }

    // Parse the last (13th) digit as the control digit
    let control_digit = jmbg
        .chars()
        .nth(12)
        .ok_or(JmbgError::InvalidControlDigit)?
        .to_digit(10)
        .ok_or(JmbgError::InvalidControlDigit)?;

    // Calculate the control digit based on the first 12 digits
    let calculated_digit = calculate_control_digit(jmbg)?;

    // Compare the control digit
    if control_digit == calculated_digit {
        Ok(())
    } else {
        Err(JmbgError::InvalidControlDigit)
    }
}
