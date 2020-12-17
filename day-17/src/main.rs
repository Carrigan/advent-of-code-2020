use std::num;

#[derive(Debug)]
struct Cube {
    w: i32,
    x: i32,
    y: i32,
    z: i32
}

struct Universe {
    active_cubes: Vec<Cube>
}

impl Universe {
    fn new(path: &str) -> Self {
        let mut active_cubes = Vec::new();

        for (y, line) in std::fs::read_to_string(path).unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' { active_cubes.push(Cube { x: x as i32, y: y as i32, z: 0, w: 0 }); }
            }
        }

        Universe { active_cubes }
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut w_min = 10000;
        let mut w_max = 0;
        let mut x_min = 10000;
        let mut x_max = 0;
        let mut y_min = 10000;
        let mut y_max = 0;
        let mut z_min = 10000;
        let mut z_max = 0;

        for cube in self.active_cubes.iter() {
            if cube.w < w_min { w_min = cube.w; }
            if cube.w > w_max { w_max = cube.w; }
            if cube.x < x_min { x_min = cube.x; }
            if cube.x > x_max { x_max = cube.x; }
            if cube.y < y_min { y_min = cube.y; }
            if cube.y > y_max { y_max = cube.y; }
            if cube.z < z_min { z_min = cube.z; }
            if cube.z > z_max { z_max = cube.z; }
        }

        BoundingBox { w_min, w_max, x_min, x_max, y_min, y_max, z_min, z_max }
    }

    fn active_cubes_adjacent_to(&self, cube: &Cube) -> usize {
        self.active_cubes
            .iter()
            .filter(|c| {
                i32::abs(c.w - cube.w) <= 1 &&
                i32::abs(c.x - cube.x) <= 1 &&
                i32::abs(c.y - cube.y) <= 1 &&
                i32::abs(c.z - cube.z) <= 1 &&
                !((cube.w == c.w) && (cube.y == c.y) && (cube.z == c.z) && (cube.x == c.x))
            })
            .count()
    }

    fn step_3d(&mut self) {
        let mut next_generation = Vec::new();
        let bounding_box = self.bounding_box();

        for x in (bounding_box.x_min - 1)..=(bounding_box.x_max + 1) {
            for y in (bounding_box.y_min - 1)..=(bounding_box.y_max + 1) {
                for z in (bounding_box.z_min- 1)..=(bounding_box.z_max + 1) {
                    let cube = Cube { w: 0, x, y, z };
                    let nearby_active = self.active_cubes_adjacent_to(&cube);

                    if nearby_active < 2 || nearby_active > 3 { continue; }

                    let is_active = self.active_cubes
                        .iter()
                        .find(|c| c.x == x && c.y == y && c.z == z)
                        .is_some();

                    let next_active_state = match (is_active, nearby_active) {
                        (true, 2..=3) => true,
                        (false, 3) => true,
                        _ => false
                    };

                    if next_active_state { next_generation.push(cube); }
                }
            }
        }

        self.active_cubes = next_generation;
    }

    fn step_4d(&mut self) {
        let mut next_generation = Vec::new();
        let bounding_box = self.bounding_box();

        for w in (bounding_box.w_min - 1)..=(bounding_box.w_max + 1) {
            for x in (bounding_box.x_min - 1)..=(bounding_box.x_max + 1) {
                for y in (bounding_box.y_min - 1)..=(bounding_box.y_max + 1) {
                    for z in (bounding_box.z_min- 1)..=(bounding_box.z_max + 1) {
                        let cube = Cube { w, x, y, z };
                        let nearby_active = self.active_cubes_adjacent_to(&cube);

                        if nearby_active < 2 || nearby_active > 3 { continue; }

                        let is_active = self.active_cubes
                            .iter()
                            .find(|c| c.w == w && c.x == x && c.y == y && c.z == z)
                            .is_some();

                        let next_active_state = match (is_active, nearby_active) {
                            (true, 2..=3) => true,
                            (false, 3) => true,
                            _ => false
                        };

                        if next_active_state { next_generation.push(cube); }
                    }
                }
            }
        }

        self.active_cubes = next_generation;
    }
}

struct BoundingBox {
    w_min: i32,
    w_max: i32,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32
}

fn main() {
    let mut universe = Universe::new("input.txt");
    (0..6).for_each(|_| universe.step_3d());
    println!("Part one: {}", universe.active_cubes.len());

    let mut universe = Universe::new("input.txt");
    (0..6).for_each(|_| universe.step_4d());
    println!("Part two: {}", universe.active_cubes.len());
}


#[test]
fn test_part_one() {
    let mut universe = Universe::new("example1.txt");
    (0..6).for_each(|_| universe.step_3d());
    assert_eq!(universe.active_cubes.len(), 112);
}

#[test]
fn test_part_two() {
    let mut universe = Universe::new("example1.txt");
    (0..6).for_each(|_| universe.step_4d());
    assert_eq!(universe.active_cubes.len(), 848);
}