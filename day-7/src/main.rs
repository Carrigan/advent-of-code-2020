struct BagRule {
    container_index: usize,
    contained_index: usize,
    count: u32
}

fn parse_container_string(string: &str) -> String {
    String::from(&string[0..string.trim().find(" bag").unwrap()])
}

fn parse_contained_string(string: &str) -> Vec<(u32, String)> {
    if string.contains("no other bags") {
        return Vec::new();
    }

    string
        .split(",")
        .map(|entry| entry.trim())
        .map(|entry| {
            let first_space = entry.find(" ").unwrap();
            let count = entry[0..first_space].parse::<u32>().unwrap();
            let container = parse_container_string(&entry[(first_space + 1)..]);

            (count, container)
        })
        .collect()
}

fn parse_line(line: &str) -> (String, Vec<(u32, String)>) {
    let mut parts = line.split("contain");

    // Parse the container
    let container_raw = parts.next().unwrap();
    let container_string = parse_container_string(container_raw);

    // Parse the contained strings
    let contained_str = parts.next().unwrap();
    let contained_pairs = parse_contained_string(contained_str);


    (container_string, contained_pairs)
}

fn add_rule(bags: &mut Vec<String>, bag_rules: &mut Vec<BagRule>, line: &str) {
    let (container, contained) = parse_line(line);

    // Make sure all bags are in the arena
    if !bags.contains(&container) { bags.push(container.clone()); }
    contained.iter().for_each(|(_, con)| {
        if !bags.contains(&con) { bags.push(con.clone()); }
    });

    // Make connections
    contained.iter().for_each(|(count, con)| {
        bag_rules.push(BagRule { 
            container_index: bags.iter().position(|search_container| search_container == &container).unwrap(),
            contained_index: bags.iter().position(|search_container| search_container == con).unwrap(),
            count: *count
        });
    });
}

fn traverse_up(found: &mut Vec<usize>, current: usize, rules: &Vec<BagRule>) {
    if found.contains(&current) { return; }
    
    found.push(current);
    
    let containers_to_search: Vec<usize> = rules
        .iter()
        .filter(|rule| rule.contained_index == current)
        .map(|rule| rule.container_index)
        .collect();
    
    containers_to_search.iter().for_each(|container_index| traverse_up(found, *container_index, rules));
}

fn traverse_up_from_point(bags: &Vec<String>, bag_rules: &Vec<BagRule>, starting_point: &str) -> usize {
    let starting_index = bags.iter().position(|bag| (*bag).as_str() == starting_point).unwrap();
    let mut found = Vec::new();

    // Fill up the traversal
    traverse_up(&mut found, starting_index, bag_rules);

    // Subtract one to remove the node itself
    found.len() - 1
}

fn self_and_bags_contained(rules: &Vec<BagRule>, index: usize) -> usize {
    1 + rules
        .iter()
        .filter(|rule| rule.container_index == index)
        .map(|rule| (rule.count as usize) * self_and_bags_contained(rules, rule.contained_index))
        .sum::<usize>()
}

fn traverse_bags_contained(rules: &Vec<BagRule>, index: usize) -> usize {
    self_and_bags_contained(rules, index) - 1
}

fn main() {
    // Fill our rules up
    let mut bags = Vec::new();
    let mut bag_rules = Vec::new();

    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|l| add_rule(&mut bags, &mut bag_rules, l));

    // Part one
    let graph_count = traverse_up_from_point(&bags, &bag_rules, "shiny gold");
    println!("Part one: {}", graph_count);

    // Part two
    let starting_index = bags.iter().position(|bag| (*bag).as_str() == "shiny gold").unwrap();
    let bags_contained = traverse_bags_contained(&bag_rules, starting_index);
    println!("Part two: {}", bags_contained);
}

#[test]
fn test_parsers() {
    let container_string = "light red bags ";
    assert_eq!(&parse_container_string(container_string), "light red");

    let zero_contained_string = " no other bags.";
    assert_eq!(parse_contained_string(zero_contained_string), Vec::new());

    let two_contained_string = " 2 shiny gold bags, 9 faded blue bags";
    let two_contained_vec = parse_contained_string(two_contained_string);
    assert_eq!(two_contained_vec[0], (2, String::from("shiny gold")));
    assert_eq!(two_contained_vec[1], (9, String::from("faded blue")));

    let one_contained_string = " 1 shiny gold bag.";
    let one_contained_vec = parse_contained_string(one_contained_string);
    assert_eq!(one_contained_vec[0], (1, String::from("shiny gold")));
}

#[test]
fn test_example_part_1() {
    let mut bags = Vec::new();
    let mut bag_rules = Vec::new();
    
    std::fs::read_to_string("example.txt")
        .unwrap()
        .lines()
        .for_each(|l| add_rule(&mut bags, &mut bag_rules, l));

    assert_eq!(bag_rules.len(), 13);

    let graph_count = traverse_up_from_point(&bags, &bag_rules, "shiny gold");
    assert_eq!(graph_count, 4);
}

#[test]
fn test_example_part_2() {
    let mut bags = Vec::new();
    let mut bag_rules = Vec::new();
    
    std::fs::read_to_string("example2.txt")
        .unwrap()
        .lines()
        .for_each(|l| add_rule(&mut bags, &mut bag_rules, l));

    let starting_index = bags.iter().position(|bag| (*bag).as_str() == "shiny gold").unwrap();
    let contains_bags = traverse_bags_contained(&bag_rules, starting_index);
    assert_eq!(contains_bags, 126);
}