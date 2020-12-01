use std::fs;

fn find_sum_of_two(numbers: &[i32], sum: i32) -> Option<(i32, i32)> {
    if numbers.len() <= 1 { return None; }

    let n1 = numbers[0];

    for n2 in &numbers[1..] {
        if n1 + n2 == sum { return Some((n1, *n2)) };
    }

    find_sum_of_two(&numbers[1..], sum)
}

fn find_sum_of_three(numbers: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    if numbers.len() <= 2 { return None; }

    let n1 = numbers[0];

    match find_sum_of_two(&numbers[1..], sum - n1) {
        Some((n2, n3)) => Some((n1, n2, n3)),
        None => find_sum_of_three(&numbers[1..], sum)
    }
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    let input = fs::read_to_string("input").unwrap();
    for entry in input.lines() {
        numbers.push(entry.parse::<i32>().unwrap());
    }

    // Part 1
    if let Some((n1, n2)) = find_sum_of_two(numbers.as_slice(), 2020) {
        println!("{} * {} = {}", n1, n2, n1 * n2);
    }

    // Part 2
    if let Some((n1, n2, n3)) = find_sum_of_three(numbers.as_slice(), 2020) {
        println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
    }
}

#[test]
fn test_sum_two() {
    let numbers = [1721, 979, 366, 299, 675, 1456];
    let (n1, n2) = find_sum_of_two(&numbers, 2020).unwrap();

    assert_eq!(n1 * n2, 514579);
}

#[test]
fn test_sum_three() {
    let numbers = [1721, 979, 366, 299, 675, 1456];
    let (n1, n2, n3) = find_sum_of_three(&numbers, 2020).unwrap();

    assert_eq!(n1 * n2 * n3, 241861950);
}
