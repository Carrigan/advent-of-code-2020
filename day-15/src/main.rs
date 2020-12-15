use std::collections::HashMap;

#[derive(Debug)]
struct MemoryGame {
    index: usize,
    last_number: usize,
    last_indeces: HashMap<usize, usize>
}

impl MemoryGame {
    fn new(starters: &[usize]) -> Self {
        let mut last_indeces = HashMap::new();
        for (i, &n) in starters[..starters.len() - 1].iter().enumerate() {
            last_indeces.insert(n, i);
        }

        Self { index: starters.len() - 1, last_indeces, last_number: *starters.last().unwrap() }
    }

    fn generate(&mut self, size: usize) {
        while self.index < size - 1 {
            let next_number = match self.last_indeces.get(&self.last_number) {
                Some(index) => self.index - index,
                None => 0
            };

            self.last_indeces.insert(self.last_number, self.index);
            self.index += 1;
            self.last_number = next_number;
        }
    }

}

fn main() {
    // Part one
    let mut game = MemoryGame::new(&[17, 1, 3, 16, 19, 0]);
    game.generate(2020);
    println!("Part one: {}", game.last_number);

    // Part one
    game.generate(30000000);
    println!("Part two: {}", game.last_number);
}

#[test]
fn test_part_one() {
    let mut game = MemoryGame::new(&[0, 3, 6]);

    game.generate(2020);
    assert_eq!(game.last_number, 436);

    game.generate(30000000);
    assert_eq!(game.last_number, 175594);
}