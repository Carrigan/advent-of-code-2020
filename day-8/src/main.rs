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

#[derive(Debug, Eq, PartialEq)]
enum StepResult {
    Ok,
    EndOfProgram,
    AlreadyVisited,
    OutOfBounds
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

    fn reset(&mut self) {
        self.executed = Vec::new();
        self.acc = 0;
        self.pc = 0;
    }

    fn run(&mut self) -> Result<(), StepResult> {
        self.reset();

        loop {
            match self.step() {
                StepResult::Ok => (),
                StepResult::EndOfProgram => return Ok(()),
                StepResult::AlreadyVisited => return Err(StepResult::AlreadyVisited),
                StepResult::OutOfBounds => return Err(StepResult::OutOfBounds)
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

        if self.pc > self.instructions.len() {
            return StepResult::OutOfBounds;
        }

        if self.pc >= self.instructions.len() {
            return StepResult::EndOfProgram;
        }

        StepResult::Ok
    }

    fn attempt_correction(&mut self) -> bool {
        for index in 0..self.instructions.len() {
            let (original_instruction, mutated_instruction) = match self.instructions[index] {
                Instruction::Jmp(x) => (Instruction::Jmp(x), Instruction::Nop(x)),
                Instruction::Nop(x) => (Instruction::Nop(x), Instruction::Jmp(x)), 
                Instruction::Acc(_) => continue
            };
    
            // Perform the swap
            self.instructions[index] = mutated_instruction;
    
            // Run the program
            match self.run() {
                Ok(_) => return true,
                Err(_) => self.instructions[index] = original_instruction
            }
        }

        false
    }
}

fn main() {
    // Part one
    let mut program = Program::new("input.txt");
    let _result = program.run();
    println!("Part 1: {}", program.acc);

    // Part two
    assert!(program.attempt_correction());
    println!("Part 2: {}", program.acc);
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
    let result = program.run();

    assert_eq!(result.err().unwrap(), StepResult::AlreadyVisited);
    assert_eq!(program.acc, 5);
}

#[test]
fn test_example_2() {
    let mut program = Program::new("example1.txt");
    let corrected = program.attempt_correction();

    assert_eq!(corrected, true);
    assert_eq!(program.acc, 8);
}
