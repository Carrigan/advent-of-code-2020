#![allow(dead_code, unused_imports)]
#[cfg(test)]
mod tests;
mod tile;

use tile::Tile;

fn index_rotated_grid(x: usize, y: usize, width: usize, height: usize, rotation: usize, flipped: bool) -> (usize, usize) {
    let height_index = height - 1;
    let width_index = width - 1;

    match flipped {
        false => {
            match rotation {
                0 => (x, y),
                1 => (height_index - y, x),
                2 => (width_index - x, height_index - y),
                _ => (y, width_index - x),
            }
        }
        true => {
            match rotation {
                0 => (width_index - x, y),
                1 => (y, x),
                2 => (x, height_index - y),
                _ => (height_index - y, width_index - x),
            }
        }
    }
}

fn parse_input(path: &str) -> Vec<Tile> {
    std::fs::read_to_string(path).unwrap()
        .split("\n\n")
        .map(|section| Tile::from(section))
        .collect()
}

#[derive(Debug)]
struct PlacedTile {
    tile_index: usize,
    x: i32,
    y: i32,
    rotation: usize,
    flipped: bool
}

impl PlacedTile {
    fn placed_side(&self, tiles: &[Tile], index: usize) -> u32 {
        tiles[self.tile_index].side_with_translations(index, self.rotation, self.flipped)
    }
}

fn available_edges(placements: &[PlacedTile], tiles: &[Tile], placement_index: usize) -> Vec<(usize, usize, u32)> {
    let px = placements[placement_index].x;
    let py = placements[placement_index].y;
    let side_adjacency = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    side_adjacency.iter()
        .enumerate()
        .filter(|(_, &(x, y))|
            placements.iter().find(|p| p.x == (px + x) && p.y == (py + y)).is_none()
        )
        .map(|(side, _)|
            (placement_index, side, placements[placement_index].placed_side(tiles, side))
        )
        .collect()
}

fn xor(cond1: bool, cond2: bool) -> bool {
    (cond1 || cond2) && !(cond1 && cond2)
}

struct Puzzle(Vec<PlacedTile>);

impl Puzzle {
    fn new() -> Self {
        Puzzle(Vec::new())
    }

    fn solve(&mut self, tiles: &[Tile]) {
        let mut all_placed = false;
        let mut last_placed = 0;

        self.place(tiles, 8);

        while !all_placed {
            println!("\n\nPlacement iteration...");

            let mut currently_placed = 0;
            all_placed = true;

            for i in 0..tiles.len() {
                match self.place(tiles, i) {
                    true => currently_placed += 1,
                    false => all_placed = false
                }
            }

            last_placed = match last_placed == currently_placed {
                true => break,
                false => currently_placed
            };
        }

        // Normalize the grid to 0, 0
        let x_min = self.0.iter().map(|p| p.x).min().unwrap();
        let y_min = self.0.iter().map(|p| p.y).min().unwrap();
        for placed in self.0.iter_mut() {
            placed.x -= x_min;
            placed.y -= y_min;
        }
    }

    fn print(&self, tiles: &[Tile], rotation: usize, flipped: bool)  {
        let x_max = self.0.iter().map(|p| p.x).max().unwrap();
        let y_max = self.0.iter().map(|p| p.y).max().unwrap();

        println!("Status: ");
        for y in 0..=y_max {
            for x in 0..=x_max {
                let (shifted_x, shifted_y) = index_rotated_grid(x as usize, y as usize, x_max as usize + 1, y_max as usize + 1, rotation, flipped);
                print!("{} ", match self.0.iter().find(|p| p.x as usize == shifted_x && p.y as usize == shifted_y) {
                    Some(placement) => format!("|{:04} {} {}",
                        tiles[placement.tile_index].label,
                        placement.rotation,
                        if placement.flipped { "x" } else { "." }
                    ),
                    None => String::from("|none 0 f")
                });
            }

            println!("");
        }
    }

    fn tile_index_at(&self, x: i32, y: i32) -> usize {
        self.0.iter().find(|p| p.x == x && p.y == y).unwrap().tile_index
    }

    fn placement_at(&self, x: i32, y: i32) -> usize {
        self.0.iter().position(|p| p.x == x && p.y == y).unwrap()
    }

