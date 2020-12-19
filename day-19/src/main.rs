use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    Concrete(char),
    SingleReference(Vec<usize>),
    DoubleReference(Vec<usize>, Vec<usize>),
    Special(String)
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

    let first_match = pipe_split.next().unwrap();
    match pipe_split.next() {
        Some(second_reference) => Rule::DoubleReference(first_match, second_reference),
        None => Rule::SingleReference(first_match)
    }
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

fn join_refs(rules: &HashMap<usize, Rule>, refs: &Vec<usize>) -> String {
    refs.iter().map(|rule_ref| rule_to_string(rules, *rule_ref)).collect::<Vec<String>>().join("")
}

fn rule_to_string(rules: &HashMap<usize, Rule>, index: usize) -> String {
    match &rules[&index] {
        Rule::Concrete(n) => n.to_string(),
        Rule::SingleReference(refs) => join_refs(rules, &refs),
        Rule::DoubleReference(first_refs, second_refs) => {
            let first_ref_string = join_refs(rules, &first_refs);
            let second_ref_string = join_refs(rules, &second_refs);

            format!("({}|{})", first_ref_string, second_ref_string)
        },
        Rule::Special(s) => s.clone()
    }
}

fn modify_rules(rules: &mut HashMap<usize, Rule>) {
    let r42 = rule_to_string(rules, 42);
    let r31 = rule_to_string(rules, 31);

    let modified_rule_8 = Rule::Special(
        format!("({r42})+", r42=r42)
    );

    let modified_rule_11 = Rule::Special(
        format!("({r42}({r42}({r42}({r42}{r31})?{r31})?{r31})?{r31})", r42=r42, r31=r31)
    );

    rules.insert(8, modified_rule_8);
    rules.insert(11, modified_rule_11);
}

fn valid_message_count(path: &str, modify: bool) -> usize {
    let (mut rules, messages) = read_input(path);
    if modify { modify_rules(&mut rules); }
    let rule_regex = rule_to_string(&rules, 0);
    let r = Regex::new(&format!("^{}$", rule_regex)).unwrap();

    messages.iter().filter(|m| r.is_match(m)).count()
}

fn main() {
    println!("Part one count: {}", valid_message_count("input.txt", false));
    println!("Part two count: {}", valid_message_count("input.txt", true));
}

#[test]
fn test_part_one() {
    assert_eq!(2, valid_message_count("example.txt", false));
}

#[test]
fn test_part_two() {
    assert_eq!(3, valid_message_count("example2.txt", false));
    assert_eq!(12, valid_message_count("example2.txt", true));
}
