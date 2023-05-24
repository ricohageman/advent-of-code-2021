use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Player {
    pub position: usize,
    pub score: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    pub fn step(&mut self, amount: usize) {
        self.position += amount % 10;

        if self.position > 10 {
            self.position -= 10;
        }

        self.score += self.position;
    }
}

struct DeterministicDice {
    rolls: usize,
    current: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            rolls: 0,
            current: 1,
        }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        let value = self.current;

        self.current = match self.current {
            100 => 1,
            value => value + 1,
        };

        value
    }
}

fn parse_input(input: &str) -> (Player, Player) {
    input
        .lines()
        .map(|line| Player::new(line.split(": ").skip(1).next().unwrap().parse().unwrap()))
        .collect_tuple()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut player_1, mut player_2) = parse_input(input);

    let mut dice = DeterministicDice::new();
    let mut game_ended = false;
    let mut player_ones_turn = true;

    while !game_ended {
        let dice_result = dice.roll() + dice.roll() + dice.roll();

        match player_ones_turn {
            true => {
                player_1.step(dice_result);
                game_ended = player_1.score >= 1000;
            }
            false => {
                player_2.step(dice_result);
                game_ended = player_2.score >= 1000;
            }
        };

        player_ones_turn = !player_ones_turn;
    }

    Some(min(player_1.score, player_2.score) * dice.rolls)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut player_1, mut player_2) = parse_input(input);

    let possible_outcomes = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut wins: (usize, usize) = (0, 0);
    let mut queue = VecDeque::new();
    queue.push_front((player_1, player_2, true, 1));

    while let Some((player_1, player_2, player_ones_turn, cases)) = queue.pop_front() {
        if player_1.score >= 21 {
            wins.0 += cases;
            continue;
        }

        if player_2.score >= 21 {
            wins.1 += cases;
            continue;
        }

        for (dice_result, frequency) in possible_outcomes {
            let mut player_1 = player_1.clone();
            let mut player_2 = player_2.clone();

            match player_ones_turn {
                true => player_1.step(dice_result),
                false => player_2.step(dice_result),
            }

            queue.push_front((player_1, player_2, !player_ones_turn, cases * frequency));
        }
    }

    Some(max(wins.0, wins.1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(739785));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(444356092776315));
    }
}
