#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
}

#[derive(Debug)]
enum PassportError {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId
}

fn validate_year_option(year: &Option<String>, min: u32, max: u32) -> bool {
    match year {
        None => false,
        Some(year) => {
            let year_int: u32 = year.parse().unwrap();
            year_int >= min && year_int <= max
        }
    }
}

impl Passport {
    fn consume_token(&mut self, kvpair: &str) {
        let mut tokens = kvpair.split(':');
        let key = tokens.next().unwrap();
        let value = String::from(tokens.next().unwrap());

        match key {
            "byr" => self.birth_year = Some(value),
            "iyr" => self.issue_year = Some(value),
            "eyr" => self.expiration_year = Some(value),
            "hgt" => self.height = Some(value),
            "hcl" => self.hair_color = Some(value),
            "ecl" => self.eye_color = Some(value),
            "pid" => self.passport_id = Some(value),
            "cid" => self.country_id = Some(value),
            _ => panic!()
        }
    }

    fn valid_presence(&self) -> bool {
        self.birth_year.is_some() && self.issue_year.is_some() &&
        self.expiration_year.is_some() && self.height.is_some() &&
        self.hair_color.is_some() && self.eye_color.is_some() &&
        self.passport_id.is_some()
    }

    fn errors_part_two(&self) -> Option<PassportError> {
        if !self.valid_birth_year() { return Some(PassportError::BirthYear) };
        if !self.valid_issue_year() { return Some(PassportError::IssueYear) };
        if !self.valid_expiration_year() { return Some(PassportError::ExpirationYear) };
        if !self.valid_height() { return Some(PassportError::Height) };
        if !self.valid_hair_color() { return Some(PassportError::HairColor) };
        if !self.valid_eye_color() { return Some(PassportError::EyeColor) };
        if !self.valid_passport_id() { return Some(PassportError::PassportId) };

        None
    }

    fn valid_birth_year(&self) -> bool {
        validate_year_option(&self.birth_year, 1920, 2002)
    }

    fn valid_issue_year(&self) -> bool {
        validate_year_option(&self.issue_year, 2010, 2020)
    }

    fn valid_expiration_year(&self) -> bool {
        validate_year_option(&self.expiration_year, 2020, 2030)
    }

    fn valid_height(&self) -> bool {
        match &self.height {
            None => false,
            Some(value) => {
                if value.len() <= 2 { return false; }

                let (height, height_unit) = value.split_at(value.len() - 2);
                let height_int: usize = height.parse().unwrap();

                match height_unit {
                    "in" => height_int >= 59 && height_int <= 76,
                    "cm" => height_int >= 150 && height_int <= 193,
                    _ => false
                }
            }
        }
    }

    fn valid_hair_color(&self) -> bool {
        let color = match &self.hair_color {
            None => return false,
            Some(c) => c
        };

        let (hash, code) = color.split_at(1);

        if hash != "#" { return false; }

        code.len() == 6 && code.chars().all(|c| c.is_ascii_hexdigit())
    }

    fn valid_eye_color(&self) -> bool {
        let color = match &self.eye_color {
            None => return false,
            Some(c) => c
        };
        
        match color.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    }

    fn valid_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(pid) => pid.len() == 9 && pid.chars().all(|c| c.is_numeric()),
            None => false
        }
    }
}

fn read_input(path: &str) -> Vec<Passport> {
    let file = std::fs::read_to_string(path).expect("could not open file");

    file.lines().fold(vec![Passport { ..Default::default() }], |mut passports, line| {
        if line.is_empty() {
            passports.push(Passport { ..Default::default() });
        } else {
            line.split(" ").for_each(|kvpair| passports.last_mut().unwrap().consume_token(kvpair));
        }

        passports
    })
}

fn main() {
    let passports = read_input("input.txt");
    
    // Part 1
    let valid = passports.iter().filter(|passport| passport.valid_presence()).count();
    println!("Valid passports for part 1: {}", valid);

    // Part 2
    let valid = passports.iter().filter(|passport| passport.errors_part_two().is_none()).count();
    println!("Valid passports for part 2: {}", valid);
}

#[test]
fn test_part_one_validation() {
    let passports = read_input("example.txt");
    
    let valid = passports.iter().filter(|passport| passport.valid_presence()).count();
    assert_eq!(valid, 2);
}

#[test]
fn test_part_two_individual() {
    // Birth Year
    let passport = Passport { birth_year: Some(String::from("2002")), ..Default::default() };
    assert_eq!(passport.valid_birth_year(), true);

    let passport = Passport { birth_year: Some(String::from("2003")), ..Default::default() };
    assert_eq!(passport.valid_birth_year(), false);

    // Height
    let passport = Passport { height: Some(String::from("60in")), ..Default::default() };
    assert_eq!(passport.valid_height(), true);

    let passport = Passport { height: Some(String::from("190cm")), ..Default::default() };
    assert_eq!(passport.valid_height(), true);

    let passport = Passport { height: Some(String::from("190in")), ..Default::default() };
    assert_eq!(passport.valid_height(), false);

    let passport = Passport { height: Some(String::from("190")), ..Default::default() };
    assert_eq!(passport.valid_height(), false);

    // Hair Color
    let passport = Passport { hair_color: Some(String::from("#123abc")), ..Default::default() };
    assert_eq!(passport.valid_hair_color(), true);

    let passport = Passport { hair_color: Some(String::from("#123abz")), ..Default::default() };
    assert_eq!(passport.valid_hair_color(), false);

    let passport = Passport { hair_color: Some(String::from("123abc")), ..Default::default() };
    assert_eq!(passport.valid_hair_color(), false);

    // Eye Color
    let passport = Passport { eye_color: Some(String::from("brn")), ..Default::default() };
    assert_eq!(passport.valid_eye_color(), true);

    let passport = Passport { eye_color: Some(String::from("wat")), ..Default::default() };
    assert_eq!(passport.valid_eye_color(), false);

    // Passport ID
    let passport = Passport { passport_id: Some(String::from("000000001")), ..Default::default() };
    assert_eq!(passport.valid_passport_id(), true);

    let passport = Passport { passport_id: Some(String::from("0123456789")), ..Default::default() };
    assert_eq!(passport.valid_passport_id(), false);
}

#[test]
fn test_part_two_full() {
    let valid_passports = read_input("example_valid.txt");
    valid_passports.iter().for_each(|passport| assert!(passport.errors_part_two().is_none()));

    let invalid_passports = read_input("example_invalid.txt");
    invalid_passports.iter().for_each(|passport| assert!(passport.errors_part_two().is_some()));
}
