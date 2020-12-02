use std::convert::TryFrom;
use std::fs;

struct Password<'a> {
    content: &'a str
}

impl <'a> From<&'a str> for Password<'a> {
    fn from(line: &'a str) -> Self {
        Password { content: line.trim() }
    }
}

impl <'a> Password<'a> {
    fn valid_for(&self, policy: &PasswordPolicy) -> bool {
        let mut character_count = 0;
        
        for character in self.content.chars() {
            if character == policy.character { character_count += 1 };
        }

        character_count >= policy.min && character_count <= policy.max
    }
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char
}

#[derive(Debug)]
enum PasswordPolicyError {
    HyphenNotFound,
    SpaceNotFound,
    MinParseError,
    MaxParseError,
    CharacterError
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = PasswordPolicyError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let hyphen_position = line.find('-')
            .ok_or(PasswordPolicyError::HyphenNotFound)?;
        let space_position = line.find(' ')
            .ok_or(PasswordPolicyError::SpaceNotFound)?;

        let min = line[0..hyphen_position].parse::<usize>()
            .or(Err(PasswordPolicyError::MinParseError))?;

        let max = line[hyphen_position + 1..space_position].parse::<usize>()
            .or(Err(PasswordPolicyError::MaxParseError))?;

        let character = line.chars().nth(space_position + 1)
            .ok_or(PasswordPolicyError::CharacterError)?;

        Ok(PasswordPolicy{ min, max, character })
    }
}

fn parse_line<'a>(line: &'a str) -> (PasswordPolicy, Password<'a>) {
    let colon_position = line.find(':').unwrap();
    let policy_str = &line[0..colon_position];
    let password_str = &line[colon_position + 1..];

    (PasswordPolicy::try_from(policy_str).unwrap(), Password::from(password_str))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("file not found");

    let compliant_passwords = input
        .lines()
        .into_iter()
        .map( |line| parse_line(line))
        .fold(0, |total, (policy, password)| 
            if password.valid_for(&policy) { total + 1 } else { total }
        );

    println!("Compliant passwords: {}", compliant_passwords);
}

#[test]
fn test_conversion() {
    let policy_string = "1-3 a";
    let policy = PasswordPolicy::try_from(policy_string).unwrap();

    assert_eq!(policy.min, 1);
    assert_eq!(policy.max, 3);
    assert_eq!(policy.character, 'a');
}

#[test]
fn test_full_line() {
    let (policy, password) = parse_line("1-3 a: abcde");

    assert_eq!(policy.min, 1);
    assert_eq!(policy.max, 3);
    assert_eq!(policy.character, 'a');
    assert_eq!(password.content, "abcde");
}

#[test]
fn test_examples() {
    let (policy1, password1) = parse_line("1-3 a: abcde");
    assert!(password1.valid_for(&policy1));

    let (policy2, password2) = parse_line("1-3 b: cdefg");
    assert_eq!(password2.valid_for(&policy2), false);
 
    let (policy3, password3) = parse_line("2-9 c: ccccccccc");
    assert!(password3.valid_for(&policy3));
}
