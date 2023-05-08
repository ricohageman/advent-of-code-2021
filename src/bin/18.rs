use itertools::Itertools;
use std::cmp::max;

fn parse_number(input: &str) -> Vec<(usize, usize)> {
    let mut depth = 0;
    let mut number: Vec<(usize, usize)> = vec![];

    for char in input.chars() {
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => continue,
            value => number.push((value.to_string().parse().unwrap(), depth)),
        }
    }

    number
}

fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input.lines().map(|line| parse_number(line)).collect()
}

fn add(lhs: &[(usize, usize)], rhs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (value, depth) in lhs {
        result.push((*value, *depth + 1));
    }

    for (value, depth) in rhs {
        result.push((*value, *depth + 1));
    }

    reduce(result)
}

fn reduce(number: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // First check if there is anything that needs to be exploded
    if let Some((index, (lhs, depth))) =
        number.iter().enumerate().find(|(_, (_, depth))| depth > &4)
    {
        assert_eq!(depth, &5);
        let mut number = number.clone();

        let (rhs, other_depth) = number[index + 1];
        assert_eq!(other_depth, 5);

        // the pair's left value is added to the first regular number to the left of the exploding pair (if any),
        if index > 0 {
            number[index - 1].0 += lhs;
        }

        // and the pair's right value is added to the first regular number to the right of the exploding pair (if any).
        if number.len() > index + 2 {
            number[index + 2].0 += rhs;
        }

        // Exploding pairs will always consist of two regular numbers.
        // Then, the entire exploding pair is replaced with the regular number 0.
        number[index] = (0, depth - 1);
        number.remove(index + 1);

        return reduce(number);
    }

    // Then check if there is a number that is too large
    if let Some((index, (split, depth))) =
        number.iter().enumerate().find(|(_, (value, _))| value > &9)
    {
        let mut number = number.clone();

        // the left element of the pair should be the regular number divided by two and rounded down
        number[index] = (split / 2, depth + 1);
        // the right element of the pair should be the regular number divided by two and rounded up
        number.insert(index + 1, ((split + 1) / 2, depth + 1));

        return reduce(number);
    }

    number
}

fn magnitude(number: Vec<(usize, usize)>) -> usize {
    let number = _fold(number, 4);
    assert_eq!(number.len(), 1);

    number[0].0
}

fn _fold(number: Vec<(usize, usize)>, depth: usize) -> Vec<(usize, usize)> {
    let same_depth_pair: Option<(usize, (usize, usize))> = number
        .iter()
        .tuple_windows()
        .enumerate()
        .filter(|(_, ((_, d1), (_, d2)))| d1 == d2 && *d1 == depth)
        .map(|(index, ((v1, _), (v2, _)))| (index, (*v1, *v2)))
        .next();

    if let Some((index, (v1, v2))) = same_depth_pair {
        let mut folded_number = number.clone();
        folded_number.remove(index + 1);
        folded_number.remove(index);
        folded_number.insert(index, (3 * v1 + 2 * v2, depth - 1));

        return _fold(folded_number, depth);
    }

    if depth > 0 {
        return _fold(number, depth - 1);
    }

    number
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = parse_input(input);
    let mut number: Option<Vec<(usize, usize)>> = None;

    for rhs in numbers.into_iter() {
        number = Some(match number {
            None => rhs,
            Some(lhs) => add(&lhs, &rhs),
        });
    }

    number.map(|number| magnitude(number))
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_input(input)
        .iter()
        .tuple_combinations()
        .map(|(lhs, rhs)| max(magnitude(add(rhs, lhs)), magnitude(add(lhs, rhs))))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_simple() {
        let number = add(&parse_number("[1,1]"), &parse_number("[2,2]"));
        assert_eq!(number, parse_number("[[1,1],[2,2]]"));

        let number = add(&number, &parse_number("[3,3]"));
        assert_eq!(number, parse_number("[[[1,1],[2,2]],[3,3]]"));

        let number = add(&number, &parse_number("[4,4]"));
        assert_eq!(number, parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

        let number = add(&number, &parse_number("[5,5]"));
        assert_eq!(number, parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]"));

        let number = add(&number, &parse_number("[6,6]"));
        assert_eq!(number, parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_sum() {
        let number = add(
            &parse_number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            &parse_number("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        );
        assert_eq!(number, parse_number("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));

        let number = add(&number, &parse_number("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"));
        assert_eq!(number, parse_number("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"));

        let number = add(&number, &parse_number("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"));
        assert_eq!(number, parse_number("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"));

        let number = add(&number, &parse_number("[7,[5,[[3,8],[1,4]]]]"));
        assert_eq!(number, parse_number("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"));

        let number = add(&number, &parse_number("[[2,[2,2]],[8,[8,1]]]"));
        assert_eq!(number, parse_number("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"));

        let number = add(&number, &parse_number("[2,9]"));
        assert_eq!(number, parse_number("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"));

        let number = add(&number, &parse_number("[1,[[[9,3],9],[[9,0],[0,7]]]]"));
        assert_eq!(number, parse_number("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"));

        let number = add(&number, &parse_number("[[[5,[7,4]],7],1]"));
        assert_eq!(number, parse_number("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"));

        let number = add(&number, &parse_number("[[[[4,2],2],6],[8,7]]"));
        assert_eq!(number, parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_magnitude() {
        assert_eq!(magnitude(parse_number("[9,1]")), 29);
        assert_eq!(magnitude(parse_number("[1,9]")), 21);
        assert_eq!(magnitude(parse_number("[[9,1],[1,9]]")), 129);
        assert_eq!(magnitude(parse_number("[[1,7],4]")), 59);
        assert_eq!(magnitude(parse_number("[3,[1,7]]")), 43);
        assert_eq!(magnitude(parse_number("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(magnitude(parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
        assert_eq!(magnitude(parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(magnitude(parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(magnitude(parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
        assert_eq!(magnitude(parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")), 3488);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(4140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(3993));
    }
}
