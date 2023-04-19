use std::collections::HashSet;

#[derive(Debug)]
struct BingoBoardBuilder {
    board: Vec<Vec<usize>>,
}

impl BingoBoardBuilder {
    fn new() -> Self {
        Self { board: vec![] }
    }

    fn add_row(&mut self, row: Vec<usize>) {
        self.board.push(row)
    }

    fn build(self) -> BingoBoard {
        BingoBoard { board: self.board }
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<usize>>,
}

impl BingoBoard {
    fn won(&self, numbers: &HashSet<usize>) -> bool {
        // First check if any of the rows causes the board to be winning.
        if self
            .board
            .iter()
            .any(|row| row.iter().all(|number| numbers.contains(number)))
        {
            return true;
        }

        // Then check if any of the columns causes the board to be winning.
        (0..self.board[0].len()).any(|colum_index| {
            self.board
                .iter()
                .all(|row| numbers.contains(&row[colum_index]))
        })
    }

    fn unused_numbers(&self, numbers: &HashSet<usize>) -> Vec<usize> {
        self.board
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|number| !numbers.contains(number))
                    .cloned()
                    .collect::<Vec<usize>>()
            })
            .collect()
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    lines.next();

    let (mut boards, builder) = lines.fold(
        (vec![], BingoBoardBuilder::new()),
        |(mut boards, mut builder), line| {
            if line.is_empty() {
                boards.push(builder.build());
                builder = BingoBoardBuilder::new();
            } else {
                builder.add_row(
                    line.split(" ")
                        .filter(|element| !element.is_empty())
                        .map(|number| number.parse().unwrap())
                        .collect(),
                )
            }

            (boards, builder)
        },
    );

    boards.push(builder.build());

    (numbers, boards)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, boards) = parse_input(input);
    let mut played_numbers: HashSet<usize> = HashSet::new();

    for number in numbers {
        played_numbers.insert(number);

        if let Some(winning_board) = boards
            .iter()
            .filter(|board| board.won(&played_numbers))
            .next()
        {
            let sum_of_unused_numbers: usize =
                winning_board.unused_numbers(&played_numbers).iter().sum();

            return Some((sum_of_unused_numbers * number) as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, mut boards) = parse_input(input);
    let mut played_numbers: HashSet<usize> = HashSet::new();

    for number in numbers {
        played_numbers.insert(number);

        if boards.len() == 1 && boards[0].won(&played_numbers) {
            let sum_of_unused_numbers: usize =
                boards[0].unused_numbers(&played_numbers).iter().sum();

            return Some((sum_of_unused_numbers * number) as u32);
        }

        boards.retain(|board| !board.won(&played_numbers));
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(1924));
    }
}
