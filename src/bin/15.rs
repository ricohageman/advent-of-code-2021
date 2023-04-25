use std::cmp::Reverse;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|element| !element.is_empty())
                .map(|element| element.parse().unwrap())
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

fn dijkstra(grid: Vec<Vec<usize>>) -> Option<u32> {
    let width = grid.len();
    let height = grid[0].len();

    let target = (width - 1, height - 1);

    // Naive implementation of dijkstra without using a priority queue
    let mut queue: Vec<(usize, usize)> = (0..=width)
        .flat_map(|x| (0..=height).map(|y| (x, y)).collect::<Vec<_>>())
        .collect();

    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    distance.insert((0, 0), 0);

    while queue.len() > 0 {
        let (index, u, d) = queue
            .iter()
            .enumerate()
            .filter_map(|(index, element)| {
                Some((
                    index,
                    *element,
                    *distance.get(element).unwrap_or(&usize::MAX),
                ))
            })
            .min_by_key(|element| element.2)
            .unwrap();

        if u == target {
            break;
        }

        queue.remove(index);

        for neighbour in neighbours(u, width, height) {
            if !queue.contains(&neighbour) {
                continue;
            }

            let alternative = d + grid[neighbour.0][neighbour.1];
            if alternative < *distance.get(&neighbour).unwrap_or(&usize::MAX) {
                distance.insert(neighbour, alternative);
            }
        }
    }

    distance.get(&target).map(|distance| *distance as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    dijkstra(grid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut extended_grid: Vec<Vec<usize>> = Vec::new();

    for index in 0..5 {
        for row in &grid {
            extended_grid.push(
                row.iter()
                    .cloned()
                    .map(|x| {
                        if x + index > 9 {
                            return x + index - 9;
                        }

                        x + index
                    })
                    .collect(),
            )
        }
    }

    let mut twice_extended_grid: Vec<Vec<usize>> = Vec::new();
    for row in &extended_grid {
        let mut extended_row = Vec::new();

        for index in 0..5 {
            extended_row.extend(row.iter().cloned().map(|x| {
                if x + index > 9 {
                    return x + index - 9;
                }

                x + index
            }))
        }

        twice_extended_grid.push(extended_row);
    }

    dijkstra(twice_extended_grid)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(315));
    }
}
