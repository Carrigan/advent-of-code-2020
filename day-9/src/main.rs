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

fn find_continuous_sum(numbers: &Vec<u64>, target: u64) -> Option<(usize, usize)> {
    for index in 0..numbers.len() {
        let mut sum = numbers[index];
        let mut inner_index = index + 1;

        while sum < target {
            sum += numbers[inner_index];
            inner_index += 1;
        }

        if sum == target {
            return Some((index, inner_index - index));
        }       
    }

    None
}

fn sum_of_extremes(numbers: &[u64]) -> u64 {
    numbers.iter().min().unwrap() + numbers.iter().max().unwrap()
}

fn main() {
    let numbers = read_input("input.txt");

    // Part 1
    let invalid_number = (25..numbers.len())
        .find(|x| !is_valid(*x, &numbers, 25))
        .unwrap();

    println!("The first invalid number is: {}", numbers[invalid_number]);

    // Part 2
    let (index, length) = find_continuous_sum(&numbers, numbers[invalid_number]).unwrap();
    let slice = &numbers[index..index + length];

    println!("The sum of the highest and lowest number in a continuous slice that add to that are: {}", sum_of_extremes(&slice));

}

#[test]
fn test_find_invalid() {
    let numbers = read_input("example.txt");
    let invalid_numbers: Vec<usize> = (5..numbers.len())
        .filter(|x| !is_valid(*x, &numbers, 5))
        .collect();

    assert_eq!(invalid_numbers.len(), 1);
    assert_eq!(numbers[invalid_numbers[0]], 127);
}

#[test]
fn test_sum_to_invalid() {
    let numbers = read_input("example.txt");
    let (index, length) = find_continuous_sum(&numbers, 127).unwrap();

    assert_eq!(index, 2);
    assert_eq!(length, 4);

    let slice = &numbers[index..index + length];
    assert_eq!(sum_of_extremes(&slice), 62);
}
