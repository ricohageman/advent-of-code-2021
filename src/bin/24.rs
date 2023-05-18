use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Placeholder {
    Variable(Variable),
    Number(isize),
    ParsedInput(isize),
    Input,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub variable: Variable,
    pub placeholder: Placeholder,
}

impl Placeholder {
    pub fn new(data: &str) -> Self {
        match data {
            "w" => Placeholder::Variable(Variable::W),
            "x" => Placeholder::Variable(Variable::X),
            "y" => Placeholder::Variable(Variable::Y),
            "z" => Placeholder::Variable(Variable::Z),
            _ => Placeholder::Number(data.parse().unwrap()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut data = line.split(" ");
            let instruction_type = match data.next().unwrap() {
                "inp" => InstructionType::Inp,
                "add" => InstructionType::Add,
                "mul" => InstructionType::Mul,
                "div" => InstructionType::Div,
                "mod" => InstructionType::Mod,
                "eql" => InstructionType::Eql,
                _ => panic!("{line}"),
            };

            let variable = match Placeholder::new(data.next().unwrap()) {
                Placeholder::Variable(variable) => variable,
                _ => panic!("{line}"),
            };

            let placeholder = match data.next() {
                None => Placeholder::Input,
                Some(data) => Placeholder::new(data),
            };

            Instruction {
                instruction_type,
                variable,
                placeholder,
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticLogicUnitInput {
    input: Vec<isize>,
    index: usize,
}

impl ArithmeticLogicUnitInput {
    pub fn new(input: Vec<isize>) -> Self {
        Self { input, index: 0 }
    }

    pub fn next(&mut self) -> isize {
        let result = self.input[self.index];
        self.index += 1;
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArithmeticLogicUnit {
    registers: [isize; 4],
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        Self {
            registers: [0, 0, 0, 0],
        }
    }

    fn variable_index(&self, variable: Variable) -> usize {
        match variable {
            Variable::W => 0,
            Variable::X => 1,
            Variable::Y => 2,
            Variable::Z => 3,
        }
    }

    pub fn get_variable_value(&self, variable: Variable) -> isize {
        self.registers[self.variable_index(variable)]
    }

    fn set_variable_value(&mut self, variable: Variable, value: isize) {
        self.registers[self.variable_index(variable)] = value;
    }

    fn get_placeholder_value(&mut self, placeholder: Placeholder) -> isize {
        match placeholder {
            Placeholder::Variable(variable) => self.get_variable_value(variable),
            Placeholder::Number(number) => number,
            Placeholder::ParsedInput(number) => number,
            Placeholder::Input => panic!(),
        }
    }

    pub fn apply_all(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.apply(instruction);
        }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        let variable = instruction.variable;
        let variable_value = self.get_variable_value(variable);
        let value = self.get_placeholder_value(instruction.placeholder);

        let result = match instruction.instruction_type {
            InstructionType::Inp => value,
            InstructionType::Add => variable_value + value,
            InstructionType::Mul => variable_value * value,
            InstructionType::Div => {
                assert_ne!(value, 0);
                variable_value / value
            }
            InstructionType::Mod => {
                assert!(value > 0);
                assert!(variable_value >= 0);
                variable_value % value
            }
            InstructionType::Eql => (variable_value == value) as isize,
        };

        self.set_variable_value(variable, result);
    }
}

fn construct_model_number(
    alu: ArithmeticLogicUnit,
    instructions: &[Instruction],
    instruction_index: usize,
    input: [isize; 14],
    input_index: usize,
    possible_numbers: &[isize],
    infeasible_states: &mut HashSet<(usize, isize)>,
) -> Option<usize> {
    if input_index == 14 {
        return match alu.get_variable_value(Variable::Z) {
            0 => Some(input.iter().join("").parse().unwrap()),
            _ => None,
        };
    }

    let state_key = (input_index, alu.get_variable_value(Variable::Z));
    if infeasible_states.contains(&state_key) {
        return None;
    }

    for number in possible_numbers {
        let mut alu = alu;
        let mut instruction_index = instruction_index + 1;

        alu.apply(&Instruction {
            instruction_type: InstructionType::Inp,
            variable: Variable::W,
            placeholder: Placeholder::ParsedInput(*number),
        });

        while let Some(instruction) = instructions.get(instruction_index) {
            if instruction.instruction_type == InstructionType::Inp {
                break;
            }

            alu.apply(instruction);
            instruction_index += 1;
        }

        let mut input = input.clone();
        input[input_index] = *number;

        let result = construct_model_number(
            alu,
            instructions,
            instruction_index,
            input,
            input_index + 1,
            possible_numbers,
            infeasible_states,
        );

        if result.is_some() {
            return result;
        }

        infeasible_states.insert(state_key);
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions = parse_input(input);

    let mut infeasible_states: HashSet<(usize, isize)> = HashSet::new();
    construct_model_number(
        ArithmeticLogicUnit::new(),
        &instructions,
        0,
        [0; 14],
        0,
        &[9, 8, 7, 6, 5, 4, 3, 2, 1],
        &mut infeasible_states,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_input(input);

    let mut infeasible_states: HashSet<(usize, isize)> = HashSet::new();
    construct_model_number(
        ArithmeticLogicUnit::new(),
        &instructions,
        0,
        [0; 14],
        0,
        &[1, 2, 3, 4, 5, 6, 7, 8, 9],
        &mut infeasible_states,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
