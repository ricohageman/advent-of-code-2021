extern crate core;

use itertools::Itertools;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum LocationType {
    Room(Amphipod),
    Hallway,
}

struct Grid {
    pub rooms: Vec<Vec<Coordinate>>,
    grid: HashMap<Coordinate, LocationType>,
}

impl Grid {
    pub fn new(locations: HashMap<Coordinate, LocationType>) -> Self {
        let rooms = [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ]
        .into_iter()
        .map(|amphipod| {
            locations
                .iter()
                .filter(|(_, location)| {
                    if LocationType::Room(amphipod) == **location {
                        return true;
                    }

                    false
                })
                .map(|(coordinate, _)| *coordinate)
                .collect()
        })
        .collect::<Vec<Vec<Coordinate>>>();

        Self {
            rooms,
            grid: locations,
        }
    }

    fn location(&self, coordinate: &Coordinate) -> LocationType {
        *self.grid.get(coordinate).unwrap()
    }

    fn neighbours(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let (i, j) = *coordinate;

        vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
            .into_iter()
            .filter(|coordinate| self.grid.contains_key(coordinate))
            .collect()
    }

    fn can_stop_at(&self, coordinate: &Coordinate) -> bool {
        match self.grid.get(coordinate).unwrap() {
            LocationType::Room(_) => true,
            LocationType::Hallway => {
                self.neighbours(coordinate).iter().all(|coordinate| {
                    match self.grid.get(coordinate).unwrap() {
                        LocationType::Room(_) => false,
                        _ => true,
                    }
                })
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<((usize, usize), Amphipod)>, Grid) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut number_of_rooms = 0;

            line.split("")
                .filter(|element| !element.is_empty())
                .enumerate()
                .filter_map(move |(x, element)| {
                    let location_type = match element {
                        "#" => None,
                        "." => Some(LocationType::Hallway),
                        "A" | "B" | "C" | "D" => {
                            let result = match number_of_rooms {
                                0 => LocationType::Room(Amphipod::Amber),
                                1 => LocationType::Room(Amphipod::Bronze),
                                2 => LocationType::Room(Amphipod::Copper),
                                3 => LocationType::Room(Amphipod::Desert),
                                _ => panic!(),
                            };
                            number_of_rooms += 1;
                            Some(result)
                        }
                        " " => None,
                        _ => panic!("{element}"),
                    }?;

                    Some(((x, y), location_type))
                })
        })
        .collect();

    let grid = Grid::new(grid);

    let amphipods = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split("")
                .filter(|element| !element.is_empty())
                .enumerate()
                .filter_map(move |(x, element)| {
                    let amphipod = match element {
                        "#" | "." | " " => None,
                        "A" => Some(Amphipod::Amber),
                        "B" => Some(Amphipod::Bronze),
                        "C" => Some(Amphipod::Copper),
                        "D" => Some(Amphipod::Desert),
                        _ => panic!("{element}"),
                    }?;

                    Some(((x, y), amphipod))
                })
        })
        .collect();

    (amphipods, grid)
}

