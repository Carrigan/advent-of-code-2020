fn parse_input(path: &str) -> (u32, Vec<u32>) {
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

fn soonest_arriving_after(timestamp: u32, buses: &Vec<u32>) -> (u32, u32) {
    buses
        .iter()
        .map(|bus| (*bus, ((timestamp / *bus) + 1) * *bus))
        .min_by_key(|(_bus, first_arriving)| *first_arriving)
        .unwrap()
}

fn main() {
    let (timestamp, buses) = parse_input("input.txt");
    let (bus, arrival) = soonest_arriving_after(timestamp, &buses);

    println!("Part one: bus {} arrives {} minutes late", bus, arrival - timestamp);
}

#[test]
fn test_part_one() {
    let (timestamp, buses) = parse_input("example.txt");
    let (bus, arrival) = soonest_arriving_after(timestamp, &buses);
    assert_eq!(bus, 59);
    assert_eq!(arrival, 944);
}