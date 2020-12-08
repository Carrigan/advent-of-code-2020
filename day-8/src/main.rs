#[derive(Eq, PartialEq, Debug)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32)
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let operand = line[4..].parse().unwrap();

        match &line[0..3] {
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            _ => Instruction::Nop(operand)
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    executed: Vec<usize>,
    acc: i32,
    pc: usize
}

enum StepResult {
    Ok,
    EndOfProgram,
    AlreadyVisited
}

impl Program {
    fn new(path: &str) -> Self {
        let instructions = std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|l| Instruction::from(l))
            .collect();

        Program { instructions, executed: Vec::new(), acc: 0, pc: 0 }
    }

    fn run(&mut self) {
        loop {
            match self.step() {
                StepResult::Ok => (),
                StepResult::EndOfProgram => panic!(),
                StepResult::AlreadyVisited => break
            }
        }
    }

    fn step(&mut self) -> StepResult {
        if self.executed.contains(&self.pc) {
            return StepResult::AlreadyVisited;
        }

        self.executed.push(self.pc);
        self.pc = match self.instructions[self.pc] {
            Instruction::Nop(_) => self.pc + 1,
            Instruction::Acc(x) => { self.acc += x; self.pc + 1 },
            Instruction::Jmp(x) => ((self.pc as i32) + x) as usize
        };

        if self.pc >= self.instructions.len() {
            return StepResult::EndOfProgram;
        }

        StepResult::Ok
    }
}

fn main() {
    // Part one
    let mut program = Program::new("input.txt");
    program.run();
    println!("Part 1: {}", program.acc);
}

#[test]
fn test_instruction_parser() {
    assert_eq!(Instruction::from("jmp +0"), Instruction::Jmp(0));
    assert_eq!(Instruction::from("acc +1"), Instruction::Acc(1));
    assert_eq!(Instruction::from("nop -40"), Instruction::Nop(-40));
}

#[test]
fn test_example_1() {
    let mut program = Program::new("example1.txt");
    program.run();
    assert_eq!(program.acc, 5);
}
