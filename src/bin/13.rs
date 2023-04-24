use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

struct Paper {
    points: Vec<(usize, usize)>,
}

impl Paper {
    pub fn new(points: Vec<(usize, usize)>) -> Self {
        Self { points }
    }

    pub fn apply(&self, fold_instruction: FoldInstruction) -> Self {
        let points = self
            .points
            .iter()
            .cloned()
            .map(|(x, y)| match fold_instruction {
                FoldInstruction::X(line) => match x.cmp(&line) {
                    Ordering::Less => (x, y),
                    Ordering::Equal | Ordering::Greater => (2 * line - x, y),
                },
                FoldInstruction::Y(line) => match y.cmp(&line) {
                    Ordering::Less => (x, y),
                    Ordering::Equal | Ordering::Greater => (x, 2 * line - y),
                },
            })
            .unique()
            .collect::<Vec<_>>();

        Self { points }
    }

    pub fn output(&self) {
        let width = self.points.iter().map(|(x, _)| x).max().unwrap();
        let height = self.points.iter().map(|(_, y)| y).max().unwrap();

        for j in 0..=*height {
            for i in 0..=*width {
                if self.points.contains(&(i, j)) {
                    print!(" # ");
                } else {
                    print!(" . ");
                }
            }

            print!("\n");
        }
    }
}

fn parse_input(input: &str) -> (Paper, Vec<FoldInstruction>) {
    let mut data = input.split("\n\n");
    let points = data
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut data = line.split(",");
            (
                data.next().unwrap().parse::<usize>().unwrap(),
                data.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let instructions = data
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut data = line.split("=");
            match data.next().unwrap() {
                "fold along x" => FoldInstruction::X(data.next().unwrap().parse().unwrap()),
                "fold along y" => FoldInstruction::Y(data.next().unwrap().parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    (Paper::new(points), instructions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (paper, instructions) = parse_input(input);
    let result = paper.apply(*instructions.first().unwrap());

    Some(result.points.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (paper, instructions) = parse_input(input);

    let result = instructions
        .iter()
        .fold(paper, |paper, instruction| paper.apply(*instruction));

    result.output();

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(17));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
