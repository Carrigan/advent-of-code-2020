use std::collections::HashMap;

use regex::Regex;
struct TicketValidation {
    name: String,
    lower_one: usize,
    upper_one: usize,
    lower_two: usize,
    upper_two: usize
}

impl From<&str> for TicketValidation {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"(\w+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let matches = re.captures(line).unwrap();

        TicketValidation {
            name: String::from(&matches[1]),
            lower_one: matches[2].parse().unwrap(),
            upper_one: matches[3].parse().unwrap(),
            lower_two: matches[4].parse().unwrap(),
            upper_two: matches[5].parse().unwrap()
        }
    }
}

impl TicketValidation {
    fn validate(&self, other: usize) -> bool {
        (other >= self.lower_one && other <= self.upper_one) ||
        (other >= self.lower_two && other <= self.upper_two)
    }
}

fn parse_input(path: &str) -> (Vec<TicketValidation>, Vec<usize>, Vec<Vec<usize>>) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut lines = file.lines();

    let mut ticket_validations = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() { break; }
        ticket_validations.push(TicketValidation::from(line));
    }

    lines.next();
    let my_ticket = lines.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();

    lines.next();
    lines.next();
    let mut nearby_tickets = Vec::new();
    while let Some(line) = lines.next() {
        nearby_tickets.push(
            line.split(",").map(|n| n.parse().unwrap()).collect()
        );
    }

    (ticket_validations, my_ticket, nearby_tickets)
}

fn find_invalid_fields(nearby: &[Vec<usize>], validations: &[TicketValidation]) -> Vec<usize> {
    nearby.iter()
        .flatten()
        .filter(|&&n| validations.iter().all(|val| !val.validate(n)))
        .map(|n| *n)
        .collect()
}

fn column_possibilities(nearby: &[Vec<usize>], validations: &[TicketValidation]) -> Vec<Vec<usize>> {
    let valid_tickets: Vec<&Vec<usize>> = nearby
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|&n| {
                validations.iter().any(|val| val.validate(n))
            })
        })
        .collect();

    let col_count = valid_tickets[0].len();

    // Return a list of length col_count where each is a [rule_index]
    (0..col_count)
        .map(|index| {
            let rule_indeces = validations
                .iter()
                .enumerate()
                .filter(|(_, validation)| {
                    valid_tickets.iter().map(|ticket| ticket[index]).all(|n| validation.validate(n))
                })
                .map(|(val_idx, _)| val_idx)
                .collect();

            rule_indeces
        })
        .collect()
}

fn only<F: Iterator<Item=usize>>(iter: &mut F) -> Option<usize> {
    let first_number = match iter.next() {
        Some(n) => n,
        None => return None
    };

    match iter.next() {
        Some(_) => None,
        None => Some(first_number)
    }
}

fn solve_possibilities(columns: &[Vec<usize>]) -> HashMap<usize, usize> {
    // `possibilities` is a list of lists where each inner list is called a
    // possibility_list and each entry in that is called a possibility
    // solved contains a map of rule_index: column_index
    let mut solved: HashMap<usize, usize> = HashMap::new();

    loop {
        // Each iteration of this we are looking for possibility_lists that can
        // only be one value.
        let solved_possibility_lists: Vec<(usize, usize)> = columns
            .iter()
            .enumerate()
            .filter(|(column_index, _)| solved.values().find(|&v| v == column_index).is_none())
            .filter_map(|(column_index, rule_list)| {
                let mut unclaimed_rule_indeces = rule_list
                    .iter()
                    .filter(|possibility| !solved.contains_key(possibility))
                    .map(|&n| n);

                only(&mut unclaimed_rule_indeces).map(|rule_index| (rule_index, column_index))
            })
            .collect();

        if solved_possibility_lists.len() == 0 { break; }

        solved_possibility_lists.iter().for_each(|(rule_index, column_index)|
            { solved.insert(*rule_index, *column_index); });
    }

    solved
}

fn main() {
    // Part one
    let (validations, my_ticket, nearby) = parse_input("input.txt");
    let invalid_sum = find_invalid_fields(&nearby, &validations).iter().sum::<usize>();
    println!("Part one: {}", invalid_sum);

    // positions 0-5 are the departure fields
    let columns = column_possibilities(&nearby, &validations);
    let mappings = solve_possibilities(&columns);
    println!("{:?}", mappings);
    let product = mappings.iter()
        .filter(|(&rule_index, _)| rule_index < 6)
        .map(|(_, &col_index)| my_ticket.get(col_index).unwrap())
        .product::<usize>();

    println!("Part two: {}", product);
}

#[test]
fn test_parsers() {
    let val = TicketValidation::from("class: 1-3 or 5-7");
    assert_eq!(val.name, String::from("class"));
    assert_eq!(val.lower_one, 1);
    assert_eq!(val.upper_one, 3);
    assert_eq!(val.lower_two, 5);
    assert_eq!(val.upper_two, 7);
    assert_eq!(val.validate(2), true);
    assert_eq!(val.validate(8), false);
}

#[test]
fn test_part_one() {
    let (validations, _, nearby) = parse_input("example1.txt");
    assert_eq!(find_invalid_fields(&nearby, &validations).iter().sum::<usize>(), 71);
}

#[test]
fn test_part_two() {
    let (validations, _, nearby) = parse_input("example1.txt");
    let columns = column_possibilities(&nearby, &validations);
    let solved = solve_possibilities(&columns);

    println!("{:?}", solved);
    assert_eq!(solved[&0], 1);
    assert_eq!(solved[&1], 0);
    assert_eq!(solved[&2], 2);
}