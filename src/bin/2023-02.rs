use std::collections::HashMap;

use adventofcode::*;
use regex::Regex;

#[derive(Debug, Default)]
struct Item {
    count: usize,
    color: String,
}

#[derive(Debug, Default)]
struct Move {
    items: Vec<Item>,
}

#[derive(Debug, Default)]
struct Game {
    moves: Vec<Move>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|s| parse_line(s)).collect()
}
fn parse_line(line: &str) -> Game {
    // Remove prefix in form of "Game x: "
    let (_, line) = line.split_once(": ").unwrap();
    let moves = line.split(";").collect::<Vec<&str>>();
    Game {
        moves: moves.iter().map(|m| parse_move(m)).collect(),
    }
}

fn parse_move(line: &str) -> Move {
    // dbg!(&line);
    let re = Regex::new(r"([0-9]+) ([a-z]+)").unwrap();
    let mut moves = vec![];
    for (_, [count, color]) in re.captures_iter(line).map(|c| c.extract()) {
        moves.push(Item {
            count: count.parse::<usize>().unwrap(),
            color: color.to_owned(),
        });
    }
    Move { items: moves }
}
fn max_counts_in_game(game: &Game) -> HashMap<String, usize> {
    // Max of counts in each game
    let mut max = HashMap::new();
    for m in &game.moves {
        for i in &m.items {
            let count = max.entry(i.color.clone()).or_insert(0);
            *count = (*count).max(i.count);
        }
    }
    max
}

fn part2() {
    let input = get_input();
    let games = parse_input(&input);
    let mut answer = 0;

    for game in games.iter() {
        let sum = max_counts_in_game(&game);
        // dbg!(&sum);
        answer += sum.values().product::<usize>();
    }
    dbg!(&answer);
}

fn part1() {
    let input = get_input();
    let games = parse_input(&input);
    let mut answer = 0;

    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let cube_counts: HashMap<&str, usize> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for (i, game) in games.iter().enumerate() {
        let sum = max_counts_in_game(&game);
        // Check if sum is possible with below cube counts
        // dbg!(&sum);
        if cube_counts
            .iter()
            .all(|(color, count)| sum.get(*color).unwrap_or(&usize::MAX) <= count)
        {
            println!("{}", i + 1);
            answer += i + 1;
        }
    }
    dbg!(&answer);
}

fn main() {
    part2();
}
