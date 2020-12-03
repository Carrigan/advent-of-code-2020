use std::fs;

enum Terrain {
    Clear,
    Tree
}

impl From<char> for Terrain {
    fn from(c: char) -> Terrain {
        match c {
            '.' => Terrain::Clear,
            _ => Terrain::Tree
        }
    }
}

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

fn toboggan_traverse(world: &World) -> usize {
    let width = world.width();
    let mut count = 0;
    let mut x_index = 0;

    for line in world.data.lines() {
        count = match Terrain::from(line.chars().nth(x_index).unwrap()) {
            Terrain::Clear => count,
            Terrain::Tree => count + 1
        };

        x_index += 3;
        if x_index >= width {
            x_index -= width;
        }
    }

    count
}

fn main() {
    let world = World::new("input.txt");
    let tree_count = toboggan_traverse(&world);

    println!("Tree count: {}", tree_count);
}

#[test]
fn test_example() {
    let world = World::new("example.txt");
    let tree_count = toboggan_traverse(&world);

    assert_eq!(tree_count, 7);
}
