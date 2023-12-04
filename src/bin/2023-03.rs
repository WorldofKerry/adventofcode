use std::collections::{HashMap, HashSet};

use adventofcode::*;
use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Data {
    Value(usize),
    None,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Coord(usize, usize);

/// Given a point, get a vector of all its neighbours
/// Includes diagonal neighbours
fn neighbours(
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let xs = std::cmp::max(1, x) - 1..=std::cmp::min(x + 1, x_max - 1);
    let ys = std::cmp::max(1, y) - 1..=std::cmp::min(y + 1, y_max - 1);
    xs.flat_map(move |x_| ys.clone().map(move |y_| (x_, y_)))
        .filter(move |&(x_, y_)| x_ != x || y_ != y)
}

fn part2() {
    let input = get_input();

    // Populate numbers
    let mut numbers = vec![];
    let mut coord_to_index = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let re = Regex::new(r"([0-9]+)").unwrap();
        for capture in re.captures_iter(line) {
            let (_, [value]) = capture.extract();
            let start = capture.get(1).unwrap().start();
            let end = capture.get(1).unwrap().end();
            let value = value.parse::<usize>().unwrap();
            numbers.push(value);
            for idx in start..end {
                coord_to_index.insert(Coord { 0: i, 1: idx }, numbers.len() - 1);
            }
        }
    }

    // Populate symbols
    let mut symbols = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let re = Regex::new(r"([^[0-9].])").unwrap();
        for capture in re.captures_iter(line) {
            let (_, [value]) = capture.extract();
            let start = capture.get(1).unwrap().start();
            symbols.insert(Coord { 0: i, 1: start }, value.to_owned());
        }
    }

    let (line_width, line_height) = {
        let mut line_width = 0;
        let mut line_height = 0;
        for line in input.lines() {
            line_width = line_width.max(line.len());
            line_height += 1;
        }
        (line_width, line_height)
    };

    // Find symbols with exactly two number neighbours
    // Sum up the product of the numbers
    let mut sum_of_gears = 0;
    for symbol in symbols.keys() {
        let neighbours = neighbours(symbol.0, symbol.1, line_width, line_height);

        let mut neighbour_indices = HashSet::new();
        for neighbour in neighbours {
            if let Some(idx) = coord_to_index.get(&Coord {
                0: neighbour.0,
                1: neighbour.1,
            }) {
                neighbour_indices.insert(*idx);
            }
        }

        if let [idx1, idx2] = neighbour_indices.iter().collect::<Vec<_>>()[..] {
            sum_of_gears += numbers[*idx1] * numbers[*idx2];
        }
    }

    dbg!(&sum_of_gears);
}

fn part1() {
    let input = get_input();

    // Populate numbers
    let mut numbers = vec![];
    let mut coord_to_index = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let re = Regex::new(r"([0-9]+)").unwrap();
        for capture in re.captures_iter(line) {
            let (_, [value]) = capture.extract();
            let start = capture.get(1).unwrap().start();
            let end = capture.get(1).unwrap().end();
            let value = value.parse::<usize>().unwrap();
            numbers.push(value);
            for idx in start..end {
                coord_to_index.insert(Coord { 0: i, 1: idx }, numbers.len() - 1);
            }
        }
    }

    // Populate symbols
    let mut symbols = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let re = Regex::new(r"([^[0-9].])").unwrap();
        for capture in re.captures_iter(line) {
            let (_, [value]) = capture.extract();
            let start = capture.get(1).unwrap().start();
            symbols.insert(Coord { 0: i, 1: start }, value.to_owned());
        }
    }

    let (line_width, line_height) = {
        let mut line_width = 0;
        let mut line_height = 0;
        for line in input.lines() {
            line_width = line_width.max(line.len());
            line_height += 1;
        }
        (line_width, line_height)
    };

    // Mark visited numbers
    let mut visited = vec![false; numbers.len()];
    for symbol in symbols.keys() {
        let neighbours = neighbours(symbol.0, symbol.1, line_width, line_height);
        for neighbour in neighbours {
            if let Some(idx) = coord_to_index.get(&Coord {
                0: neighbour.0,
                1: neighbour.1,
            }) {
                visited[*idx] = true;
            }
        }
    }

    // Take sum of visited numbers
    let sum = {
        let mut sum = 0;
        for (i, v) in visited.iter().enumerate() {
            if *v {
                sum += numbers[i];
            }
        }
        sum
    };
    dbg!(&sum);
}

fn main() {
    part2();
}
