use std::collections::HashMap;

#[derive(Debug)]
struct RuleReference {
    matches: Vec<usize>,
    alt_matches: Option<Vec<usize>>
}

#[derive(Debug)]
enum Rule {
    Concrete(char),
    Reference(RuleReference)
}

fn parse_reference(post_semicolon: &str) -> Rule {
    let mut pipe_split = post_semicolon
        .split(|c| c == '|')
        .map(|sequence| {
            sequence
                .split(|c| c == ' ')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        });

    let rule_ref = Rule::Reference(RuleReference {
        matches: pipe_split.next().unwrap(),
        alt_matches: pipe_split.next()
    });

    rule_ref
}

fn read_input(path: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut parsing_rules = true;
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        match parsing_rules {
            true => {
                let mut split_iter = line.split(|c| c == ':');
                let rule_number_str = split_iter.next().unwrap();
                let rest = split_iter.next().unwrap();

                let rule = match rest.contains(|c| c == '"') {
                    true => Rule::Concrete(rest.chars().nth(2).unwrap()),
                    false => parse_reference(rest)
                };

                rules.insert(rule_number_str.parse().unwrap(), rule);
            },
            false => messages.push(String::from(line))
        }
    }

    (rules, messages)
}

fn permute_lists(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    let mut permutes = Vec::new();

    for s1 in v1.iter() {
        for s2 in v2.iter() {
            permutes.push(s1.clone() + &s2.clone());
        }
    }

    permutes
}

fn combine(indeces: &[usize], rules: &HashMap<usize, Rule>) -> Vec<String> {
    let mut rule_iter = indeces.iter();
    let first_index = rule_iter.next().unwrap();
    let starting_strings = expand(rules, *first_index);

    rule_iter.fold(starting_strings, |current, next| {
        permute_lists(current, expand(rules, *next))
    })
}

fn expand(rules: &HashMap<usize, Rule>, index: usize) -> Vec<String> {
    match rules.get(&index).unwrap() {
        Rule::Concrete(c) => vec![c.to_string()],
        Rule::Reference(rule_ref) => {
            let mut out = combine(&rule_ref.matches, rules);

            if let Some(other_matches) = &rule_ref.alt_matches {
                let mut combined = combine(&other_matches, rules);
                out.append(&mut combined);
            }

            out
        }
    }
}

fn main() {
    // Part One
    let (rules, messages) = read_input("input.txt");
    let options = expand(&rules, 0);
    println!("Part one options expanded: {} found", options.len());

    let valid = messages.iter().filter(|m| options.contains(m)).count();
    println!("Valid for part one: {}", valid);
}

#[test]
fn test_part_one() {
    let (rules, messages) = read_input("example.txt");
    let options = expand(&rules, 0);
    let valid = messages.iter().filter(|m| options.contains(m)).count();

    assert_eq!(valid, 2);
}
