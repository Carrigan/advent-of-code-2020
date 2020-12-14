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

fn run_program(path: &str) -> HashMap<u64, u64> {
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

fn main() {
    // Part One
    let output = run_program("input.txt");
    println!("Part 1: {}", output.values().sum::<u64>());
}

#[test]
fn test_part_one() {
    let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(mask.or_value, 0b1000000);
    assert_eq!(mask.and_value, 0xFFFF_FFFF_FFFF_FFFD);
    assert_eq!(mask.apply_to(11), 73);

    let output = run_program("example1.txt");
    assert_eq!(output.values().sum::<u64>(), 165);
}
