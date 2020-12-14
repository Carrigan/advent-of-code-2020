use std::collections::HashMap;
use regex::Regex;

struct Mask {
    or_value: u64,
    and_value: u64
}

impl Mask {
    fn apply_to(&self, n: u64) -> u64 {
        (n & self.and_value) | self.or_value
    }
}

impl From<&str> for Mask {
    fn from(input: &str) -> Self {
        let mut and_value = 0xFFFF_FFFF_FFFF_FFFF;
        let mut or_value  = 0x0000_0000_0000_0000;

        input.chars().enumerate().for_each(|(index, c)| match c {
            '1' => { or_value = or_value | (1u64 << (35 - index)); },
            '0'=> { and_value = and_value & !(1u64 << (35 - index)); },
            _ => ()
        });

        Mask { or_value, and_value }
    }
}

fn run_bitmask_program(path: &str) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut current_mask = Mask::from("");
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        if &line[0..4] == "mask" {
            current_mask = Mask::from(&line[7..]);
        } else {
            let captures = mem_regex.captures(line).unwrap();
            let memory_address = captures[1].parse().unwrap();
            let value = captures[2].parse().unwrap();

            memory.insert(memory_address, current_mask.apply_to(value));
        }
    }

    memory
}

struct MemoryMask {
    floating_indeces: Vec<usize>,
    mask: u64
}

impl MemoryMask {
    fn floating_iter<'a>(&'a self, base: u64) -> FloatingIterator {
        FloatingIterator {
            floating_indeces: &self.floating_indeces,
            iterator: 0,
            base: base | self.mask
        }
    }
}

struct FloatingIterator<'a> {
    floating_indeces: &'a Vec<usize>,
    iterator: usize,
    base: u64
}

impl <'a> Iterator for FloatingIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator == (1 << self.floating_indeces.len()) { return None; }

        let mut output = self.base;
        for (position_index, floating_index) in self.floating_indeces.iter().enumerate() {
            let shifted_bit = 1u64 << floating_index;

            output = match self.iterator & (1 << position_index) {
                0 => output & !shifted_bit,
                _ => output | shifted_bit
            }

        }

        self.iterator += 1;
        Some(output)
    }
}

impl From<&str> for MemoryMask {
    fn from(input: &str) -> Self {
        let mut mask  = 0x0000_0000_0000_0000;
        let mut floating_indeces = Vec::new();

        input.chars().enumerate().for_each(|(index, c)| match c {
            '1' => { mask = mask | (1u64 << (35 - index)); },
            'X'=> { floating_indeces.push(35 - index) },
            _ => ()
        });

        MemoryMask { floating_indeces, mask }
    }
}


fn run_memory_address_program(path: &str) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut current_mask = MemoryMask::from("");
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        if &line[0..4] == "mask" {
            current_mask = MemoryMask::from(&line[7..]);
        } else {
            let captures = mem_regex.captures(line).unwrap();
            let memory_address = captures[1].parse().unwrap();
            let value = captures[2].parse().unwrap();

            for potential in current_mask.floating_iter(memory_address) {
                memory.insert(potential, value);
            }
        }
    }

    memory
}

fn main() {
    // Part One
    let output = run_bitmask_program("input.txt");
    println!("Part 1: {}", output.values().sum::<u64>());

    // Part Two
    let output = run_memory_address_program("input.txt");
    println!("Part 2: {}", output.values().sum::<u64>());
}

#[test]
fn test_part_one() {
    let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(mask.or_value, 0b1000000);
    assert_eq!(mask.and_value, 0xFFFF_FFFF_FFFF_FFFD);
    assert_eq!(mask.apply_to(11), 73);

    let output = run_bitmask_program("example1.txt");
    assert_eq!(output.values().sum::<u64>(), 165);
}

#[test]
fn test_part_two() {
    let output = run_memory_address_program("example2.txt");
    assert_eq!(output.values().sum::<u64>(), 208);
}