    fn corner_labels(&self, tiles: &[Tile]) -> [u64; 4] {
        let x_max = self.0.iter().map(|p| p.x).max().unwrap();
        let y_max = self.0.iter().map(|p| p.y).max().unwrap();

        [
            tiles[self.tile_index_at(0, 0)].label as u64,
            tiles[self.tile_index_at(x_max, 0)].label as u64,
            tiles[self.tile_index_at(x_max, y_max)].label as u64,
            tiles[self.tile_index_at(0, y_max)].label as u64
        ]
    }

    fn place(&mut self, tiles: &[Tile], index: usize) -> bool {
        match self.0.iter().find(|t| t.tile_index == index) {
            Some(p) => {
                println!("- Already placed: {} {:?}", index, p);
                return true;
            },
            None => ()

        }

        println!("\nAttempting placement of piece {} ({})", index, tiles[index].label);
        println!("- {:?}", tiles[index].sides);

        // Place the first piece at the origin with no orientation
        if self.0.is_empty() {
            println!("- Placing initial piece at 0, 0, 0, false");
            self.0.push(PlacedTile { tile_index: index, x: 0, y: 0, rotation: 0, flipped: false });
            return true;
        }

        // Generate a mating edge list in form (PlacedTile index, side index, value)
        let possible_edges: Vec<(usize, usize, u32)> = (0..(self.0.len()))
            .map(|i| available_edges(&self.0, tiles, i))
            .flatten()
            .collect();

        println!("- Possible mates: {:?}", possible_edges);

        // See if any mate is possible
        let tile = &tiles[index];

        // The mate will be of the form (PlacedTile index, side_index, mate_side, mate_flipped)
        let mate = possible_edges.iter().find_map(|&(placement_index, side_index, side_value)|
            tile.mates(side_value).map(|(rotation, flipped)| (placement_index, side_index, rotation, flipped, side_value))
        );

        // If no mate is possible, return false
        let mate = match mate {
            Some(m) => m,
            None => {
                println!("- No mate found.");
                return false;
            }
        };

        println!(
            "- Mate found: [ptidx: {}, ptside: {}, mateside: {}, flipped: {}, value: {}, mate_value: {}]",
            mate.0, mate.1, mate.2, mate.3, mate.4, tile::invert_side(10, mate.4));

        // Orient the piece accordingly
        let side_adjacency = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mate_tile = &self.0[mate.0];
        let x = mate_tile.x + side_adjacency[mate.1].0;
        let y = mate_tile.y + side_adjacency[mate.1].1;
        let rotation = derive_rotation(mate.1, mate.2, mate.3);

        // Place the piece
        let placement = PlacedTile { tile_index: index, x, y, rotation, flipped: mate.3 };

        println!("- {}: {:?}", self.0.len(), placement);
        self.0.push(placement);

        true
    }

    fn width(&self) -> usize {
        self.0.iter().map(|p| p.x).max().unwrap() as usize + 1
    }

    fn height(&self) -> usize {
        self.0.iter().map(|p| p.y).max().unwrap() as usize + 1
    }

    fn iter_sea_monster_windows<'a>(&'a self, tiles: &'a [Tile], rotation: usize, flipped: bool) -> SeaMonsterWindowIterator {
        let rendered = self.render(tiles, rotation, flipped);

