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

    // This should match the advent example
    puzzle.print_entirety(&tiles, Orientation { rotation: Rotation::RightSideUp, flipped: false });

    let (monster_count, _) = puzzle.find_sea_monsters(&tiles);
    let roughness = tiles.iter().map(|t| t.trues()).sum::<usize>();
    assert_eq!(roughness - (monster_count * 15), 273);
}

#[test]
fn test_side_translations() {
    let tile = Tile { width: 10, sides: [376, 156, 28, 80], label: 0, data: Vec::new() };
    let orientation = Orientation { rotation: Rotation::RotatedOnceCounterClockwise, flipped: true };

    assert_eq!(tile.side_with_translations(0, orientation), 40);
    assert_eq!(tile.side_with_translations(1, orientation), 224);
    assert_eq!(tile.side_with_translations(2, orientation), 228);
    assert_eq!(tile.side_with_translations(3, orientation), 122);
}

#[test]
fn test_rotation() {
    let tile = &parse_input("example2.txt")[0];

    let orientation = Orientation { rotation: Rotation::RightSideUp, flipped: false };
    tile.show(orientation);
    assert!(tile.index(7, 1, orientation));

    let orientation = Orientation { rotation: Rotation::RotatedOnceClockwise, flipped: false };
    tile.show(orientation);
    assert!(tile.index(6, 7, orientation));

    let orientation = Orientation { rotation: Rotation::UpsideDown, flipped: false };
    tile.show(orientation);
    assert!(tile.index(0, 6, orientation));

    let orientation = Orientation { rotation: Rotation::RotatedOnceCounterClockwise, flipped: false };
    tile.show(orientation);
    assert!(tile.index(1, 0, orientation));

    let orientation = Orientation { rotation: Rotation::RightSideUp, flipped: true };
    tile.show(orientation);
    assert!(tile.index(0, 1, orientation));

    let orientation = Orientation { rotation: Rotation::RotatedOnceClockwise, flipped: true };
    tile.show(orientation);
    assert!(tile.index(6, 0, orientation));

    let orientation = Orientation { rotation: Rotation::UpsideDown, flipped: true };
    tile.show(orientation);
    assert!(tile.index(7, 6, orientation));

    let orientation = Orientation { rotation: Rotation::RotatedOnceCounterClockwise, flipped: true };
    tile.show(orientation);
    assert!(tile.index(1, 7, orientation));
}

#[test]
fn test_has_unique_edges() {
    let tiles = parse_input("input.txt");
    let mut side_count = HashMap::new();

    tiles.iter().for_each(|t| {
        let sides = t.sides;
        let inverted_sides = [
            tile::invert_side(10, sides[0]),
            tile::invert_side(10, sides[1]),
            tile::invert_side(10, sides[2]),
            tile::invert_side(10, sides[3])
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
