use std::fs;

fn find_sum_of_two(numbers: Vec<i32>) -> (i32, i32) {
    let mut numbers = numbers;

    let n1 = numbers.remove(0);
    for n2 in &numbers {
        if n1 + n2 == 2020 { return (n1, *n2) };
    }

    find_sum_of_two(numbers)
}

fn find_sum_of_three(numbers: Vec<i32>) -> (i32, i32, i32) {
    let mut numbers = numbers;

    let n1 = numbers.remove(0);

    for index_n2 in 0..(numbers.len() - 1) {
        let n2 = numbers.get(index_n2).unwrap();
        
        for index_n3 in (index_n2 + 1)..numbers.len() {
            let n3 = numbers.get(index_n3).unwrap();

            if n1 + n2 + n3 == 2020 {
                return (n1, *n2, *n3);
            }
        }
    }

    find_sum_of_three(numbers)
}

fn main() {
    
    let mut numbers: Vec<i32> = Vec::new();

    let input = fs::read_to_string("input").unwrap();
    for entry in input.lines() {
        numbers.push(entry.parse::<i32>().unwrap());
    }

    // Part 1
    // let (n1, n2) = find_sum_of_two(numbers);
    // println!("{} * {} = {}", n1, n2, n1 * n2);

    // Part 2
    let (n1, n2, n3) = find_sum_of_three(numbers);
    println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
}

#[test]
fn test_sum_two() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let (n1, n2) = find_sum_of_two(numbers);

    assert_eq!(n1 * n2, 514579);
}

#[test]
fn test_sum_three() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let (n1, n2, n3) = find_sum_of_three(numbers);

    assert_eq!(n1 * n2 * n3, 241861950);
}
