use std::collections::HashMap;
use super::*;

#[test]
fn test_part_one() {
    let tiles = parse_input("example1.txt");
    let mut puzzle = Puzzle::new();

    puzzle.solve(&tiles);

    let corner_product: u64 = puzzle.corner_labels(&tiles).iter().product();
    assert_eq!(corner_product, 20899048083289);
}

#[test]
fn test_part_two() {
    let tiles = parse_input("example1.txt");
    let mut puzzle = Puzzle::new();
    puzzle.solve(&tiles);

    puzzle.print_entirety(&tiles, 3, false);
    println!("{:?}", tiles.iter().map(|t| t.trues()).sum::<usize>());
    println!("{:?}. {}", puzzle.find_sea_monsters(&tiles), 303 - 30);
}

#[test]
fn test_rotation() {
    let tile = Tile { width: 10, sides: [376, 156, 28, 80], label: 0, data: Vec::new() };

    assert_eq!(tile.side_with_translations(0, 1, true), 40);
    assert_eq!(tile.side_with_translations(1, 1, true), 224);
    assert_eq!(tile.side_with_translations(2, 1, true), 228);
    assert_eq!(tile.side_with_translations(3, 1, true), 122);

    let tiles = parse_input("example2.txt");

    tiles[0].show(0, false);
    assert!(tiles[0].index(7, 1, 0, false));

    tiles[0].show(1, false);
    assert!(tiles[0].index(1, 0, 1, false));

    tiles[0].show(2, false);
    assert!(tiles[0].index(0, 6, 2, false));

    tiles[0].show(3, false);
    assert!(tiles[0].index(6, 7, 3, false));

    tiles[0].show(0, true);
    assert!(tiles[0].index(0, 1, 0, true));

    tiles[0].show(1, true);
    assert!(tiles[0].index(1, 7, 1, true));

    tiles[0].show(2, true);
    assert!(tiles[0].index(7, 6, 2, true));

    tiles[0].show(3, true);
    assert!(tiles[0].index(6, 0, 3, true));
}

#[test]
fn test_has_unique_edges() {
    let tiles = parse_input("input.txt");
    let mut side_count = HashMap::new();

    tiles.iter().for_each(|t| {
        let sides = t.sides;
        let inverted_sides = [
            invert_side(10, sides[0]),
            invert_side(10, sides[1]),
            invert_side(10, sides[2]),
            invert_side(10, sides[3])
        ];

        for side in sides.iter().chain(inverted_sides.iter()) {
            *side_count.entry(*side).or_insert(0) += 1;
        }
    });

    assert!(side_count.values().all(|&v| v <= 2));
    println!("{:?}", side_count);
}

#[test]
fn test_sea_monster_finding() {
    let test = "                  # #    ##    ##    ### #  #  #  #  #  #   ";
    let data: Vec<bool> = test.chars().map(|c| c == '#').collect();
    assert!(is_sea_monster(&data));

    let test = "                    #    ##    ##    ### #  #  #  #  #  #   ";
    let data: Vec<bool> = test.chars().map(|c| c == '#').collect();
    assert_eq!(is_sea_monster(&data), false);

    let test = "############################################################";
    let data: Vec<bool> = test.chars().map(|c| c == '#').collect();
    assert!(is_sea_monster(&data));
}
