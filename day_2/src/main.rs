use day_2::{get_games, find_possible_game_ids, get_total_power};
use aoc2023::read_as_lines;

fn main() {
    let input = read_as_lines("../inputs/day_2/input");
    let games = get_games(input).unwrap();
    let possible_ids = find_possible_game_ids(&games);
    println!("Part 1: {}", possible_ids.into_iter().sum::<usize>());
    let total_power = get_total_power(&games);
    println!("Part 2: {total_power}");
}

