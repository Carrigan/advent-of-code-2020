struct Cups(Vec<usize>);

fn decrement_with_wrap(value: usize) -> usize {
    if value == 1 { 9 } else { value - 1 }
}

impl Cups {
    fn new(input: &str) -> Self {
        let cups = input.chars()
            .map(|c| c.to_digit(10).map(|v| v as usize).unwrap() )
            .collect();

        Self(cups)
    }

    fn state_string(&self) -> String {
        self.0.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("")
    }

    fn play_round(&mut self) {
        let current_cup_value = self.0.remove(0);
        let removed_cups: Vec<usize> = self.0.drain(0..3).collect();

        let mut destination_cup = decrement_with_wrap(current_cup_value);
        while removed_cups.contains(&destination_cup) {
            destination_cup = decrement_with_wrap(destination_cup)
        }

        let destination_cup_position = self.0.iter()
            .position(|&c| c == destination_cup)
            .unwrap();

        removed_cups.iter().rev().for_each(|c|
            self.0.insert(destination_cup_position + 1, *c)
        );

        self.0.push(current_cup_value);
    }
}

fn main() {
    let mut cups = Cups::new("598162734");
    for _ in 0..100 { cups.play_round(); }
    println!("{}", cups.state_string());
}

#[test]
fn test_part_one() {
    let mut cups = Cups::new("389125467");
    for _ in 0..10 { cups.play_round(); }
    assert_eq!(cups.state_string(), "837419265");
}
