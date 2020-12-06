struct GroupAnswers {
    first_consumed: bool,
    answers: [bool; 26]
}

impl GroupAnswers {
    fn new() -> Self {
        GroupAnswers { answers: [false; 26], first_consumed: true }
    }

    fn consume_line_or(&mut self, line: &str) {
        line.chars().for_each(|letter| self.mark(letter));
    }

    fn consume_line_and(&mut self, line: &str) {
        let mut current_answers = GroupAnswers::new();
        current_answers.consume_line_or(line);
        current_answers.answers.iter()
            .enumerate()
            .for_each(|(index, current_value)| {
                self.answers[index] = match self.first_consumed {
                    true => *current_value,
                    false => self.answers[index] && *current_value
                };
            });

        self.first_consumed = false;
    }

    fn mark(&mut self, letter: char) {
        self.answers[(letter as usize) - 97] = true;
    }

    fn count(&self) -> usize {
        self.answers.iter().filter(|x| **x).count()
    }
}

enum SummationType {
    And,
    Or
}

fn summed_answer_count(path: &str, operation: SummationType) -> usize {
    std::fs::read_to_string(path)
        .expect("could not open file")
        .lines()
        .fold(vec![GroupAnswers::new()], |mut answers: Vec<GroupAnswers>, line| {
            if line.is_empty() {
                answers.push(GroupAnswers::new());
            } else {
                match operation {
                    SummationType::Or => answers.last_mut().unwrap().consume_line_or(line),
                    SummationType::And => answers.last_mut().unwrap().consume_line_and(line)
                };
            }

            answers
        })
        .iter()
        .map(|ga| ga.count())
        .sum()
}

fn main() {
    // Part 1
    println!("Part 1: {}", summed_answer_count("input.txt", SummationType::Or));

    // Part 2
    println!("Part 2: {}", summed_answer_count("input.txt", SummationType::And));
}

#[test]
fn test_individuals() {
    let mut ga = GroupAnswers::new();
    ga.consume_line_or("abcx");
    ga.consume_line_or("abcy");
    ga.consume_line_or("abcz");
    
    assert_eq!(ga.count(), 6);
}

#[test]
fn test_file_part_1() {
    assert_eq!(summed_answer_count("example.txt", SummationType::Or), 11);
}

#[test]
fn test_file_part_2() {
    assert_eq!(summed_answer_count("example.txt", SummationType::And), 6);
}
