use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn parse_input(path: &str) -> (Vec<usize>, Vec<usize>) {
    let file = std::fs::read_to_string(path).unwrap();

    let mut decks = file
        .split("\n\n")
        .map(|chunk|
            chunk.lines().skip(1).map(|l| l.parse().unwrap()).collect()
        );

    (decks.next().unwrap(), decks.next().unwrap())
}

fn score_deck(deck: &Vec<usize>) -> usize {
    let deck_size = deck.len();

    deck
        .iter()
        .enumerate()
        .map(|(idx, value)| (deck_size - idx) * value)
        .sum()
}

fn play_simple_round(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) {
    let card_1 = deck1.remove(0);
    let card_2 = deck2.remove(0);

    if card_1 > card_2 {
        deck1.push(card_1);
        deck1.push(card_2);
    } else if card_2 > card_1 {
        deck2.push(card_2);
        deck2.push(card_1);
    } else {
        panic!();
    }
}

fn play_simple_game(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        play_simple_round(deck1, deck2);
    }

    if deck1.is_empty() {
        return 2;
    } else {
        return 1;
    }
}

fn play_recursive_round(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) {
    let card_1 = deck1.remove(0);
    let card_2 = deck2.remove(0);

    // Subgame
    let winner = if card_1 <= deck1.len() && card_2 <= deck2.len() {
        let mut minigame_deck_1: Vec<usize> = deck1[0..card_1].iter().cloned().collect();
        let mut minigame_deck_2: Vec<usize> = deck2[0..card_2].iter().cloned().collect();

        play_recursive_game(&mut minigame_deck_1, &mut minigame_deck_2)
    } else {
        if card_1 > card_2 { 1 } else { 2 }
    };

    if winner == 1 {
        deck1.push(card_1);
        deck1.push(card_2);
    } else {
        deck2.push(card_2);
        deck2.push(card_1);
    }
}

fn game_state_hash(d1: &Vec<usize>, d2: &Vec<usize>) -> (u64, u64) {
    let mut h = DefaultHasher::new();
    d1.hash(&mut h);
    let d1_hash = h.finish();

    let mut h = DefaultHasher::new();
    d2.hash(&mut h);
    let d2_hash = h.finish();

    (d1_hash, d2_hash)
}

fn play_recursive_game(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) -> usize {
    let mut deck_states: Vec<(u64, u64)> = Vec::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        let current_game_state = game_state_hash(&deck1, &deck2);

        if deck_states.iter().any(|ds| ds == &current_game_state) {
            return 1;
        }

        deck_states.push(current_game_state);

        play_recursive_round(deck1, deck2);
    }

    if deck1.is_empty() {
        return 2;
    } else {
        return 1;
    }
}

fn main() {
    // Part one
    let (mut deck1, mut deck2) = parse_input("input.txt");
    let winning_deck = match play_simple_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    println!("{:?}", score_deck(&winning_deck));

    // Part two
    let (mut deck1, mut deck2) = parse_input("input.txt");
    let winning_deck = match play_recursive_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    println!("{:?}", score_deck(&winning_deck));
}

#[test]
fn test_part_one() {
    let (mut deck1, mut deck2) = parse_input("example.txt");
    let winning_deck = match play_simple_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    assert_eq!(score_deck(&winning_deck), 306);
}

#[test]
fn test_part_two_recursive() {
    let (mut deck1, mut deck2) = parse_input("example2.txt");
    play_recursive_game(&mut deck1, &mut deck2);
}


#[test]
fn test_part_two() {
    let (mut deck1, mut deck2) = parse_input("example.txt");
    let winning_deck = match play_recursive_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    assert_eq!(score_deck(&winning_deck), 291);
}
