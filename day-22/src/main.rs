fn parse_input(path: &str) -> (Vec<usize>, Vec<usize>) {
    let file = std::fs::read_to_string(path).unwrap();

    let mut decks = file
        .split("\n\n")
        .map(|chunk|
            chunk.lines().skip(1).map(|l| l.parse().unwrap()).collect()
        );

    (decks.next().unwrap(), decks.next().unwrap())
}

fn play_round(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) {
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

fn play_game(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        play_round(deck1, deck2);
    }

    if deck1.is_empty() {
        return 2;
    } else {
        return 1;
    }
}

fn score_deck(deck: &Vec<usize>) -> usize {
    let deck_size = deck.len();

    deck
        .iter()
        .enumerate()
        .map(|(idx, value)| (deck_size - idx) * value)
        .sum()
}

fn main() {
    let (mut deck1, mut deck2) = parse_input("input.txt");
    let winning_deck = match play_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    println!("{:?}", score_deck(&winning_deck));
}

#[test]
fn test_part_one() {
    let (mut deck1, mut deck2) = parse_input("example.txt");
    let winning_deck = match play_game(&mut deck1, &mut deck2) {
        1 => deck1,
        _ => deck2
    };

    assert_eq!(score_deck(&winning_deck), 306);
}
