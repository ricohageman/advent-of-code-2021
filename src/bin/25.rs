use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_east(&self, grid_size: &(usize, usize)) -> Self {
        let mut next_x = self.x.clone() + 1;
        if next_x > grid_size.0 {
            next_x = 0;
        }

        Self {
            x: next_x,
            y: self.y.clone(),
        }
    }

    fn to_south(&self, grid_size: &(usize, usize)) -> Self {
        let mut next_y = self.y.clone() + 1;
        if next_y > grid_size.1 {
            next_y = 0;
        }

        Self {
            x: self.x.clone(),
            y: next_y,
        }
    }
}

struct Grid {
    pub grid_size: (usize, usize),
    pub east_facing_cucumbers: HashSet<Coordinate>,
    pub south_facing_cucumbers: HashSet<Coordinate>,
}

impl Grid {
    fn new(
        east_facing_cucumbers: HashSet<Coordinate>,
        south_facing_cucumbers: HashSet<Coordinate>,
    ) -> Self {
        let mut all_points = east_facing_cucumbers.clone();
        all_points.extend(&south_facing_cucumbers);

        let max_x = all_points.iter().map(|coord| coord.x).max().unwrap();
        let max_y = all_points.iter().map(|coord| coord.y).max().unwrap();

        Self {
            grid_size: (max_x, max_y),
            east_facing_cucumbers,
            south_facing_cucumbers,
        }
    }

    fn contains_cucumber_at(&self, target: Coordinate) -> bool {
        self.east_facing_cucumbers.contains(&target)
            || self.south_facing_cucumbers.contains(&target)
    }

    fn simulate(&mut self) -> bool {
        let mut has_changed = false;
        let mut east_facing_cucumbers_changes: Vec<(Coordinate, Coordinate)> = Vec::new();
        let mut south_facing_cucumbers_changes: Vec<(Coordinate, Coordinate)> = Vec::new();

        for origin in &self.east_facing_cucumbers {
            let target = origin.to_east(&self.grid_size);

            if self.contains_cucumber_at(target) {
                continue;
            }

            east_facing_cucumbers_changes.push((*origin, target))
        }

        has_changed = has_changed || east_facing_cucumbers_changes.len() > 0;

        for (origin, target) in east_facing_cucumbers_changes {
            self.east_facing_cucumbers.remove(&origin);
            self.east_facing_cucumbers.insert(target);
        }

        for origin in &self.south_facing_cucumbers {
            let target = origin.to_south(&self.grid_size);

            if self.contains_cucumber_at(target) {
                continue;
            }

            south_facing_cucumbers_changes.push((*origin, target))
        }

        has_changed = has_changed || south_facing_cucumbers_changes.len() > 0;

        for (origin, target) in south_facing_cucumbers_changes {
            self.south_facing_cucumbers.remove(&origin);
            self.south_facing_cucumbers.insert(target);
        }

        has_changed
    }
}

fn parse_input(input: &str) -> Grid {
    let cucumbers: Vec<(usize, usize, bool)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split("")
                .filter(|element| element != &"")
                .enumerate()
                .filter_map(|(x, element)| match element {
                    "." => None,
                    ">" => Some((x, y, true)),
                    "v" => Some((x, y, false)),
                    _ => panic!("{}", element),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut east_facing_cucumbers = HashSet::default();
    let mut south_facing_cucumbers = HashSet::default();

    for (x, y, is_east_facing_cucumber) in cucumbers {
        if is_east_facing_cucumber {
            east_facing_cucumbers.insert(Coordinate::new(x, y));
        } else {
            south_facing_cucumbers.insert(Coordinate::new(x, y));
        }
    }

    Grid::new(east_facing_cucumbers, south_facing_cucumbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let mut iterations = 1;

    while grid.simulate() {
        iterations += 1;
    }

    Some(iterations)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(58));
    }
}
