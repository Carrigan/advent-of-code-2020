fn read_input(path: &str) -> Vec<u64> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn is_valid(index: usize, numbers: &Vec<u64>, look_behind_amount: usize) -> bool {
    let possibilities = &numbers[(index - look_behind_amount)..index];
    let target = numbers[index];

    for lower_index in 0..(possibilities.len() - 1) {
        for upper_index in (1 + lower_index)..possibilities.len() {
            if possibilities[lower_index] + possibilities[upper_index] == target {
                return true;
            }
        }
    }

    false
}

fn main() {
    let numbers = read_input("input.txt");

    // Part 1
    let invalid_number = (25..numbers.len())
        .find(|x| !is_valid(*x, &numbers, 25))
        .unwrap();

    println!("The first invalid number is: {}", numbers[invalid_number]);

}

#[test]
fn test_example() {
    let numbers = read_input("example.txt");
    let invalid_numbers: Vec<usize> = (5..numbers.len())
        .filter(|x| !is_valid(*x, &numbers, 5))
        .collect();

    assert_eq!(invalid_numbers.len(), 1);
    assert_eq!(numbers[invalid_numbers[0]], 127);
}
