struct MemoryGame {
    numbers: Vec<u32>
}

impl MemoryGame {
    fn new(starters: &[u32]) -> Self {
        Self { numbers: starters.iter().map(|n| *n).collect() }
    }

    fn generate(&mut self, size: usize) {
        while self.numbers.len() < size {
            let (rest, final_number) = self.numbers.split_at(self.numbers.len() - 1);
            let last_heard_index = rest.iter().rev().position(|&x| x == final_number[0]);

            let next_number = match last_heard_index {
                Some(index) => index as u32 + 1,
                None => 0
            };

            self.numbers.push(next_number);
        }
    }

}

fn main() {
    // Part one
    let mut game = MemoryGame::new(&[17, 1, 3, 16, 19, 0]);
    game.generate(2020);
    println!("Part one: {}", *game.numbers.last().unwrap());
}

#[test]
fn test_part_one() {
    let mut game = MemoryGame::new(&[0, 3, 6]);
    game.generate(2020);

    assert_eq!(*game.numbers.last().unwrap(), 436);
}