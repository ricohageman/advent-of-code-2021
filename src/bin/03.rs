use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn transpose_input(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed_input: Vec<Vec<_>> = (0..input[0].len()).map(|_| vec![]).collect();
    for row in input {
        for (item, transposed_row) in row.into_iter().zip(&mut transposed_input) {
            transposed_row.push(*item);
        }
    }

    transposed_input
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = transpose_input(&parse_input(input));

    let (gamma_rate, epsilon_rate) =
        input
            .iter()
            .enumerate()
            .fold((0, 0), |(gamma_rate, epsilon_rate), (index, row)| {
                let addition = 2_u32.pow((input.len() - index - 1) as u32);

                match row.iter().sum::<u32>() >= (row.len() / 2) as u32 {
                    true => (gamma_rate + addition, epsilon_rate),
                    false => (gamma_rate, epsilon_rate + addition),
                }
            });

    Some(gamma_rate * epsilon_rate)
}

fn convert_binary_to_decimal(binary: &Vec<u32>) -> u32 {
    binary
        .iter()
        .enumerate()
        .map(|(index, bool)| bool * 2_u32.pow((binary.len() - index - 1) as u32))
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut binary_numbers = parse_input(input);
    let mut position = 0;

    while binary_numbers.len() > 1 {
        let transposed_input = transpose_input(&binary_numbers);
        let number_of_ones = transposed_input[position].iter().sum::<u32>() as usize;
        let number_of_zeros = binary_numbers.len() - number_of_ones;
        let target_value = match number_of_ones.cmp(&number_of_zeros) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 1,
        };

        binary_numbers = binary_numbers
            .into_iter()
            .filter(|binary_number| binary_number[position] == target_value)
            .collect();

        position += 1;
    }

    let oxygen_generator_rating = convert_binary_to_decimal(&binary_numbers[0]);

    let mut binary_numbers = parse_input(input);
    let mut position = 0;

    while binary_numbers.len() > 1 {
        let transposed_input = transpose_input(&binary_numbers);
        let number_of_ones = transposed_input[position].iter().sum::<u32>() as usize;
        let number_of_zeros = binary_numbers.len() - number_of_ones;
        let target_value = match number_of_ones.cmp(&number_of_zeros) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 0,
        };

        binary_numbers = binary_numbers
            .into_iter()
            .filter(|binary_number| binary_number[position] == target_value)
            .collect();

        position += 1;
    }

    let co2_scrubber_rating = convert_binary_to_decimal(&binary_numbers[0]);

    Some(oxygen_generator_rating * co2_scrubber_rating)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(230));
    }
}
