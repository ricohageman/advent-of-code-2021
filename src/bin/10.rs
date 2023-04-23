use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Character {
    Round,
    Block,
    Curly,
    Triangle,
}

enum Type {
    Open,
    Closed,
}

fn parse_line(line: &str) -> Vec<(Type, Character)> {
    line.split("")
        .filter(|element| !element.is_empty())
        .map(|element| match element {
            "(" => (Type::Open, Character::Round),
            ")" => (Type::Closed, Character::Round),
            "[" => (Type::Open, Character::Block),
            "]" => (Type::Closed, Character::Block),
            "{" => (Type::Open, Character::Curly),
            "}" => (Type::Closed, Character::Curly),
            "<" => (Type::Open, Character::Triangle),
            ">" => (Type::Closed, Character::Triangle),
            _ => panic!("{:?}", element),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let line = parse_line(line);

                let mut stack: VecDeque<Character> = VecDeque::new();

                for (t, char) in line {
                    match (stack.is_empty(), t) {
                        (true, Type::Closed) => panic!(),
                        (true, Type::Open) => stack.push_front(char),
                        (_, Type::Open) => stack.push_back(char),
                        (_, Type::Closed) => {
                            let previous = stack.pop_back().unwrap();

                            if previous != char {
                                return Some(char);
                            }
                        }
                    }
                }

                None
            })
            .map(|char| match char {
                Character::Round => 3,
                Character::Block => 57,
                Character::Curly => 1197,
                Character::Triangle => 25137,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let sorted_scores = input
        .lines()
        .filter_map(|line| {
            let line = parse_line(line);

            let mut stack: VecDeque<Character> = VecDeque::new();

            for (t, char) in line {
                match (stack.is_empty(), t) {
                    (true, Type::Closed) => panic!(),
                    (true, Type::Open) => stack.push_front(char),
                    (_, Type::Open) => stack.push_back(char),
                    (_, Type::Closed) => {
                        let previous = stack.pop_back().unwrap();

                        if previous != char {
                            return None;
                        }
                    }
                }
            }

            Some(stack)
        })
        .map(|chars| {
            chars
                .iter()
                .rev()
                .map(|char| match char {
                    Character::Round => 1,
                    Character::Block => 2,
                    Character::Curly => 3,
                    Character::Triangle => 4,
                })
                .fold(0, |acc, amount| acc * 5 + amount)
        })
        .sorted()
        .collect::<Vec<usize>>();

    Some(sorted_scores[sorted_scores.len() / 2] as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(26397));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(288957));
    }
}
