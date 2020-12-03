use std::fs;

struct World {
    data: String
}

impl World {
    fn new(path: &str) -> Self {
        World { data: fs::read_to_string(path).expect("could not open file") }
    }

    fn width(&self) -> usize {
        self.data.lines().nth(0).unwrap().len()
    }
}

fn toboggan_traverse(world: &World, x_step: usize, y_step: usize) -> usize {
    let width = world.width();
    let mut count = 0;
    let mut x_index = 0;
    let mut y_index = 0;

    loop {
        let line = match world.data.lines().nth(y_index) {
            Some(l) => l,
            None => break
        };

        count = match line.chars().nth(x_index).unwrap() {
            '.' => count,
            _ => count + 1
        };

        x_index += x_step;
        if x_index >= width { x_index -= width; }

        y_index += y_step;
    }

    count
}

fn main() {
    // Part 1
    let world = World::new("input.txt");
    let tree_count = toboggan_traverse(&world, 3, 1);

    println!("(3, 1) count: {}", tree_count);

    // Part 2
    let multiplied_trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| toboggan_traverse(&world, *x, *y))
        .fold(1, |total, current| total * current);

    println!("Multiplied aggregate count: {}", multiplied_trees);
}

#[test]
fn test_example() {
    let world = World::new("example.txt");

    // Part 1
    let tree_count = toboggan_traverse(&world, 3, 1);
    assert_eq!(tree_count, 7);

    // Part 2
    let multiplied_trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| toboggan_traverse(&world, *x, *y))
        .fold(1, |total, current| total * current);

    assert_eq!(multiplied_trees, 336);
}
