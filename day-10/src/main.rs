fn build_distribution(path: &str) -> (usize, usize) {
    let mut input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    input.sort();

    let mut ones = 0;
    let mut threes = 0;

    input.iter().zip([0].iter().chain(input.iter()))
        .for_each(|(larger, smaller)| {
            match larger - smaller {
                1 => ones += 1,
                3 => threes +=1,
                _ => ()
            }
        });

    (ones, threes + 1)
}

fn main() {
    // Part one
    let (ones, threes) = build_distribution("input.txt");
    println!("Part 1: {}", ones * threes);
}

#[test]
fn test_example() {
    let (ones, threes) = build_distribution("example2.txt");
    assert_eq!(ones, 7);
    assert_eq!(threes, 5);

    let (ones, threes) = build_distribution("example.txt");
    assert_eq!(ones, 22);
    assert_eq!(threes, 10);
}
