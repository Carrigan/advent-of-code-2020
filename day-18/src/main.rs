
#[derive(Debug)]
enum Token {
    Number(u64),
    MultiplicationSymbol,
    AdditionSymbol,
    ExpressionStart,
    ExpressionEnd
}

fn tokenize(input: &str) -> Vec<Token> {
    input.chars().filter(|c| !c.is_whitespace()).map(|c| match c {
        '(' => Token::ExpressionStart,
        ')' => Token::ExpressionEnd,
        '+' => Token::AdditionSymbol,
        '*' => Token::MultiplicationSymbol,
        n => Token::Number(n.to_digit(10).expect("could not tokenize number") as u64)
    }).collect()
}

fn apply(accumulator: u64, operand: u64, operator: Option<Token>) -> u64 {
    match operator {
        Some(Token::MultiplicationSymbol) => accumulator * operand,
        Some(Token::AdditionSymbol) => accumulator + operand,
        _ => panic!()
    }
}

fn evaluate_ltr(tokens: &[Token]) -> (usize, u64) {


    let mut total = 0;
    let mut index = 0;
    let mut context = Some(Token::AdditionSymbol);

    while index < tokens.len() {
        let token = &tokens[index];

        match token {
            Token::AdditionSymbol => context = Some(Token::AdditionSymbol),
            Token::MultiplicationSymbol => context = Some(Token::MultiplicationSymbol),
            Token::Number(n) => {
                total = apply(total, *n, context);
                context = None;
            }
            Token::ExpressionStart => {
                let (increment, n) = evaluate_ltr(&tokens[index + 1..]);
                total = apply(total, n, context);
                context = None;
                index += increment;
            }
            Token::ExpressionEnd => return (index + 1, total)
        }

        index += 1;
    }

    (index, total)
}


fn evaluate_with_precedence(tokens: &[Token]) -> (usize, u64) {
    let mut total = 0;
    let mut index = 0;
    let mut context = Some(Token::AdditionSymbol);

    while index < tokens.len() {
        let token = &tokens[index];

        match token {
            Token::AdditionSymbol => context = Some(Token::AdditionSymbol),
            Token::MultiplicationSymbol => {
                let (increment, n) = evaluate_with_precedence(&tokens[index + 1..]);
                return (index + increment + 1, total * n);
            }
            Token::Number(n) => {
                total = apply(total, *n, context);
                context = None;
            }
            Token::ExpressionStart => {
                let (increment, n) = evaluate_with_precedence(&tokens[index + 1..]);
                total = apply(total, n, context);
                context = None;
                index += increment;
            }
            Token::ExpressionEnd => return (index + 1, total)
        }

        index += 1;
    }

    (index, total)
}

fn main() {
    // Part One
    let sum: u64 = std::fs::read_to_string("input.txt").unwrap().lines()
        .map(|line| {
            let tokens = tokenize(line);
            evaluate_ltr(&tokens).1
        })
        .sum();

    println!("Part one: {}", sum);

    // Part Two
    let sum: u64 = std::fs::read_to_string("input.txt").unwrap().lines()
        .map(|line| {
            let tokens = tokenize(line);
            evaluate_with_precedence(&tokens).1
        })
        .sum();

    println!("Part two: {}", sum);
}

#[test]
fn test_evaluate_ltr() {
    let tokens = tokenize("2 * 3 + (4 * 5)");
    assert_eq!(evaluate_ltr(&tokens).1, 26);

    let tokens = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(evaluate_ltr(&tokens).1, 437);

    let tokens = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(evaluate_ltr(&tokens).1, 12240);

    println!("last one");
    let tokens = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(evaluate_ltr(&tokens).1, 13632);
}

#[test]
fn test_evaluate_with_predence() {
    let tokens = tokenize("2 * 3 + (4 * 5)");
    assert_eq!(evaluate_with_precedence(&tokens).1, 46);

    let tokens = tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(evaluate_with_precedence(&tokens).1, 1445);

    let tokens = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(evaluate_with_precedence(&tokens).1, 669060);

    let tokens = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(evaluate_with_precedence(&tokens).1, 23340);
}
