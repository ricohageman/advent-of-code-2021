pub fn part_one(input: &str) -> Option<u32> {
    let measurements: Vec<usize> = input.lines()
        .map(|measurement| measurement.parse().unwrap())
        .collect();

    Some(measurements.windows(2)
        .filter(|window| window[0] < window[1])
        .count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let measurements: Vec<usize> = input.lines()
        .map(|measurement| measurement.parse().unwrap())
        .collect();

    Some(measurements.windows(4)
        .filter(|window| window[0] < window[3])
        .count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
