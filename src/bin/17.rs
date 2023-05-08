use itertools::Itertools;

fn parse_input(input: &str) -> ((isize, isize), (isize, isize)) {
    let mut data = input.split(", ");
    let x: (isize, isize) = data
        .next()
        .unwrap()
        .split("target area: x=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|element| element.parse().unwrap())
        .collect_tuple()
        .unwrap();

    let y: (isize, isize) = data
        .next()
        .unwrap()
        .split("y=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|element| element.parse().unwrap())
        .collect_tuple()
        .unwrap();

    (x, y)
}

pub fn part_one(input: &str) -> Option<isize> {
    let (_, (y, _)) = parse_input(input);

    Some((y * (y + 1)) / 2)
}

fn will_hit_target(
    vx: isize,
    vy: isize,
    x: isize,
    y: isize,
    target: ((isize, isize), (isize, isize)),
) -> bool {
    let ((tx1, tx2), (ty1, ty2)) = target;

    if x > tx2 {
        return false;
    }

    if y < ty1 {
        return false;
    }

    if x >= tx1 && y <= ty2 {
        return true;
    }

    will_hit_target(vx - vx.signum(), vy - 1, x + vx, y + vy, target)
}

pub fn part_two(input: &str) -> Option<usize> {
    let target = parse_input(input);
    let ((_, x2), (y1, _)) = target;

    Some(
        (1..=x2)
            .map(|x| {
                (y1..-y1)
                    .filter(|y| will_hit_target(x, *y, 0, 0, target))
                    .count()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(45));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(112));
    }
}
