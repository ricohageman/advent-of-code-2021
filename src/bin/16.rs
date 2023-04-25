struct BinaryStream {
    binary: Vec<bool>,
    pointer: usize,
}

impl BinaryStream {
    pub fn from_hexadecimal(input: &str) -> Self {
        Self::from_binary(hexadecimal_to_binary(input))
    }

    pub fn from_binary(input: Vec<bool>) -> Self {
        Self {
            binary: input,
            pointer: 0,
        }
    }

    pub fn to_decimal(&self) -> usize {
        self.binary
            .iter()
            .rev()
            .enumerate()
            .filter(|(_, bool)| **bool)
            .map(|(index, _)| 2_usize.pow(index as u32))
            .sum()
    }

    pub fn to_bool(&self) -> bool {
        match self.to_decimal() {
            0 => false,
            1 => true,
            _ => panic!(),
        }
    }

    pub fn read_next(&mut self, size: usize) -> BinaryStream {
        let result = Self::from_binary(
            self.binary
                .iter()
                .skip(self.pointer)
                .take(size)
                .cloned()
                .collect(),
        );
        self.pointer += size;
        result
    }

    pub fn has_next(&self) -> bool {
        self.binary.iter().skip(self.pointer).len() >= 11
    }
}

fn hexadecimal_to_binary(input: &str) -> Vec<bool> {
    input
        .split("")
        .filter(|element| !element.is_empty())
        .flat_map(|char| match char {
            "0" => vec![0, 0, 0, 0],
            "1" => vec![0, 0, 0, 1],
            "2" => vec![0, 0, 1, 0],
            "3" => vec![0, 0, 1, 1],
            "4" => vec![0, 1, 0, 0],
            "5" => vec![0, 1, 0, 1],
            "6" => vec![0, 1, 1, 0],
            "7" => vec![0, 1, 1, 1],
            "8" => vec![1, 0, 0, 0],
            "9" => vec![1, 0, 0, 1],
            "A" => vec![1, 0, 1, 0],
            "B" => vec![1, 0, 1, 1],
            "C" => vec![1, 1, 0, 0],
            "D" => vec![1, 1, 0, 1],
            "E" => vec![1, 1, 1, 0],
            "F" => vec![1, 1, 1, 1],
            _ => panic!("{}", char),
        })
        .map(|element| element != 0)
        .collect()
}

#[derive(Debug, PartialEq)]
enum Package {
    Literal {
        version: usize,
        literal: usize,
    },
    Operation {
        version: usize,
        operator_type: usize,
        content: Vec<Package>,
    },
}

impl Package {
    pub fn version_sum(&self) -> usize {
        match self {
            Package::Literal { version, .. } => *version,
            Package::Operation {
                version, content, ..
            } => {
                *version
                    + content
                        .iter()
                        .map(|package| package.version_sum())
                        .sum::<usize>()
            }
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Package::Literal { literal, .. } => *literal,
            Package::Operation {
                operator_type,
                content,
                ..
            } => {
                let mut values = content.iter().map(|package| package.value());

                match operator_type {
                    0 => values.sum::<usize>(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => (values.next().unwrap() > values.next().unwrap()) as usize,
                    6 => (values.next().unwrap() < values.next().unwrap()) as usize,
                    7 => (values.next().unwrap() == values.next().unwrap()) as usize,
                    _ => panic!(),
                }
            }
        }
    }
}

fn parse_packages(binary: &mut BinaryStream) -> Package {
    let packet_version = binary.read_next(3).to_decimal();
    let packet_type = binary.read_next(3).to_decimal();

    match packet_type {
        4 => {
            let mut literal_groups: Vec<BinaryStream> = Vec::new();
            let mut final_literal_found = false;

            while !final_literal_found {
                final_literal_found = !binary.read_next(1).to_bool();
                literal_groups.push(binary.read_next(4))
            }

            Package::Literal {
                version: packet_version,
                literal: BinaryStream::from_binary(
                    literal_groups
                        .into_iter()
                        .flat_map(|binary| binary.binary)
                        .collect::<Vec<bool>>(),
                )
                .to_decimal(),
            }
        }
        _ => match binary.read_next(1).to_bool() {
            false => {
                let content_length = binary.read_next(15).to_decimal();
                let mut content_binary = binary.read_next(content_length);
                let mut content: Vec<Package> = Vec::new();

                while content_binary.has_next() {
                    content.push(parse_packages(&mut content_binary));
                }

                Package::Operation {
                    version: packet_version,
                    operator_type: packet_type,
                    content,
                }
            }
            true => {
                let number_of_packages = binary.read_next(11).to_decimal();
                let mut content: Vec<Package> = Vec::new();

                while content.len() != number_of_packages {
                    content.push(parse_packages(binary));
                }

                Package::Operation {
                    version: packet_version,
                    operator_type: packet_type,
                    content,
                }
            }
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut binary = BinaryStream::from_hexadecimal(input);
    let package = parse_packages(&mut binary);

    Some(package.version_sum() as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut binary = BinaryStream::from_hexadecimal(input);
    let package = parse_packages(&mut binary);

    Some(package.value())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal_to_binary() {
        assert_eq!(
            hexadecimal_to_binary("D2FE28")
                .into_iter()
                .map(|element| element as u32)
                .collect::<Vec<_>>(),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn test_parse_package_literal_1() {
        let mut binary_stream = BinaryStream::from_hexadecimal("D2FE28");
        let package = parse_packages(&mut binary_stream);
        assert_eq!(
            package,
            Package::Literal {
                version: 6,
                literal: 2021
            }
        );
    }

    #[test]
    fn test_parse_package_literal_2() {
        let mut binary_stream = BinaryStream::from_hexadecimal("38006F45291200");
        let package = parse_packages(&mut binary_stream);

        assert_eq!(
            package,
            Package::Operation {
                version: 1,
                operator_type: 6,
                content: vec![
                    Package::Literal {
                        version: 6,
                        literal: 10
                    },
                    Package::Literal {
                        version: 2,
                        literal: 20
                    },
                ]
            }
        );
    }

    #[test]
    fn test_parse_package_literal_3() {
        let mut binary_stream = BinaryStream::from_hexadecimal("EE00D40C823060");
        let package = parse_packages(&mut binary_stream);

        assert_eq!(
            package,
            Package::Operation {
                version: 7,
                operator_type: 3,
                content: vec![
                    Package::Literal {
                        version: 2,
                        literal: 1
                    },
                    Package::Literal {
                        version: 4,
                        literal: 2
                    },
                    Package::Literal {
                        version: 1,
                        literal: 3
                    },
                ]
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("8A004A801A8002F478"), Some(16));
        assert_eq!(part_one("620080001611562C8802118E34"), Some(12));
        assert_eq!(part_one("C0015000016115A2E0802F182340"), Some(23));
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), Some(31));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("C200B40A82"), Some(3));
        assert_eq!(part_two("04005AC33890"), Some(54));
        assert_eq!(part_two("880086C3E88112"), Some(7));
        assert_eq!(part_two("CE00C43D881120"), Some(9));
        assert_eq!(part_two("D8005AC2A8F0"), Some(1));
        assert_eq!(part_two("F600BC2D8F"), Some(0));
        assert_eq!(part_two("9C005AC2F8F0"), Some(0));
        assert_eq!(part_two("9C0141080250320F1802104A08"), Some(1));
    }
}
