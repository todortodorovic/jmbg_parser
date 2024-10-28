use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u32)] // Ensures each variant is represented by a u32
pub enum Gender {
    Male = 0,
    Female = 1,
}

impl Gender {
    /// Kreiranje `Gender` iz `unique_number` (BBB)
    pub fn from_unique_number(unique_number: u32) -> Gender {
        if (0..=499).contains(&unique_number) {
            Gender::Male
        } else {
            Gender::Female
        }
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u32)] // Ensures each variant is represented by a u32
pub enum Region {
    StranciUBiH = 1,
    StranciUCrnojGori = 2,
    StranciUHrvatskoj = 3,
    StranciUSevernojMakedoniji = 4,
    StranciUSloveniji = 5,
    Vojvodina = 8,
    Kosovo = 9,
    Bihac = 10,
    Doboj = 11,
    Gorazde = 12,
    Livno = 13,
    Mostar = 14,
    Prijedor = 15,
    Sarajevo = 16,
    Tuzla = 17,
    Zenica = 18,
    Podgorica = 20,
    Bar = 21,
    Budva = 22,
    HercegNovi = 23,
    Cetinje = 24,
    Niksic = 25,
    Berane = 26,
    BijeloPolje = 27,
    Pljevlja = 28,
    Osijek = 30,
    Varazdin = 31,
    Zagreb = 32,
    Karlovac = 33,
    Lika = 34,
    Sisak = 35,
    Dalmacija = 36,
    Bitolj = 40,
    Kumanovo = 41,
    Ohrid = 42,
    Prilep = 43,
    Skoplje = 44,
    Strumica = 45,
    Tetovo = 46,
    Veles = 47,
    Stip = 48,
    Slovenija = 50,
    Beograd = 71,
    Kragujevac = 72,
    Nis = 73,
    Leskovac = 74,
    Zajecar = 75,
    Smederevo = 76,
    Macva = 77,
    Cacak = 78,
    Uzice = 79,
    NoviSad = 80,
    Sombor = 81,
    Subotica = 82,
    Vrbas = 83,
    Kikinda = 84,
    Zrenjanin = 85,
    Pancevo = 86,
    Vrsac = 87,
    SremskaMitrovica = 88,
    Pristina = 90,
    KosovskaMitrovica = 91,
    Pec = 92,
    Djakovica = 93,
    Prizren = 94,
    KosovskaKamenica = 95,
}
