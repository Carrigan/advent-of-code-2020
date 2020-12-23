fn decrement_with_wrap(value: usize, max: usize) -> usize {
    if value == 0 { max - 1 } else { value - 1 }
}

// Note: all values are -1.
struct Cups {
    nexts: Vec<usize>,
    current_value: usize
}

impl Cups {
    fn new(input: &str, size: usize) -> Self {
        // Prefill the array
        let mut nexts = Vec::with_capacity(size);
        (0..size).for_each(|n| nexts.push(n + 1));

        // Fill in the input values
        let input_values: Vec<usize> = input.chars()
            .map(|c| c.to_digit(10).map(|v| (v - 1) as usize).unwrap())
            .collect();

        let mut previous_value = match input.len() < size {
            true => size - 1,
            false => *input_values.last().unwrap()
        };

        // Iterate through and fill the premade values
        for &initial_value in input_values.iter() {
            nexts[previous_value] = initial_value;
            previous_value = initial_value;
        }

        // Point to the remainder of the array
        if size > input_values.len() {
            nexts[previous_value] = input_values.len();
        }

        Self { nexts, current_value: *input_values.first().unwrap() }
    }

    fn play_round(&mut self) {
        // Mark the beginning and ends of the picked up area
        let picked_up_start = self.nexts[self.current_value];
        let mut picked_up_end = picked_up_start;
        for _ in 0..2 { picked_up_end = self.nexts[picked_up_end]; }
        let after_picked_up = self.nexts[picked_up_end];

        // Find the destination cup value
        let mut destination_cup_value = decrement_with_wrap(self.current_value, self.nexts.len());

        // While its a number in the picked up section, continue to decrement with wrap
        loop {
            let mut picked_up_value = self.nexts[self.current_value];
            let value_in_picked_up_section = (0..3).any(|_| {
                let same = picked_up_value == destination_cup_value;
                picked_up_value = self.nexts[picked_up_value];

                same
            });

            if value_in_picked_up_section {
                destination_cup_value = decrement_with_wrap(destination_cup_value, self.nexts.len());
            } else {
                break;
            }
        }

        // Find what cup is currently after the destination cup
        let after_destination = self.nexts[destination_cup_value];

        // Make the current value point at what the removed section pointed to
        self.nexts[self.current_value] = after_picked_up;

        // Insert the removed section after the destination
        self.nexts[destination_cup_value] = picked_up_start;
        self.nexts[picked_up_end] = after_destination;

        // Finally, point to the next value
        self.current_value = self.nexts[self.current_value];
    }

    fn next_values(&self, index: usize, length: usize) -> Vec<usize> {
        let mut current_value = index;

        (0..length).map(|_| {
            let value = current_value;
            current_value = self.nexts[value];

            value + 1
        }).collect()
    }
}

fn main() {
    // Part One:
    let mut cups = Cups::new("598162734", 9);
    for _ in 0..100 { cups.play_round(); }

    let vals = cups.next_values(0, 9)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("Part one: {}", vals);

    // Part two:
    let mut cups = Cups::new("598162734", 1_000_000);
    for _ in 0..10_000_000 { cups.play_round(); }

    let vals = cups.next_values(0, 3);
    println!("Part two: {}", vals[1] * vals[2]);
}

#[test]
fn test_part_one() {
    let mut cups = Cups::new("389125467", 9);
    println!("{:?}", cups.nexts);
    for _ in 0..100 { cups.play_round(); }

    let vals = cups.next_values(0, 9)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq!(&vals[1..], "67384529");
}

#[test]
fn test_part_two() {
    let mut cups = Cups::new("389125467", 1_000_000);
    for _ in 0..10_000_000 { cups.play_round(); }
    let vals = cups.next_values(0, 3);

    assert_eq!(vals[1], 934001);
    assert_eq!(vals[2], 159792);
}
