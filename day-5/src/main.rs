
struct Seat {
    id: u32
}

impl From<&str> for Seat {
    fn from(input: &str) -> Self {
        let id = input
            .chars()
            .enumerate()
            .fold(0, |total, (index, ch)| {
                let binary_value = match ch {
                    'B' | 'R' => 1,
                    _ => 0
                };

                total + (binary_value << (9 - index))
            });

        Seat { id }
    }
}

fn main() {
    // Input
    let input = std::fs::read_to_string("input.txt")
        .expect("could not open input.txt");

    let seat_ids = input
        .lines()
        .map(|l| Seat::from(l).id)
        .collect::<Vec<u32>>();

    // Part 1
    let max = *seat_ids.iter().max().unwrap();
    println!("Part 1: {}", max);

    // Part 2
    let min = *seat_ids.iter().min().unwrap();
    let my_seat = (min..max)
        .find(|x| !seat_ids.contains(x))
        .unwrap();

    println!("Part 2: {}", my_seat);
}

#[test]
fn test_example() {
    let seat = Seat::from("FBFBBFFRLR");
    assert_eq!(seat.id, 357);

    let seat = Seat::from("BFFFBBFRRR");
    assert_eq!(seat.id, 567);

    let seat = Seat::from("FFFBBBFRRR");
    assert_eq!(seat.id, 119);

    let seat = Seat::from("BBFFBBFRLL");
    assert_eq!(seat.id, 820);
}
