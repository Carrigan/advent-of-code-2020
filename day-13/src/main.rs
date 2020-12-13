fn parse_input_part_one(path: &str) -> (u32, Vec<u32>) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse().unwrap())
        .collect();

    (timestamp, buses)
}

fn parse_input_part_two(path: &str) -> Vec<(u32, u32)> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    lines.next();

    lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(x, bus)| (x as u32, bus.parse().unwrap()))
        .collect()
}

fn soonest_arriving_after(timestamp: u32, buses: &Vec<u32>) -> (u32, u32) {
    buses
        .iter()
        .map(|bus| (*bus, ((timestamp / *bus) + 1) * *bus))
        .min_by_key(|(_bus, first_arriving)| *first_arriving)
        .unwrap()
}

fn find_magic_time(buses: &Vec<(u32, u32)>) -> u64 {
    let (step_offset, step) = buses.iter().max_by_key(|(_, s)| *s).unwrap();
    let mut t: u64 = *step as u64 - *step_offset as u64;

    loop {
        if buses.iter().all(|(offset, bus)| (t + *offset as u64) % *bus as u64 == 0) {
            return t;
        }

        t += *step as u64;
    }
}

fn main() {
    // Part one
    let (timestamp, buses) = parse_input_part_one("input.txt");
    let (bus, arrival) = soonest_arriving_after(timestamp, &buses);
    println!("Part one: bus {} arrives {} minutes late", bus, arrival - timestamp);

    // Part two
    let buses = parse_input_part_two("input.txt");
    println!("{:?}", find_magic_time(&buses))
}

#[test]
fn test_part_one() {
    let (timestamp, buses) = parse_input_part_one("example.txt");
    let (bus, arrival) = soonest_arriving_after(timestamp, &buses);
    assert_eq!(bus, 59);
    assert_eq!(arrival, 944);
}

#[test]
fn test_part_two() {
    let buses = parse_input_part_two("example.txt");
    assert_eq!(find_magic_time(&buses), 1068781);
}