fn sorted_adapters(path: &str) -> Vec<u32> {
    let mut input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    input.sort();

    input
}

fn build_distribution(input: &Vec<u32>) -> (usize, usize) {
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

// Could be made more efficient with a cache
fn ways(run: u64) -> u64 {
    match run {
        0 | 1 | 2 => 1,
        3 => 2,
        r => ways(r - 3) + ways(r - 2) + ways(r - 1)
    }
}

fn possibilities_for(adapters: &Vec<u32>) -> u64 {
    // Build a diff list
    let mut diffs: Vec<u32> = adapters.iter().zip([0].iter().chain(adapters.iter()))
        .map(|(larger, smaller)| larger - smaller)
        .collect();

    diffs.push(3);

    // Find all of the runs in it
    let (runs, _) = diffs.iter().fold((Vec::new(), 1 as u64), |(mut runs, current_run), diff| {
        match diff {
            1 => (runs, current_run + 1),
            3 => { runs.push(current_run); (runs, 1) },
            _ => panic!()
        }
    });

    // Each run has some number of different combos; multiply them all
    runs.iter().map(|r| ways(*r)).product()
}

fn main() {
    let adapters = sorted_adapters("input.txt");

    // Part one
    let (ones, threes) = build_distribution(&adapters);
    println!("Part 1: {}", ones * threes);

    // Part two
    println!("Part 2: {}", possibilities_for(&adapters));
}

#[test]
fn test_part_one() {
    let adapters = sorted_adapters("example2.txt");
    let (ones, threes) = build_distribution(&adapters);
    assert_eq!(ones, 7);
    assert_eq!(threes, 5);

    let adapters = sorted_adapters("example.txt");
    let (ones, threes) = build_distribution(&adapters);
    assert_eq!(ones, 22);
    assert_eq!(threes, 10);
}

#[test]
fn test_part_two() {
    let adapters = sorted_adapters("example2.txt");
    let possibilities = possibilities_for(&adapters);
    assert_eq!(possibilities, 8);

    let adapters = sorted_adapters("example.txt");
    let possibilities = possibilities_for(&adapters);
    assert_eq!(possibilities, 19208);
}
