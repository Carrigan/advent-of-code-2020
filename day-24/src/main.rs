use itertools::Itertools;

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

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
struct Tile(i32, i32);

impl Tile {
    fn adjacent_tiles(&self) -> TileAdjacencyIterator {
        TileAdjacencyIterator { tile: self.clone(), index: 0 }
    }
}

struct TileAdjacencyIterator {
    tile: Tile,
    index: usize
}

impl Iterator for TileAdjacencyIterator {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 6 { return None; }

        let transforms = [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)];
        let transform = transforms[self.index];
        let output = Tile(self.tile.0 + transform.0, self.tile.1 + transform.1);

        self.index += 1;
        Some(output)
    }
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

fn final_coordinates(moves: &Vec<Move>) -> Tile {
    moves
        .iter()
        .map(|m| m.to_coordinate_offsets())
        .fold(Tile(0, 0), |Tile(x_total, y_total), (x, y)| {
            Tile(x_total + x, y_total + y)
        })
}

fn derive_flipped_tiles(tiles: &Vec<Tile>) -> Vec<Tile> {
    let mut flipped_tiles = Vec::new();
    for tile in tiles {
        match flipped_tiles.iter().position(|t| t == tile) {
            Some(position) => { flipped_tiles.remove(position); },
            None => { flipped_tiles.push(*tile); }
        };
    }

    flipped_tiles
}

fn build_next_state(flipped_tiles: &Vec<Tile>) -> Vec<Tile> {
    // Since our data is sparse, evaluate all known black tile AND adjacent while tiles
    let tiles_to_evaluate = flipped_tiles
        .iter()
        .map(|tile| {
            let mut adjacent = tile.adjacent_tiles().collect::<Vec<Tile>>();
            adjacent.push(*tile);

            adjacent
        })
        .flatten()
        .unique()
        .collect::<Vec<Tile>>();

    // For each tile, evaluate its next stage
    tiles_to_evaluate
        .iter()
        .filter(|tile| {
            let adjacent_flipped_count = tile
                .adjacent_tiles()
                .filter(|adj| flipped_tiles.contains(adj))
                .count();

            match flipped_tiles.contains(tile) {
                true => adjacent_flipped_count == 1 || adjacent_flipped_count == 2,
                false => adjacent_flipped_count == 2
            }
        })
        .map(|&t| t)
        .collect()
}

fn main() {
    // Part One
    let tiles_to_flip: Vec<Tile> = parse_file("input.txt")
        .iter()
        .map(|line| final_coordinates(line))
        .collect();

    let flipped_tiles = derive_flipped_tiles(&tiles_to_flip);
    println!("Part one: {}", flipped_tiles.len());

    // Part Two
    let mut state = flipped_tiles;
    (0..100).for_each(|_| state = build_next_state(&state));
    println!("Part two: {}", state.len());
}

#[test]
fn test_both_parts() {
    let tiles_to_flip: Vec<Tile> = parse_file("example1.txt")
        .iter()
        .map(|line| final_coordinates(line))
        .collect();

    let flipped_tiles = derive_flipped_tiles(&tiles_to_flip);
    assert_eq!(flipped_tiles.len(), 10);

    // Part 2
    let day_1 = build_next_state(&flipped_tiles);
    assert_eq!(day_1.len(), 15);

    let day_2 = build_next_state(&day_1);
    assert_eq!(day_2.len(), 12);

    let mut state = day_2;
    (0..98).for_each(|_| state = build_next_state(&state));
    assert_eq!(state.len(), 2208);
}
