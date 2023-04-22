use crate::Line::{Diagonal, Horizontal, Vertical};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

fn parse_coordinate(element: &str) -> (isize, isize) {
    let x: Vec<isize> = element
        .split(",")
        .map(|element| element.parse().unwrap())
        .collect();

    (x[0], x[1])
}

#[derive(Debug)]
enum Line {
    Horizontal {
        y: isize,
        x_range: (isize, isize),
    },
    Vertical {
        x: isize,
        y_range: (isize, isize),
    },
    Diagonal {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    },
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut data = line.split(" -> ");
            let x = parse_coordinate(data.next().unwrap());
            let y = parse_coordinate(data.next().unwrap());

            (x, y)
        })
        .filter_map(|((x1, y1), (x2, y2))| match (x1 == x2, y1 == y2) {
            (true, false) => Some(Vertical {
                x: x1,
                y_range: (min(y1, y2), max(y1, y2)),
            }),
            (false, true) => Some(Horizontal {
                y: y1,
                x_range: (min(x1, x2), max(x1, x2)),
            }),
            (false, false) => Some(Diagonal { x1, y1, x2, y2 }),
            _ => panic!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .filter(|line| match line {
                Horizontal { .. } => true,
                Vertical { .. } => true,
                Diagonal { .. } => false,
            })
            .combinations(2)
            .map(|l| (l[0], l[1]))
            .flat_map(|(l1, l2)| match (l1, l2) {
                (
                    Vertical {
                        x: x1,
                        y_range: (y11, y12),
                    },
                    Vertical {
                        x: x2,
                        y_range: (y21, y22),
                    },
                ) => {
                    if x1 != x2 {
                        return vec![];
                    }

                    (*max(y11, y21)..min(y12, y22) + 1)
                        .collect::<Vec<isize>>()
                        .iter()
                        .map(|y| (*x1, *y))
                        .collect()
                }
                (
                    Horizontal {
                        y: y1,
                        x_range: (x11, x12),
                    },
                    Horizontal {
                        y: y2,
                        x_range: (x21, x22),
                    },
                ) => {
                    if y1 != y2 {
                        return vec![];
                    }

                    (*max(x11, x21)..min(x12, x22) + 1)
                        .map(|x| (x, *y1))
                        .collect()
                }
                (Vertical { x, y_range }, Horizontal { y, x_range })
                | (Horizontal { y, x_range }, Vertical { x, y_range }) => {
                    if (y_range.0..=y_range.1).contains(y) && (x_range.0..=x_range.1).contains(x) {
                        return vec![(*x, *y)];
                    }

                    vec![]
                }
                _ => vec![],
            })
            .unique()
            .count() as u32,
    )
}

struct Grid {
    grid: HashMap<(isize, isize), usize>,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    fn increment(&mut self, key: (isize, isize)) {
        match self.grid.get_mut(&key) {
            None => {
                self.grid.insert(key, 1);
            }
            Some(amount) => *amount += 1,
        };
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input)
        .iter()
        .fold(Grid::new(), |mut grid, line| {
            match line {
                Vertical {
                    x,
                    y_range: (y1, y2),
                } => {
                    for y in *y1..=*y2 {
                        grid.increment((*x, y))
                    }
                }
                Horizontal {
                    y,
                    x_range: (x1, x2),
                } => {
                    for x in *x1..=*x2 {
                        grid.increment((x, *y))
                    }
                }
                Diagonal { x1, y1, x2, y2 } => {
                    let dx = (x2 - x1).signum();
                    let dy = (y2 - y1).signum();

                    let mut x = *x1;
                    let mut y = *y1;

                    while x != *x2 && y != *y2 {
                        grid.increment((x, y));
                        x += dx;
                        y += dy;
                    }

                    grid.increment((x, y));
                }
            };

            grid
        });

    Some(grid.grid.values().filter(|&&amount| amount > 1).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(12));
    }
}
