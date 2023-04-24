use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<((String, String), String)>) {
    let mut data = input.split("\n\n");
    let template = data
        .next()
        .unwrap()
        .split("")
        .filter(|element| !element.is_empty())
        .map(|element| element.to_string())
        .collect();

    let rules = data
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut data = line.split(" -> ");
            let lhs = data
                .next()
                .unwrap()
                .split("")
                .filter(|element| !element.is_empty())
                .map(|element| element.to_string())
                .collect::<Vec<_>>();

            let rhs = data.next().unwrap().to_string();

            ((lhs[0].clone(), lhs[1].clone()), rhs)
        })
        .collect();

    (template, rules)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (template, rules) = parse_input(input);

    let mut template = template;

    for _ in 1..=10 {
        let mut result: Vec<String> = template
            .iter()
            .tuple_windows()
            .flat_map(|(lhs, rhs)| {
                let rule = rules
                    .iter()
                    .filter(|((possible_lhs, possible_rhs), _)| {
                        possible_lhs == lhs && possible_rhs == rhs
                    })
                    .map(|(_, insertion)| insertion.clone())
                    .next();

                match rule {
                    Some(insertion) => vec![lhs.clone(), insertion],
                    None => vec![lhs.clone()],
                }
            })
            .collect();
        result.push(template.last().unwrap().clone());
        template = result.clone();
    }

    let occurrences_and_frequency =
        template
            .iter()
            .fold(HashMap::<String, usize>::new(), |mut map, element| {
                *map.entry(element.clone()).or_default() += 1;
                map
            });

    let frequencies = occurrences_and_frequency.values().collect::<Vec<_>>();

    match frequencies.iter().minmax() {
        MinMaxResult::NoElements => None,
        MinMaxResult::OneElement(_) => None,
        MinMaxResult::MinMax(min, max) => Some((**max - **min) as u32),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (input, rules) = parse_input(input);

    let mut template = input.clone().into_iter().tuple_windows().fold(
        HashMap::<(String, String), usize>::new(),
        |mut map, element: (String, String)| {
            *map.entry(element.clone()).or_default() += 1;
            map
        },
    );

    for _ in 1..=40 {
        let mut result = HashMap::new();

        for (pair, amount) in template {
            let rule = rules
                .iter()
                .filter(|(possible_match, _)| *possible_match == pair)
                .map(|(_, insertion)| insertion.clone())
                .next();

            match rule {
                Some(insertion) => {
                    *result.entry((pair.0, insertion.clone())).or_default() += amount;
                    *result.entry((insertion.clone(), pair.1)).or_default() += amount;
                }
                None => *result.entry(pair).or_default() += amount,
            };
        }

        template = result;
    }

    let mut frequencies: HashMap<String, usize> = HashMap::new();

    *frequencies
        .entry(input.clone().first().unwrap().clone())
        .or_default() += 1;
    for ((lhs, rhs), amount) in template {
        *frequencies.entry(lhs).or_default() += amount;
        *frequencies.entry(rhs).or_default() += amount;
    }
    *frequencies
        .entry(input.clone().last().unwrap().clone())
        .or_default() += 1;

    match frequencies.values().minmax() {
        MinMaxResult::NoElements => None,
        MinMaxResult::OneElement(_) => None,
        MinMaxResult::MinMax(min, max) => Some(*max / 2 - *min / 2),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(1588));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(2188189693529));
    }
}
