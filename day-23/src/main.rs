fn decrement_with_wrap(value: usize, max: usize) -> usize {
    if value == 0 { max - 1 } else { value - 1 }
}

fn preallocate_large_memory(size: usize) -> Vec<usize> {
    let mut values = Vec::with_capacity(1_000_000);
    (0..1_000_000).for_each(|n| values.push(n));

    values
}


// Note: all values are -1.
struct Cups<'a> {
    nexts: &'a mut [usize],
    current_value: usize
}

impl <'a> Cups <'a> {
    fn new(input: &str, nexts: &'a mut [usize]) -> Self {
        let input_values: Vec<usize> = input.chars()
            .map(|c| c.to_digit(10).map(|v| (v - 1) as usize).unwrap())
            .collect();

        let mut previous_value = match input.len() < nexts.len() {
            true => nexts.len() - 1,
            false => *input_values.last().unwrap()
        };

        // Iterate through and fill the premade values
        for &initial_value in input_values.iter() {
            nexts[previous_value] = initial_value;
            previous_value = initial_value;
        }

        // Then fill the rest sequentially
        for n in input.len()..nexts.len() {
            nexts[previous_value] = n;
            previous_value = n;
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
    let mut values = [0; 9];
    let mut cups = Cups::new("598162734", &mut values);
    for _ in 0..100 { cups.play_round(); }

    let vals = cups.next_values(0, 9)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("Part one: {}", vals);

    // Part two:
    let mut values = preallocate_large_memory(1_000_000);
    let mut cups = Cups::new("598162734", &mut values);
    for _ in 0..10_000_000 { cups.play_round(); }

    let vals = cups.next_values(0, 3);
    println!("Part two: {}", vals[1] * vals[2]);
}

#[test]
fn test_part_one() {
    let mut values = [0; 9];
    let mut cups = Cups::new("389125467", &mut values);
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
    let mut values = preallocate_large_memory(1_000_000);
    let mut cups = Cups::new("389125467", values.as_mut_slice());

    for _ in 0..10_000_000 { cups.play_round(); }
    let vals = cups.next_values(0, 3);

    assert_eq!(vals[1], 934001);
    assert_eq!(vals[2], 159792);
}
