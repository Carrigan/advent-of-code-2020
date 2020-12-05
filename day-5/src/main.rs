
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
    // Part 1
    let max = std::fs::read_to_string("input.txt")
        .expect("could not open input.txt")
        .lines()
        .map(|l| Seat::from(l).id)
        .max();

    println!("{}", max.unwrap());
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
