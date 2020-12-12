enum Instruction {
    Forward(u32),
    Left(u32),
    Right(u32),
    North(u32),
    South(u32),
    East(u32),
    West(u32)
}

impl From<&str> for Instruction {
    fn from(st: &str) -> Self {
        let amount = st[1..].parse().unwrap();

        match st.chars().nth(0).unwrap() {
            'N' => Instruction::North(amount),
            'S' => Instruction::South(amount),
            'E' => Instruction::East(amount),
            'W' => Instruction::West(amount),
            'L' => Instruction::Left(amount),
            'R' => Instruction::Right(amount),
            'F' => Instruction::Forward(amount),
            _ => panic!()
        }
    }
}

struct Ship {
    heading: u16,
    x: i32,
    y: i32
}

struct Waypoint {
    x: i32,
    y: i32
}

impl Waypoint {
    fn new() -> Self {
        Waypoint { x: 10, y: 1 }
    }

    fn rotate_ccw(&mut self, amount: u32) {
        let mut amount_left = amount;

        while amount_left > 0 {
            let (x, y) = (self.x, self.y);

            self.x = -y;
            self.y = x;

            amount_left -= 90;
        }
    }

    fn rotate_cw(&mut self, amount: u32) {
        let mut amount_left = amount;

        while amount_left > 0 {
            let (x, y) = (self.x, self.y);

            self.x = y;
            self.y = -x;

            amount_left -= 90;
        }
    }

    fn apply(&mut self, ship: &mut Ship, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                ship.x += self.x * (*amount as i32);
                ship.y += self.y * (*amount as i32);
            },
            Instruction::Left(amount) => self.rotate_ccw(*amount),
            Instruction::Right(amount) => self.rotate_cw(*amount),
            Instruction::North(amount) => self.y += *amount as i32,
            Instruction::South(amount) => self.y -= *amount as i32,
            Instruction::East(amount) => self.x += *amount as i32,
            Instruction::West(amount) => self.x -= *amount as i32
        }
    }

    fn execute(&mut self, ship: &mut Ship, input: &str) {
        std::fs::read_to_string(input).unwrap()
            .lines()
            .map(|line| Instruction::from(line))
            .for_each(|instr| self.apply(ship, &instr));
    }
}

impl Ship {
    fn new() -> Self {
        Ship { heading: 0, x: 0, y: 0 }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => match self.heading {
                0 => self.apply(&Instruction::East(*amount)),
                90 => self.apply(&Instruction::North(*amount)),
                180 => self.apply(&Instruction::West(*amount)),
                270 => self.apply(&Instruction::South(*amount)),
                _ => panic!()
            },
            Instruction::Left(amount) => self.heading = turn_degrees(self.heading, *amount as i32),
            Instruction::Right(amount) => self.heading = turn_degrees(self.heading, -(*amount as i32)),
            Instruction::North(amount) => self.y += *amount as i32,
            Instruction::South(amount) => self.y -= *amount as i32,
            Instruction::East(amount) => self.x += *amount as i32,
            Instruction::West(amount) => self.x -= *amount as i32
        }
    }

    fn execute(&mut self, input: &str) {
        std::fs::read_to_string(input).unwrap()
            .lines()
            .map(|line| Instruction::from(line))
            .for_each(|instr| self.apply(&instr));
    }
}

fn turn_degrees(heading: u16, turn: i32) -> u16 {
    let mut raw = (heading as i32 + turn) % 360;
    while raw < 0 { raw += 360; }
    raw as u16
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    // Part 1
    let mut ship = Ship::new();
    ship.execute("input.txt");
    println!("Part one: {}", manhattan_distance(ship.x, ship.y, 0, 0));

    // Part 2
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();

    waypoint.execute(&mut ship, "input.txt");

    println!("Part two: {}", manhattan_distance(ship.x, ship.y, 0, 0));
}

#[test]
fn test_part_1() {
    let mut ship = Ship::new();
    ship.execute("example.txt");

    assert_eq!(manhattan_distance(ship.x, ship.y, 0, 0), 25);
}

#[test]
fn test_part_2() {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    waypoint.execute(&mut ship, "example.txt");

    assert_eq!(manhattan_distance(ship.x, ship.y, 0, 0), 286);
}