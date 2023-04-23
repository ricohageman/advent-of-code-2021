use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|element| element.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

fn neighbours(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    let (i, j) = point;

    if i > 0 {
        neighbours.push((i - 1, j));
    }

    if i < width - 1 {
        neighbours.push((i + 1, j));
    }

    if j > 0 {
        neighbours.push((i, j - 1));
    }

    if j < height - 1 {
        neighbours.push((i, j + 1));
    }

    neighbours
}

fn find_low_point_coordinates(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let width = grid.len();
    let height = grid[0].len();

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, element)| {
                    neighbours((i, *j), width, height)
                        .into_iter()
                        .all(|(i, j)| grid[i][j] > **element)
                })
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let low_point_coordinates = find_low_point_coordinates(&grid);

    Some(
        low_point_coordinates
            .into_iter()
            .map(|(i, j)| grid[i][j] + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let width = grid.len();
    let height = grid[0].len();

    let low_point_coordinates = find_low_point_coordinates(&grid);

    Some(
        low_point_coordinates
            .into_iter()
            .map(|(i, j)| {
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_front((i, j));

                let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();

                while let Some((i, j)) = queue.pop_front() {
                    if visited_nodes.contains(&(i, j)) {
                        continue;
                    }

                    if grid[i][j] == 9 {
                        continue;
                    }

                    visited_nodes.insert((i, j));
                    queue.extend(neighbours((i, j), width, height));
                }

                visited_nodes.len() as u32
            })
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, element| acc * element),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1134));
    }
}
