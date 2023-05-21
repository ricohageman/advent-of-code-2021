use std::cmp::{max, min};

#[derive(Clone, Copy, Debug)]
pub struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn parse_range(input: &str) -> (isize, isize) {
        let mut data = input.split("=").skip(1).next().unwrap().split("..");
        (
            data.next().unwrap().parse().unwrap(),
            data.next().unwrap().parse().unwrap(),
        )
    }

    fn from_string(input: &str) -> Self {
        let mut data = input.split(",");
        let x = Cuboid::parse_range(data.next().unwrap());
        let y = Cuboid::parse_range(data.next().unwrap());
        let z = Cuboid::parse_range(data.next().unwrap());

        Self { x, y, z }
    }

    fn size(&self) -> isize {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn overlap(&self, other: Cuboid) -> Option<Cuboid> {
        // First check if the cuboids even overlap
        if self.x.0 > other.x.1
            || self.x.1 < other.x.0
            || self.y.0 > other.y.1
            || self.y.1 < other.y.0
            || self.z.0 > other.z.1
            || self.z.1 < other.z.0
        {
            return None;
        }

        let x = (max(self.x.0, other.x.0), min(self.x.1, other.x.1));
        let y = (max(self.y.0, other.y.0), min(self.y.1, other.y.1));
        let z = (max(self.z.0, other.z.0), min(self.z.1, other.z.1));

        return Some(Self { x, y, z });
    }

    fn remove(&self, other: Cuboid) -> Vec<Cuboid> {
        let mut splits = vec![];

        if other.x.0 > self.x.0 {
            splits.push(Self {
                x: (self.x.0, other.x.0 - 1),
                y: self.y,
                z: self.z,
            })
        }

        if other.x.1 < self.x.1 {
            splits.push(Self {
                x: (other.x.1 + 1, self.x.1),
                y: self.y,
                z: self.z,
            })
        }

        if other.y.0 > self.y.0 {
            splits.push(Self {
                x: (other.x.0, other.x.1),
                y: (self.y.0, other.y.0 - 1),
                z: self.z,
            })
        }

        if other.y.1 < self.y.1 {
            splits.push(Self {
                x: (other.x.0, other.x.1),
                y: (other.y.1 + 1, self.y.1),
                z: self.z,
            })
        }

        if other.z.0 > self.z.0 {
            splits.push(Self {
                x: other.x,
                y: other.y,
                z: (self.z.0, other.z.0 - 1),
            })
        }

        if other.z.1 < self.z.1 {
            splits.push(Self {
                x: other.x,
                y: other.y,
                z: (other.z.1 + 1, self.z.1),
            })
        }

        splits
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Step {
    On(Cuboid),
    Off(Cuboid),
}

impl Step {
    fn cuboid(self) -> Cuboid {
        match self {
            Step::On(cuboid) => cuboid,
            Step::Off(cuboid) => cuboid,
        }
    }

    fn from_string(input: &str) -> Self {
        let mut data = input.split(" ");
        match data.next().unwrap() {
            "on" => Step::On(Cuboid::from_string(data.next().unwrap())),
            "off" => Step::Off(Cuboid::from_string(data.next().unwrap())),
            _ => panic!("{}", input),
        }
    }

    fn apply(self, other: Step) -> Vec<Step> {
        match (self, other) {
            (Step::On(_), Step::On(_)) => vec![self],
            (Step::Off(_), Step::Off(_)) => vec![self],
            (Step::On(on_cuboid), Step::Off(off_cuboid)) => match on_cuboid.overlap(off_cuboid) {
                Some(overlapping_cuboid) => {
                    let mut steps: Vec<Step> = on_cuboid
                        .remove(overlapping_cuboid)
                        .into_iter()
                        .map(|cuboid| Step::On(cuboid))
                        .collect();

                    steps.push(Step::Off(overlapping_cuboid));
                    steps
                }
                None => vec![self],
            },
            (Step::Off(off_cuboid), Step::On(on_cuboid)) => match off_cuboid.overlap(on_cuboid) {
                Some(overlapping_cuboid) => {
                    let mut steps: Vec<Step> = off_cuboid
                        .remove(overlapping_cuboid)
                        .into_iter()
                        .map(|cuboid| Step::Off(cuboid))
                        .collect();

                    steps.push(Step::On(overlapping_cuboid));
                    steps
                }
                None => vec![self],
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<Step> {
    input.lines().map(Step::from_string).collect()
}

struct Grid {
    steps: Vec<Step>,
}

impl Grid {
    fn new(size: isize) -> Self {
        Self {
            steps: vec![Step::Off(Cuboid {
                x: (-size, size),
                y: (-size, size),
                z: (-size, size),
            })],
        }
    }

    fn apply(&mut self, step: Step) {
        self.steps = self
            .steps
            .iter()
            .flat_map(|cuboid| cuboid.apply(step))
            .collect()
    }

    fn number_of_turned_on_cubes(&self) -> isize {
        self.steps
            .iter()
            .map(|step| match step {
                Step::On(cuboid) => cuboid.size(),
                Step::Off(_) => 0,
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let steps = parse_input(input);
    let mut grid = Grid::new(50);

    for step in steps {
        grid.apply(step)
    }

    Some(grid.number_of_turned_on_cubes())
}

pub fn part_two(input: &str) -> Option<isize> {
    let steps = parse_input(input);
    let (x, y, z) = steps.iter().fold(
        ((0, 0), (0, 0), (0, 0)),
        |((xmin, xmax), (ymin, ymax), (zmin, zmax)), step| {
            let cuboid = step.cuboid();

            (
                (min(xmin, cuboid.x.0), max(xmax, cuboid.x.1)),
                (min(ymin, cuboid.y.0), max(ymax, cuboid.y.1)),
                (min(zmin, cuboid.z.0), max(zmax, cuboid.z.1)),
            )
        },
    );

    let mut grid = Grid {
        steps: vec![Step::Off(Cuboid { x, y, z })],
    };

    for step in steps {
        grid.apply(step)
    }

    Some(grid.number_of_turned_on_cubes())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuboid_size() {
        assert_eq!(
            Cuboid {
                x: (10, 12),
                y: (10, 12),
                z: (10, 12),
            }
            .size(),
            27
        );

        assert_eq!(
            Cuboid {
                x: (10, 10),
                y: (10, 10),
                z: (10, 10),
            }
            .size(),
            1
        );

        assert_eq!(
            Cuboid {
                x: (10, 16),
                y: (10, 10),
                z: (10, 10),
            }
            .size(),
            7
        );
    }

    #[test]
    fn test_example() {
        let mut grid = Grid::new(50);
        assert_eq!(grid.number_of_turned_on_cubes(), 0);

        grid.apply(Step::On(Cuboid {
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
        }));

        assert_eq!(grid.number_of_turned_on_cubes(), 27);

        grid.apply(Step::On(Cuboid {
            x: (11, 13),
            y: (11, 13),
            z: (11, 13),
        }));

        assert_eq!(grid.number_of_turned_on_cubes(), 46);

        grid.apply(Step::Off(Cuboid {
            x: (9, 11),
            y: (9, 11),
            z: (9, 11),
        }));

        assert_eq!(grid.number_of_turned_on_cubes(), 38);

        grid.apply(Step::On(Cuboid {
            x: (10, 10),
            y: (10, 10),
            z: (10, 10),
        }));

        assert_eq!(grid.number_of_turned_on_cubes(), 39);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(590784));
    }
}
