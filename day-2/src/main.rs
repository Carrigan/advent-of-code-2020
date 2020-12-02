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
    fn valid_for_count(&self, policy: &PasswordPolicy) -> bool {
        let mut character_count = 0;
        
        for character in self.content.chars() {
            if character == policy.character { character_count += 1 };
        }

        character_count >= policy.first && character_count <= policy.second
    }

    fn valid_for_xor(&self, policy: &PasswordPolicy) -> bool {
        let first_is_match = match self.content.chars().nth(policy.first - 1) {
            Some(character) => character == policy.character,
            None => false
        };

        let second_is_match = match self.content.chars().nth(policy.second - 1) {
            Some(character) => character == policy.character,
            None => false
        };

        (first_is_match || second_is_match) && !(first_is_match && second_is_match)
    }
}

struct PasswordPolicy {
    first: usize,
    second: usize,
    character: char
}

#[derive(Debug)]
enum PasswordPolicyError {
    HyphenNotFound,
    SpaceNotFound,
    FirstParseError,
    SecondParseError,
    CharacterError
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = PasswordPolicyError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let hyphen_position = line.find('-')
            .ok_or(PasswordPolicyError::HyphenNotFound)?;
        let space_position = line.find(' ')
            .ok_or(PasswordPolicyError::SpaceNotFound)?;

        let first = line[0..hyphen_position].parse::<usize>()
            .or(Err(PasswordPolicyError::FirstParseError))?;

        let second = line[hyphen_position + 1..space_position].parse::<usize>()
            .or(Err(PasswordPolicyError::SecondParseError))?;

        let character = line.chars().nth(space_position + 1)
            .ok_or(PasswordPolicyError::CharacterError)?;

        Ok(PasswordPolicy{ first, second, character })
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

    // Part 1
    let compliant_passwords = input
        .lines()
        .into_iter()
        .map( |line| parse_line(line))
        .fold(0, |total, (policy, password)| 
            if password.valid_for_count(&policy) { total + 1 } else { total }
        );

    println!("Compliant passwords by count: {}", compliant_passwords);

    // Part 2
    let compliant_passwords = input
        .lines()
        .into_iter()
        .map( |line| parse_line(line))
        .fold(0, |total, (policy, password)| 
            if password.valid_for_xor(&policy) { total + 1 } else { total }
        );

    println!("Compliant passwords by position: {}", compliant_passwords);
}

#[test]
fn test_conversion() {
    let policy_string = "1-3 a";
    let policy = PasswordPolicy::try_from(policy_string).unwrap();

    assert_eq!(policy.first, 1);
    assert_eq!(policy.second, 3);
    assert_eq!(policy.character, 'a');
}

#[test]
fn test_full_line() {
    let (policy, password) = parse_line("1-3 a: abcde");

    assert_eq!(policy.first, 1);
    assert_eq!(policy.second, 3);
    assert_eq!(policy.character, 'a');
    assert_eq!(password.content, "abcde");
}

#[test]
fn test_examples() {
    let (policy1, password1) = parse_line("1-3 a: abcde");
    assert!(password1.valid_for_count(&policy1));
    assert!(password1.valid_for_xor(&policy1));

    let (policy2, password2) = parse_line("1-3 b: cdefg");
    assert_eq!(password2.valid_for_count(&policy2), false);
    assert_eq!(password2.valid_for_xor(&policy2), false);
 
    let (policy3, password3) = parse_line("2-9 c: ccccccccc");
    assert!(password3.valid_for_count(&policy3));
    assert_eq!(password3.valid_for_xor(&policy3), false);
}
