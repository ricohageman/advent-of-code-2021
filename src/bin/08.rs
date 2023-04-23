use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
        .map(|line| {
            line.split(" | ")
                .skip(1)
                .next()
                .unwrap()
                .split(" ")
                .filter(|element| match element.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>() as u32
    )
}

#[derive(Clone, PartialEq, Eq)]
struct SevenSegmentNumber {
    activated_lines: HashSet<char>
}

impl SevenSegmentNumber {
    fn new(lines: String) -> Self {
        Self {
            activated_lines: lines.chars().collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.activated_lines.len()
    }

    pub fn overlap(&self, other: &Self) -> usize {
        other.activated_lines.iter().filter(|char| self.activated_lines.contains(&char)).count()
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.activated_lines.iter().all(|char| self.activated_lines.contains(&char))
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines()
        .map(|line| {
            let mut data = line.split(" | ");
            let input = data
                .next()
                .unwrap()
                .split(" ")
                .map(|lines| SevenSegmentNumber::new(lines.to_string()))
                .collect::<Vec<_>>();

            let output = data
                .next()
                .unwrap()
                .split(" ")
                .map(|lines| SevenSegmentNumber::new(lines.to_string()))
                .collect::<Vec<_>>();

            let one = input.iter().filter(|element| element.size() == 2).next().unwrap().clone();
            let four = input.iter().filter(|element| element.size() == 4).next().unwrap().clone();
            let seven = input.iter().filter(|element| element.size() == 3).next().unwrap().clone();
            let eight = input.iter().filter(|element| element.size() == 7).next().unwrap().clone();

            let three = input.iter()
                .filter(|element| element.size() == 5)
                .filter(|element| element.contains(&one))
                .filter(|element| element.contains(&seven))
                .next()
                .unwrap()
                .clone();

            let nine = input.iter()
                .filter(|element| element.size() == 6)
                .filter(|element| element.contains(&three))
                .filter(|element| element.contains(&four))
                .next()
                .unwrap()
                .clone();

            let two = input.iter()
                .filter(|element| element.size() == 5)
                .filter(|element| element.overlap(&nine) == 4)
                .filter(|element| **element != three)
                .next()
                .unwrap()
                .clone();

            let five = input.iter()
                .filter(|element| element.size() == 5)
                .filter(|element| **element != two)
                .filter(|element| **element != three)
                .next()
                .unwrap()
                .clone();

            let zero = input.iter()
                .filter(|element| element.size() == 6)
                .filter(|element| element.contains(&one))
                .filter(|element| **element != nine)
                .next()
                .unwrap()
                .clone();

            let six = input.iter()
                .filter(|element| element.size() == 6)
                .filter(|element| **element != zero)
                .filter(|element| **element != nine)
                .next()
                .unwrap()
                .clone();

            assert!(!vec![&two, &three, &four, &five, &six, &seven, &eight, &nine, &zero].contains(&&one));
            assert!(!vec![&one, &three, &four, &five, &six, &seven, &eight, &nine, &zero].contains(&&two));
            assert!(!vec![&one, &two, &four, &five, &six, &seven, &eight, &nine, &zero].contains(&&three));
            assert!(!vec![&one, &two, &three, &five, &six, &seven, &eight, &nine, &zero].contains(&&four));
            assert!(!vec![&one, &two, &three, &four, &six, &seven, &eight, &nine, &zero].contains(&&five));
            assert!(!vec![&one, &two, &three, &four, &five, &seven, &eight, &nine, &zero].contains(&&six));
            assert!(!vec![&one, &two, &three, &four, &five, &six, &eight, &nine, &zero].contains(&&seven));
            assert!(!vec![&one, &two, &three, &four, &five, &six, &seven, &nine, &zero].contains(&&eight));
            assert!(!vec![&one, &two, &three, &four, &five, &six, &seven, &eight, &zero].contains(&&nine));
            assert!(!vec![&one, &two, &three, &four, &five, &six, &seven, &eight, &nine].contains(&&zero));

            output.iter()
                .map(|element| {
                    if element == &zero {
                        return 0;
                    }

                    if element == &one {
                        return 1;
                    }

                    if element == &two {
                        return 2;
                    }

                    if element == &three {
                        return 3;
                    }

                    if element == &four {
                        return 4;
                    }

                    if element == &five {
                        return 5;
                    }

                    if element == &six {
                        return 6;
                    }

                    if element == &seven {
                        return 7;
                    }

                    if element == &eight {
                        return 8;
                    }

                    if element == &nine {
                        return 9;
                    }

                    panic!();
                })
                .enumerate()
                .map(|(index, amount)| match index {
                    0 => 1000 * amount,
                    1 => 100 * amount,
                    2 => 10 * amount,
                    3 => amount,
                    _ => panic!(),
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(61229));
    }
}
