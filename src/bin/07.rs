fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|element| element.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let positions = parse_input(input);
    let maximum_position = positions.iter().max().unwrap();

    (0..=*maximum_position)
        .map(|target| {
            positions
                .iter()
                .map(|position| (target - position).abs())
                .sum::<isize>() as u32
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let positions = parse_input(input);
    let maximum_position = positions.iter().max().unwrap();

    (0..=*maximum_position)
        .map(|target| {
            positions
                .iter()
                .map(|position| {
                    let distance = (target - position).abs();
                    distance * (distance + 1) / 2
                })
                .sum::<isize>() as u32
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(168));
    }
}
