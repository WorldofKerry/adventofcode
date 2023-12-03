use std::collections::HashMap;

use adventofcode::*;
use regex::Regex;

fn main() {
    {
        let input = get_input();
        let mut sum = 0;
        for line in input.lines() {
            let first = || -> u32 {
                for c in line.chars() {
                    if let Ok(i) = String::from(c).parse::<u32>() {
                        return i;
                    }
                }
                panic!();
            }();
            let second = || -> u32 {
                for c in line.chars().rev() {
                    if let Ok(i) = String::from(c).parse::<u32>() {
                        return i;
                    }
                }
                panic!();
            }();
            sum += format!("{first}{second}").parse::<u32>().unwrap();
        }
        dbg!(&sum);
    }
    {
        let input = get_input();
        let mut sum = 0;
        let str_to_int: HashMap<String, u32> =
            "[0-9]|one|two|three|four|five|six|seven|eight|nine|ten"
                .split("|")
                .enumerate()
                .map(|(i, s)| (s.to_owned(), i as u32))
                .collect();
        let re =
            Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine|ten)").unwrap();
        for line in input.lines() {
            let captures = re.find_iter(line).collect::<Vec<_>>();
            dbg!(&captures);
            break;
        }
        dbg!(&sum);
    }
}
