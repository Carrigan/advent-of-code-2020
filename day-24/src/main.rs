enum Move {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

// https://www.redblobgames.com/grids/hexagons/
// Using axial coordinates
impl Move {
    fn to_coordinate_offsets(&self) -> (i32, i32) {
        match self {
            Move::East => (1, 0),
            Move::SouthEast => (0, 1),
            Move::SouthWest => (-1, 1),
            Move::West => (-1, 0),
            Move::NorthWest => (0, -1),
            Move::NorthEast => (1, -1)
        }
    }
}

enum MovePrefix {
    South,
    North
}

fn parse_file(path: &str) -> Vec<Vec<Move>> {
    std::fs::read_to_string(path).unwrap().lines()
        .map(|l| parse_input(l))
        .collect()
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut prefix = None;

    input.chars().filter_map(|c| match c {
        'n' => {
            prefix = Some(MovePrefix::North);
            None
        },
        's' => {
            prefix = Some(MovePrefix::South);
            None
        }
        'e' => {
            let output = match prefix {
                Some(MovePrefix::South) => Some(Move::SouthEast),
                Some(MovePrefix::North) => Some(Move::NorthEast),
                None => Some(Move::East)
            };

            prefix = None;
            output
        }
        'w' => {
            let output = match prefix {
                Some(MovePrefix::South) => Some(Move::SouthWest),
                Some(MovePrefix::North) => Some(Move::NorthWest),
                None => Some(Move::West)
            };

            prefix = None;
            output
        }
        _ => panic!("unexpected character parsed")
    }).collect()
}

fn final_coordinates(moves: &Vec<Move>) -> (i32, i32) {
    moves
        .iter()
        .map(|m| m.to_coordinate_offsets())
        .fold((0, 0), |(x_total, y_total), (x, y)| {
            (x_total + x, y_total + y)
        })
}

fn derive_flipped_tiles(tiles: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut flipped_tiles = Vec::new();
    for tile in tiles {
        match flipped_tiles.iter().position(|t| t == tile) {
            Some(position) => { flipped_tiles.remove(position); },
            None => { flipped_tiles.push(*tile); }
        };
    }

    flipped_tiles
}

fn main() {
    // Part One
    let tiles_to_flip: Vec<(i32, i32)> = parse_file("input.txt")
        .iter()
        .map(|line| final_coordinates(line))
        .collect();

    let flipped_tiles = derive_flipped_tiles(&tiles_to_flip);
    println!("Part one: {}", flipped_tiles.len());
}

#[test]
fn test_part_one() {
    let tiles_to_flip: Vec<(i32, i32)> = parse_file("example1.txt")
        .iter()
        .map(|line| final_coordinates(line))
        .collect();

    let flipped_tiles = derive_flipped_tiles(&tiles_to_flip);
    assert_eq!(flipped_tiles.len(), 10);
}
