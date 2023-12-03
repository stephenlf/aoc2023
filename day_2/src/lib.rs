use anyhow::{anyhow, Result};
use aoc2023::LinesIter;

pub fn get_total_power(games: &[Game]) -> u32 {
    games.iter()
        .map(|game| game.power_of_max())
        .sum()
}

pub fn get_games(lines: LinesIter) -> Result<Vec<Game>> {
    lines.map(|line| {
        match line {
            Ok(l) => Game::try_from(l.as_str()),
            Err(e) => Err(anyhow!(e))
        }
    }).collect()
}

pub fn find_possible_game_ids(games: &[Game]) -> Vec<usize> {
    games.iter()
        .filter_map(|game| if game.is_possible() {
            Some(game.id)
        } else {
            None
        }).collect()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Round {
    red: u16,
    green: u16,
    blue: u16,
}

impl Game {
    pub fn power_of_max(&self) -> u32 {
        let max = self.max_of_rounds();
        max.blue as u32
            * max.red as u32
            * max.green as u32
    }

    /// Calculate the minimum number of cubes required for the game to be 
    /// possible.
    fn max_of_rounds(&self) -> Round {
        let mut max_round = Round::from_rgb(0, 0, 0);
        for round in &self.rounds {
            if round.red > max_round.red {
                max_round.red = round.red;
            }
            if round.green > max_round.green {
                max_round.green = round.green;
            }
            if round.blue > max_round.blue {
                max_round.blue = round.blue;
            }
        }
        max_round
    }

    /// Check that the game had no invalid rounds, given the constraints of the
    /// problem
    pub fn is_possible(&self) -> bool {
        for round in &self.rounds {
            if !round.is_possible() {
                return false;
            }
        }
        true
    }

    /// Parse game id from string of form "Game #"
    fn get_game_id(s: &str) -> Result<usize> {
        if &s[0..5] != "Game " {
            Err(anyhow!("could not find 'Game' token"))
        } else {
            let game_id = (&s[5..]).parse()?;
            Ok(game_id)
        }
    }

    /// Parse Rounds for string of form "n blue, m green, p red; [etc...]"
    fn get_rounds(s: &str) -> Result<Vec<Round>> {
        s.split(';')
            .map(|round| Round::try_from(round))
            .collect::<Result<Vec<Round>>>()
    }
}

impl Round {
    /// Check if the round was possible given the constraints of the problem
    fn is_possible(&self) -> bool {
        self.red <= 12
            && self.green <= 13
            && self.blue <= 14
    }

    pub fn from_rgb(red: u16, green: u16, blue: u16) -> Self {
        Self { red, green, blue }
    }
}

impl TryFrom::<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let (game_id, rounds) = value.split_once(':')
            .ok_or(anyhow!("missing ':' delimiter"))?;

        let id = Self::get_game_id(game_id)?;
        let rounds = Self::get_rounds(rounds)?;

        Ok(Self { id, rounds })
    }
}

impl TryFrom::<&str> for Round {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut round = Round::default();

        for color_value in value.split(',') {
            let (value, color) = color_value.trim()
                .split_once(' ')
                .ok_or(anyhow!("failed to find space-delimited color/value pair"))?;
            match color {
                "red" => round.red = value.parse()?,
                "green" => round.green = value.parse()?,
                "blue" => round.blue = value.parse()?,
                _ => return Err(anyhow!("unexpected color token")),
            }
        }

        Ok(round)
    }
}

#[cfg(test)]
mod day_2 {
    use super::*;

    #[test]
    fn get_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let test_game = Game::try_from(input).unwrap();
        let game = Game {
            id: 1, 
            rounds: vec![
                Round::from_rgb(4, 0, 3),
                Round::from_rgb(1, 2, 6),
                Round::from_rgb(0, 2, 0),
            ]};
        assert_eq!(game, test_game);
    }

    #[test]
    fn mock_part_1() {
        let input = aoc2023::read_as_lines("../inputs/day_2/mock");
        let games = get_games(input).unwrap();
        let possible_ids = find_possible_game_ids(&games);
        let sum = possible_ids.into_iter().sum::<usize>();
        assert_eq!(sum, 8);
    }
}