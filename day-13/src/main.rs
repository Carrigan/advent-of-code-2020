#[derive(Debug, Copy, Clone)]
struct PeriodicWithOffsets {
    period: u64,
    offset_from_sync: u64,
    first_sync: u64
}

impl PeriodicWithOffsets {
    fn find_first_sync(&self, other: &PeriodicWithOffsets) -> u64 {
        let mut x1 = self.first_sync;
        let mut progress = 0;

        while x1 < self.offset_from_sync { x1 += self.period; }

        loop {
            x1 += self.period;
            progress += 1;

            if progress == 100_000 {
                print!(".");
                progress = 0;
            }

            let relative_x1 = x1 + other.offset_from_sync - self.offset_from_sync;

            if (relative_x1 >= other.first_sync) && (relative_x1 - other.first_sync) % other.period == 0 {
                println!("{} {} {} {} {}", self.offset_from_sync, self.period, other.offset_from_sync, other.period, x1);
                return x1 - self.offset_from_sync;
            }
        }
    }

    fn reduce(&self, other: &PeriodicWithOffsets) -> PeriodicWithOffsets {
        let lcm = num::integer::lcm(self.period, other.period);
        let offset = self.find_first_sync(other);

        PeriodicWithOffsets { period: lcm, offset_from_sync: 0, first_sync: offset }
    }
}

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

fn parse_input_part_two(path: &str) -> Vec<PeriodicWithOffsets> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    lines.next();

    lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(x, bus)| PeriodicWithOffsets { first_sync: 0, offset_from_sync: x as u64, period: bus.parse().unwrap() })
        .collect()
}

fn soonest_arriving_after(timestamp: u32, buses: &Vec<u32>) -> (u32, u32) {
    buses
        .iter()
        .map(|bus| (*bus, ((timestamp / *bus) + 1) * *bus))
        .min_by_key(|(_bus, first_arriving)| *first_arriving)
        .unwrap()
}

fn find_magic_time(periodics: &Vec<PeriodicWithOffsets>) -> u64 {
    let mut reduced = periodics.clone();

    while reduced.len() > 1 {
        println!("{:?}", reduced);

        reduced = reduced
            .chunks(2)
            .map(|periodic_slice|
                match periodic_slice {
                    &[a, b] => a.reduce(&b),
                    &[a] => a,
                    _ => panic!()
                })
            .collect();
    }

    reduced[0].first_sync
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

    let buses = parse_input_part_two("example2.txt");
    assert_eq!(find_magic_time(&buses), 3417);

    let buses = parse_input_part_two("example3.txt");
    assert_eq!(find_magic_time(&buses), 1202161486);

    let buses = parse_input_part_two("example4.txt");
    assert_eq!(find_magic_time(&buses), 1261476);
}