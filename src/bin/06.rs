use std::collections::HashMap;

struct School {
    fishes_per_day: HashMap<usize, usize>,
}

impl School {
    pub fn new(fishes: Vec<usize>) -> Self {
        Self {
            fishes_per_day: fishes.into_iter().fold(HashMap::new(), |mut map, fish| {
                *map.entry(fish).or_default() += 1;
                map
            }),
        }
    }

    pub fn update(&self) -> Self {
        let mut updated_amount_of_fish: HashMap<usize, usize> = HashMap::new();

        for (day, amount) in &self.fishes_per_day {
            match day {
                0 => {
                    *updated_amount_of_fish.entry(6).or_default() += amount;
                    *updated_amount_of_fish.entry(8).or_default() += amount;
                }
                _ => {
                    *updated_amount_of_fish.entry(day - 1).or_default() += amount;
                }
            }
        }

        School {
            fishes_per_day: updated_amount_of_fish,
        }
    }

    pub fn number_of_fish(&self) -> usize {
        self.fishes_per_day.values().sum()
    }
}

fn parse_input(input: &str) -> School {
    School::new(
        input
            .split(",")
            .map(|element| element.parse().unwrap())
            .collect(),
    )
}

fn determine_number_of_fishes_after(fishes: School, days: usize) -> usize {
    let mut current_school = fishes;

    for _ in 0..days {
        current_school = current_school.update();
    }

    current_school.number_of_fish()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(determine_number_of_fishes_after(parse_input(input), 80) as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(determine_number_of_fishes_after(parse_input(input), 256))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5934));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26984457539));
    }
}
