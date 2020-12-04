#[derive(Default)]
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

    fn valid(&self) -> bool {
        self.birth_year.is_some() && self.issue_year.is_some() &&
        self.expiration_year.is_some() && self.height.is_some() &&
        self.hair_color.is_some() && self.eye_color.is_some() &&
        self.passport_id.is_some()
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
    let valid = passports.iter().filter(|passport| passport.valid()).count();
    println!("Valid passports for part 1: {}", valid);
}

#[test]
fn test_example() {
    let passports = read_input("example.txt");
    
    let valid = passports.iter().filter(|passport| passport.valid()).count();

    assert_eq!(valid, 2);
}