use std::collections::{HashSet, VecDeque};

fn neighbours(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let (i, j) = (point.0 as isize, point.1 as isize);
    let neighbours: Vec<(isize, isize)> = vec![
        (i - 1, j),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j),
        (i + 1, j - 1),
        (i + 1, j + 1),
        (i, j + 1),
        (i, j - 1),
    ];

    neighbours
        .into_iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < width as isize && *j < height as isize)
        .map(|(i, j)| (i as usize, j as usize))
        .collect()
}

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

fn next_step(grid: &mut Vec<Vec<u32>>) -> usize {
    let width = grid.len();
    let height = grid[0].len();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut flashing_octopus: HashSet<(usize, usize)> = HashSet::new();

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, element) in row.iter_mut().enumerate() {
            *element += 1;

            if *element > 9 {
                queue.extend(neighbours((i, j), width, height));
                flashing_octopus.insert((i, j));
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        if flashing_octopus.contains(&(i, j)) {
            continue;
        }

        grid[i][j] += 1;
        if grid[i][j] > 9 {
            queue.extend(neighbours((i, j), width, height));
            flashing_octopus.insert((i, j));
        }
    }

    for row in grid.iter_mut() {
        for element in row.iter_mut() {
            if *element > 9 {
                *element = 0;
            }
        }
    }

    flashing_octopus.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    Some((0..100).map(|_| next_step(&mut grid)).sum::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    (1..1000)
        .map(|i| (i, next_step(&mut grid)))
        .filter(|(_, number_of_flashing_octopus)| *number_of_flashing_octopus == 10 * 10)
        .next()
        .map(|(iteration, _)| iteration)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(1656));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(195));
    }
}
