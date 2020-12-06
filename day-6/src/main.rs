struct GroupAnswers {
    answers: [bool; 26]
}

impl GroupAnswers {
    fn new() -> Self {
        GroupAnswers { answers: [false; 26] }
    }

    fn consume_line(&mut self, line: &str) {
        line.chars().for_each(|letter| self.mark(letter));
    }

    fn mark(&mut self, letter: char) {
        self.answers[(letter as usize) - 97] = true;
    }

    fn count(&self) -> usize {
        self.answers.iter().filter(|x| **x).count()
    }
}

fn summed_answer_count(path: &str) -> usize {
    std::fs::read_to_string(path)
        .expect("could not open file")
        .lines()
        .fold(vec![GroupAnswers::new()], |mut answers: Vec<GroupAnswers>, line| {
            if line.is_empty() {
                answers.push(GroupAnswers::new());
            } else {
                answers.last_mut().unwrap().consume_line(line);
            }

            answers
        })
        .iter()
        .map(|ga| ga.count())
        .sum()
}

fn main() {
    // Part 1
    println!("Part 1: {}", summed_answer_count("input.txt"));
}

#[test]
fn test_individuals() {
    let mut ga = GroupAnswers::new();
    ga.consume_line("abcx");
    ga.consume_line("abcy");
    ga.consume_line("abcz");
    
    assert_eq!(ga.count(), 6);
}

#[test]
fn test_file() {
    assert_eq!(summed_answer_count("example.txt"), 11);
}
