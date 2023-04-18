use crate::Command::{Down, Forward, Up};

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|data| {
            let mut data = data.split(" ");
            let command_data = data.next().unwrap();
            let amount = data.next().unwrap().parse().unwrap();

            match command_data {
                "forward" => Forward(amount),
                "down" => Down(amount),
                "up" => Up(amount),
                _ => panic!("{}", command_data)
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_input(input)
        .iter()
        .fold((0, 0), |(depth, position), command| {
            match command {
                Forward(amount) => (depth, position + amount),
                Down(amount) => (depth + amount, position),
                Up(amount) => (depth - amount, position),
            }
        });

    Some((result.0 * result.1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse_input(input)
        .iter()
        .fold((0, 0, 0), |(depth, position, aim), command| {
            match command {
                Forward(amount) => (depth + amount * aim, position + amount, aim),
                Down(amount) => (depth, position, aim + amount),
                Up(amount) => (depth, position, aim - amount),
            }
        });

    Some((result.0 * result.1) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
