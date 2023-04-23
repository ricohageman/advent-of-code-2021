use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

fn parse_string_to_cave(input: &str) -> Cave {
    if input == "start" {
        return Cave::Start;
    }

    if input == "end" {
        return Cave::End;
    }

    if input.chars().next().unwrap().is_uppercase() {
        return Cave::Large(input.to_string());
    }

    Cave::Small(input.to_string())
}

fn parse_input(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let transitions = input
        .lines()
        .map(|line| {
            let mut data = line.split("-");
            let departure = parse_string_to_cave(data.next().unwrap());
            let arrival = parse_string_to_cave(data.next().unwrap());

            (departure, arrival)
        })
        .collect::<HashSet<(Cave, Cave)>>();

    let nodes = transitions
        .iter()
        .flat_map(|(departure, arrival)| vec![departure.clone(), arrival.clone()])
        .unique()
        .collect::<Vec<Cave>>();

    nodes
        .iter()
        .map(|departure| {
            (
                departure.to_owned(),
                nodes
                    .iter()
                    .filter(|arrival| arrival != &&Cave::Start)
                    .filter(|arrival| {
                        transitions.contains(&(departure.clone(), arrival.clone().to_owned()))
                            || transitions
                                .contains(&(arrival.clone().to_owned(), departure.clone()))
                    })
                    .map(|node| node.to_owned())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let neighbours = parse_input(input);

    let mut solutions: Vec<Vec<Cave>> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_front(vec![Cave::Start]);

    while let Some(path) = queue.pop_front() {
        let last_node = path.last().unwrap();

        if last_node == &Cave::End {
            solutions.push(path);
            continue;
        }

        for neighbour in neighbours.get(last_node).unwrap() {
            match neighbour {
                Cave::Start => panic!("Cannot go back to start to prevent loops"),
                Cave::End | Cave::Large(_) => {}
                Cave::Small(_) => {
                    if path.contains(neighbour) {
                        continue;
                    }
                }
            }

            let mut path = path.clone();
            path.push(neighbour.clone());
            queue.push_front(path);
        }
    }

    Some(solutions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let neighbours = parse_input(input);

    let mut solutions: Vec<Vec<Cave>> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_front((vec![Cave::Start], false));

    while let Some((path, visited_small_node_twice)) = queue.pop_front() {
        let last_node = path.last().unwrap();

        if last_node == &Cave::End {
            solutions.push(path);
            continue;
        }

        for neighbour in neighbours.get(last_node).unwrap() {
            match neighbour {
                Cave::Start => panic!("Cannot go back to start to prevent loops"),
                Cave::End | Cave::Large(_) => {
                    let mut path = path.clone();
                    path.push(neighbour.clone());
                    queue.push_front((path, visited_small_node_twice));
                }
                Cave::Small(_) => {
                    let will_visit_small_node_again = path.contains(neighbour);

                    if will_visit_small_node_again && visited_small_node_twice {
                        continue;
                    }

                    let visited_small_node_twice =
                        will_visit_small_node_again || visited_small_node_twice;

                    let mut path = path.clone();
                    path.push(neighbour.clone());
                    queue.push_front((path, visited_small_node_twice));
                }
            };
        }
    }

    Some(solutions.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(19));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(103));
    }
}
