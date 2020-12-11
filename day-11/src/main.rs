use std::{fmt::Display};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
    Vacant,
    Occupied,
    Floor
}

#[derive(Copy, Clone)]
enum AdjacencyMethod {
    Proximity,
    Sight
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Seat::Vacant => "L",
            Seat::Occupied => "#",
            Seat::Floor => "."
        };

        write!(f, "{}", out)
    }
}

impl From<char> for Seat {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Seat::Vacant,
            '#' => Seat::Occupied,
            _ => Seat::Floor
        }
    }
}

struct World {
    seats: Vec<Seat>,
    iterations: usize,
    length: usize,
    height: usize
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.seats.chunks(self.length) {
            for seat in row {
                write!(f, "{}", seat)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn falls_within(initial: usize, shift: i32, min: usize, max: usize) -> bool {
    let shifted = initial as i32 + shift;
    (shifted >= min as i32) && (shifted < max as i32)
}

impl World {
    fn new(path: &str) -> Self {
        let input = std::fs::read_to_string(path).unwrap();

        let seats: Vec<Seat> = input
            .lines()
            .map(|l| l.chars().map(|c| Seat::from(c)).collect::<Vec<Seat>>())
            .flatten()
            .collect();         
            
        let length = input.find(|c| c == '\n').unwrap();
        let height = seats.len() / length;

        World { seats, iterations: 0, length, height }
    }

    fn adjacent_indeces_by_proximity(&self, index: usize) -> Vec<usize> {
        let row = index / self.length;
        let col = index % self.length;

        let translations: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        translations.iter()
            .filter(|(x, y)| 
                falls_within(row, *y, 0, self.height) && 
                falls_within(col, *x, 0, self.length)
            )
            .map(|(x, y)| (index as i32+ (y * self.length as i32) + *x) as usize)
            .collect()
    }

    fn adjacent_indeces_by_sight(&self, index: usize) -> Vec<usize> {
        let row = index / self.length;
        let col = index % self.length;

        let translations: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        let mut indeces_out = Vec::new();

        for (shift_x, shift_y) in translations.iter() {
            let mut x = col;
            let mut y = row;

            loop {
                // Break if the shift cannot be performed
                if !falls_within(x, *shift_x, 0, self.length) { break; }
                if !falls_within(y, *shift_y, 0, self.height) { break; }
                
                // Perform the shift
                x = (x as i32 + shift_x) as usize;
                y = (y as i32 + shift_y) as usize;

                // If we hit a seat, return that
                let current_index = y * self.length + x;
                match self.seats[current_index] {
                    Seat::Occupied | Seat::Vacant => {
                        indeces_out.push(current_index);
                        break;
                    },
                    Seat::Floor => ()
                }
            }

        }

        indeces_out
    }

    fn next_state_for(&self, index: usize, method: AdjacencyMethod) -> Seat {
        if self.seats[index] == Seat::Floor { return Seat::Floor; }

        let adjacent_tiles = match method {
            AdjacencyMethod::Proximity => self.adjacent_indeces_by_proximity(index),
            AdjacencyMethod::Sight => self.adjacent_indeces_by_sight(index)
        };

        let adjacent_count = adjacent_tiles
            .iter()
            .map(|i| if self.seats[*i] == Seat::Occupied { 1 } else { 0 })
            .sum();

        match method { 
            AdjacencyMethod::Proximity => {
                match adjacent_count {
                    0 => Seat::Occupied,
                    1..=3 => self.seats[index],
                    _ => Seat::Vacant
                }
            },
            AdjacencyMethod::Sight => {
                match adjacent_count {
                    0 => Seat::Occupied,
                    1..=4 => self.seats[index],
                    _ => Seat::Vacant
                }
            }
        }
    }

    fn step(&mut self, method: AdjacencyMethod) -> bool {
        let mut changed = false;
        let mut new_seats = Vec::new();

        for index in 0..self.seats.len() {
            let current_state = self.seats[index];
            let new_state = self.next_state_for(index, method);

            if new_state != current_state { changed = true; }

            new_seats.push(new_state);
        }

        // Update the iterations
        if !changed { return false; }
        self.iterations += 1;
        self.seats = new_seats;
        changed
    }

    fn run_until_stabilized(&mut self, method: AdjacencyMethod) {
        while self.step(method) {}
    }   
}

fn main() {
    // Part One
    let mut world = World::new("input.txt");
    world.run_until_stabilized(AdjacencyMethod::Proximity);

    println!(
        "Part one: it took {} iterations to stabilize with {} occupied seats", 
        world.iterations, 
        world.seats.iter().filter(|s| **s == Seat::Occupied).count()
    );

    // Part two
    let mut world = World::new("input.txt");
    world.run_until_stabilized(AdjacencyMethod::Sight);

    println!(
        "Part two: it took {} iterations to stabilize with {} occupied seats", 
        world.iterations, 
        world.seats.iter().filter(|s| **s == Seat::Occupied).count()
    );
}

#[test]
fn test_part_one() {
    let mut world = World::new("example.txt");
    world.run_until_stabilized(AdjacencyMethod::Proximity);

    assert_eq!(world.iterations, 5);
    assert_eq!(world.seats.iter().filter(|s| **s == Seat::Occupied).count(), 37);
}

#[test]
fn test_part_two() {
    let mut world = World::new("example.txt");
    world.run_until_stabilized(AdjacencyMethod::Sight);

    assert_eq!(world.iterations, 6);
    assert_eq!(world.seats.iter().filter(|s| **s == Seat::Occupied).count(), 26);
}