        SeaMonsterWindowIterator {
            rendered, index: 0, width: self.width(), height: self.height()
        }
    }

    fn print_entirety(&self, tiles: &[Tile], rotation: usize, flipped: bool) {
        let rendered = self.render(tiles, rotation, flipped);

        for y in 0..(self.width() * 8) {
            for x in 0..(self.width() * 8) {
                let index = (y * (self.width() * 8)) + x;
                print!("{}{}{}",
                    if y % 8 == 0 && x == 0 { "\n" } else { "" },
                    if x % 8 == 0 { " " } else { "" },
                    if rendered[index] { "#" } else { "." }
                );
            }

            println!(" ");
        }
    }

    fn render(&self, tiles: &[Tile], rotation: usize, flipped: bool) -> Vec<bool> {
        let unrotated: Vec<bool> = (0..(self.width() * self.height() * 64))
            .map(|index| {
                let x = index % (self.width() * 8);
                let y = index / (self.width() * 8);

                // Get the relative piece
                let x_panel = x / 8;
                let y_panel = y / 8;

                // Get the index within there
                let x_in_panel = x % 8;
                let y_in_panel = y % 8;

                // Retrieve it
                let placement = self.placement_at(x_panel as i32, y_panel as i32);
                let placed_tile = &self.0[placement];
                let tile = &tiles[placed_tile.tile_index];

                // Correct rotation
                let flip_correction = !placed_tile.flipped;
                let directed_rotation: i32 = if placed_tile.flipped { 2 } else { -2 };

                let rotation_correction =
                    if placed_tile.flipped && (placed_tile.rotation % 2 == 1) {
                        placed_tile.rotation
                    } else {
                        (4 + directed_rotation + placed_tile.rotation as i32) as usize % 4
                    };

                tile.index(x_in_panel, y_in_panel, rotation_correction, flip_correction)
            }).collect();

        (0..(self.width() * self.height() * 64)).map(|index| {
            let (x, y) = (index % (self.width() * 8), index / (self.width() * 8));
            let (ind_x, ind_y) = index_rotated_grid(x, y, self.width() * 8, self.width() * 8, rotation, flipped);
            let transformed_index = ind_y * (self.width() * 8) + ind_x;

            unrotated[transformed_index]
        }).collect()
    }

    fn find_sea_monsters(&self, tiles: &[Tile]) -> (usize, usize, bool) {
        let orientations = [
            (false, 0), (false, 1), (false, 2), (false, 3),
            (true, 0), (true, 1), (true, 2), (true, 3)
        ];

        let options = orientations.iter()
            .map(|(flipped, rotation)| {
                let count = self.iter_sea_monster_windows(tiles, *rotation, *flipped)
                    .filter(|(_, _, window)| is_sea_monster(window))
                    .map(|(x, y, window)| {
                        println!("Monster at: {}, {}", x, y);
                        (x, y, window)
                    })
                    .count();

                (*flipped, *rotation, count)
            })
            .collect::<Vec<(bool, usize, usize)>>();

        println!("{:?}", options);

        options
            .iter()
            .find(|(_, _, count)| *count > 0)
            .map(|(flipped, rotation, count)| (*count, *rotation, *flipped))
            .unwrap()
    }
}

struct SeaMonsterWindowIterator {
    rendered: Vec<bool>,
    index: usize,
    width: usize,
    height: usize
}

impl Iterator for SeaMonsterWindowIterator {
    type Item = (usize, usize, Vec<bool>);

    fn next(&mut self) -> Option<Self::Item> {
        let usable_width = (self.width * 8) - 19;
        let usable_height = (self.height * 8) - 2;

        if self.index == usable_height * usable_width {
            return None;
        }

        // Figure out where it is in the grid
        let start_x = self.index % usable_width;
        let start_y = self.index / usable_width;

        // Iterate and build
        let out = (0..60)
            .map(|index| (start_x + (index % 20), start_y + (index / 20)))
            .map(|(x, y)| {
                let index = (y * self.width * 8) + x;
                self.rendered[index]
            })
            .collect();

        // Iterate and return
        self.index += 1;
        Some((start_x, start_y, out))
    }
}

// 0 2 4 6 8101214161820
//                   # |
// #    ##    ##    ###|
//  #  #  #  #  #  #   |
fn is_sea_monster(window: &Vec<bool>) -> bool {
    let positives = [
        (18, 0),
        (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
        (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)
    ];

    positives
        .iter()
        .map(|(x, y)| y * 20 + x)
        .all(|idx| window[idx])
}

fn derive_rotation(placed_side: usize, mate_side: usize, flipped: bool) -> usize {
    let mate_side_end_position = (placed_side + 2) % 4;
    let flipped_mate_side = if flipped && (mate_side == 1 || mate_side == 3) { (mate_side + 2) % 4 } else { mate_side };
    ((4 + flipped_mate_side) - mate_side_end_position) % 4
}

fn main() {
    // Part One
    let tiles = parse_input("input.txt");
    let mut puzzle = Puzzle::new();

    puzzle.solve(&tiles);
    let corner_product: u64 = puzzle.corner_labels(&tiles).iter().product();
    println!("Part one: {}", corner_product);

    // Part two
    puzzle.print(&tiles, 2, false);
    puzzle.print_entirety(&tiles, 2, false);


    let total_hash_count = tiles.iter().map(|t| t.trues()).sum::<usize>();
    let (sea_monsters, _, _) = puzzle.find_sea_monsters(&tiles);
    println!("Part two: {}, {}, {}", total_hash_count, sea_monsters, (total_hash_count - (15 * sea_monsters)));
}