fn solve(
    amphipods: Vec<(Coordinate, Amphipod)>,
    grid: &Grid,
    number_of_amphipods_per_specie: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((amphipods, 0, vec![]));

    let coordinate_per_room_ordered_by_depth: Vec<Vec<Coordinate>> = grid
        .rooms
        .iter()
        .map(|room| {
            room.iter()
                .cloned()
                .sorted_by_key(|coordinate| coordinate.1)
                .rev()
                .collect()
        })
        .collect();

    let mut visited_states = HashMap::<Vec<(Coordinate, Amphipod)>, usize>::new();
    let mut lowest_cost = usize::MAX;
    let mut number_of_iterations = 0;

    while let Some((amphipods, cost, cost_per_move)) = queue.pop_front() {
        number_of_iterations += 1;

        if cost >= lowest_cost {
            continue;
        }

        if let Some(previous_cost) = visited_states.get(&amphipods) {
            if *previous_cost <= cost {
                continue;
            }
        }

        let room_occupation: Vec<HashSet<Coordinate>> = grid
            .rooms
            .iter()
            .enumerate()
            .map(|(index, coordinates)| {
                amphipods
                    .iter()
                    .filter(|(_, amphipod)| *amphipod as usize == index)
                    .filter(|(current, _)| coordinates.contains(current))
                    .map(|(coordinate, _)| *coordinate)
                    .collect()
            })
            .collect();

        if room_occupation.iter().all(|occupied_coordinates| {
            occupied_coordinates.len() == number_of_amphipods_per_specie
        }) {
            lowest_cost = min(lowest_cost, cost);
            continue;
        }

        visited_states.insert(amphipods.clone(), cost);

        for (index, (coordinate, amphipod)) in amphipods.iter().enumerate() {
            // No need to do anything with this amphipod if the target room is completely filled with
            //  amphipods of the correct type.
            let target_room_occupation = &room_occupation[*amphipod as usize];

            if target_room_occupation.len() == number_of_amphipods_per_specie {
                continue;
            }

            let room_coordinates_ordered_by_depth =
                &coordinate_per_room_ordered_by_depth[*amphipod as usize];

            if target_room_occupation.len() > 0 {
                let current_in_stack = room_coordinates_ordered_by_depth
                    .iter()
                    .take(target_room_occupation.len())
                    .any(|target| target == coordinate);

                let stack_completely_filled = room_coordinates_ordered_by_depth
                    .iter()
                    .take(target_room_occupation.len())
                    .all(|coordinate| target_room_occupation.contains(coordinate));

                if current_in_stack && stack_completely_filled {
                    continue;
                }
            }

            let occupied_positions: HashSet<Coordinate> = amphipods
                .iter()
                .map(|(coordinate, _)| *coordinate)
                .collect();

            let mut visited_positions = HashSet::<(usize, usize)>::new();
            let mut position_queue = VecDeque::<((usize, usize), usize)>::new();

            position_queue.push_front((*coordinate, 0));

            let target_room_coordinate = room_coordinates_ordered_by_depth
                .iter()
                .filter(|coordinate| !target_room_occupation.contains(coordinate))
                .next()
                .unwrap();

            while let Some((next_coordinate, steps)) = position_queue.pop_front() {
                // If this coordinate has been explored already, the search stops.
                if visited_positions.contains(&next_coordinate) {
                    continue;
                }

                // Add all the non-explored neighbours of the current coordinate to the search.
                position_queue.extend(
                    grid.neighbours(&next_coordinate)
                        .iter()
                        .filter(|coordinate| !occupied_positions.contains(coordinate))
                        .filter(|coordinate| !visited_positions.contains(coordinate))
                        .map(|coordinate| (*coordinate, steps + 1)),
                );

                visited_positions.insert(next_coordinate);

                // Make sure that an Amphipod moves once out of a room, and once into a room.
                match (grid.location(coordinate), grid.location(&next_coordinate)) {
                    (_, LocationType::Room(_)) => {
                        if next_coordinate != *target_room_coordinate {
                            continue;
                        }
                    }
                    (LocationType::Room(_), LocationType::Hallway) => {
                        if !grid.can_stop_at(&next_coordinate) {
                            continue;
                        }
                    }
                    _ => continue,
                }

                // Add the new state to the search space.
                let mut amphipods = amphipods.clone();
                amphipods[index].0 = next_coordinate;

                let cost_per_step = match amphipod {
                    Amphipod::Amber => 1,
                    Amphipod::Bronze => 10,
                    Amphipod::Copper => 100,
                    Amphipod::Desert => 1000,
                };

                let mut cost_per_move = cost_per_move.clone();
                cost_per_move.push(steps * cost_per_step);

                let next_state = (amphipods, cost + steps * cost_per_step, cost_per_move);
                queue.push_front(next_state)
            }
        }
    }

    lowest_cost
}

pub fn part_one(input: &str) -> Option<usize> {
    let (amphipods, grid) = parse_input(input);
    let number_of_amphipods_per_specie = 2;

    Some(solve(amphipods, &grid, number_of_amphipods_per_specie))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input.lines().collect::<Vec<_>>();
    input.insert(3, "  #D#C#B#A#");
    input.insert(4, "  #D#B#A#C#");

    let input = input.join("\n");

    let (amphipods, grid) = parse_input(&input);
    let number_of_amphipods_per_specie = 4;

    Some(solve(amphipods, &grid, number_of_amphipods_per_specie))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(12521));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(44169));
    }
}
