use std::collections::HashSet;
use std::ops::Range;

const MARGIN: isize = 200;

struct Image {
    pixels_lit: HashSet<(isize, isize)>,
    x_range: (isize, isize),
    y_range: (isize, isize),
}

impl Image {
    fn x_range(&self) -> Range<isize> {
        self.x_range.0 .. self.x_range.1
    }

    fn enlarged_x_range(&self) -> Range<isize> {
        self.x_range.0 - MARGIN .. self.x_range.1 + MARGIN
    }

    fn y_range(&self) -> Range<isize> {
        self.y_range.0 .. self.y_range.1
    }

    fn enlarged_y_range(&self) -> Range<isize> {
        self.y_range.0 - MARGIN .. self.y_range.1 + MARGIN
    }

    fn neighbours(&self, pixel: (isize, isize)) -> impl Iterator<Item=(isize, isize)> {
        let (i, j) = pixel;

        [
            (i - 1, j - 1),
            (i - 0, j - 1),
            (i + 1, j - 1),

            (i - 1, j + 0),
            (i - 0, j + 0),
            (i + 1, j + 0),

            (i - 1, j + 1),
            (i - 0, j + 1),
            (i + 1, j + 1),
        ].into_iter()
    }

    fn enhance_pixel(&self, pixel: (isize, isize), algorithm: &HashSet<usize>) -> bool {
        let binary_index = self.neighbours(pixel)
            .enumerate()
            .filter(|(_, coordinate)| self.pixels_lit.contains(coordinate))
            .map(|(index, _)| 2_u32.pow((9 - index - 1) as u32))
            .sum::<u32>() as usize;

        algorithm.contains(&binary_index)
    }

    pub fn enhance(&self, algorithm: &HashSet<usize>) -> Self {
        let pixels_lit = self.enlarged_x_range()
            .flat_map(|x| self.enlarged_y_range().map(move |y| (x, y)))
            .filter(|pixel| self.enhance_pixel(*pixel, algorithm))
            .collect();
        
        Self {
            pixels_lit,
            x_range: (self.x_range.0 - 2, self.x_range.1 + 2),
            y_range: (self.y_range.0 - 2, self.y_range.1 + 2),
        }
    }

    pub fn number_of_elements_lit(&self) -> usize {
        self.pixels_lit
            .iter()
            .filter(|(x, y)| self.x_range().contains(x) && self.y_range().contains(y))
            .count()
    }
}

fn parse_input(input: &str) -> (HashSet<usize>, Image) {
    let mut input = input.split("\n\n");
    let algorithm: HashSet<usize> = input.next()
        .unwrap()
        .split("")
        .filter(|element| !element.is_empty())
        .enumerate()
        .filter(|(_, element)| element == &"#")
        .map(|(index, _)| index)
        .collect();

    let imagine_input = input.next().unwrap();
    let size = (imagine_input.lines().count() as isize, imagine_input.lines().next().unwrap().len() as isize);

    let pixels_lit: HashSet<(isize, isize)> = imagine_input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split("")
                .filter(|element| !element.is_empty())
                .enumerate()
                .filter(|(_, element)| element == &"#")
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    (algorithm, Image { pixels_lit , x_range: (0, size.0), y_range: (0, size.1)})
}


pub fn part_one(input: &str) -> Option<usize> {
    let (algorithm, image) = parse_input(input);

    let enhanced_image = image.enhance(&algorithm).enhance(&algorithm);

    Some(enhanced_image.number_of_elements_lit())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (algorithm, image) = parse_input(input);
    let mut image = image;

    for _ in 0..50 {
        image = image.enhance(&algorithm)
    }

    Some(image.number_of_elements_lit())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(3351));
    }
}
