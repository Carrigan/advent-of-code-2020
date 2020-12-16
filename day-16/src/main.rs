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

fn main() {
    // Part one
    let (validations, _, nearby) = parse_input("input.txt");
    let invalid_sum = find_invalid_fields(&nearby, &validations).iter().sum::<usize>();
    println!("Part one: {}", invalid_sum);
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
