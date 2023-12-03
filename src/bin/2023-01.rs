use std::collections::HashMap;

use adventofcode::*;
use regex::Regex;

fn main() {
    // let input = get_input();
    // let mut sum = 0;
    // for line in input.lines() {
    //     let first = || -> u32 {
    //         for c in line.chars() {
    //             if let Ok(i) = String::from(c).parse::<u32>() {
    //                 return i;
    //             }
    //         }
    //         panic!("{line}");
    //     }();
    //     let second = || -> u32 {
    //         for c in line.chars().rev() {
    //             if let Ok(i) = String::from(c).parse::<u32>() {
    //                 return i;
    //             }
    //         }
    //         panic!();
    //     }();
    //     sum += format!("{first}{second}").parse::<u32>().unwrap();
    // }
    // dbg!(&sum);
    let input = get_input();
    let mut sum = 0;
    let str_to_int: HashMap<String, u32> = "[0-9]|one|two|three|four|five|six|seven|eight|nine|ten"
        .split("|")
        .enumerate()
        .map(|(i, s)| (s.to_owned(), i as u32))
        .collect();
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine|ten)").unwrap();
    for line in input.lines() {
        let captures: Vec<(String, usize, usize)> = {
            let mut value: Vec<(String, usize, usize)> = vec![];
            for i in 0..line.len() {
                // take slice of line starting at i
                let slice = &line[i..];
                let intermediary = re.find_iter(slice).collect::<Vec<_>>();
                // Adjust start and end for offset of i
                let intermediary = intermediary
                    .iter()
                    .map(|m| (m.as_str().to_owned(), m.start() + i, m.end() + i))
                    .collect::<Vec<_>>();
                value.extend(intermediary);
            }
            // Sort based on start index, then end index if start index is equal
            value.sort_by(|(a0, a1, a2), (b0, b1, b2)| a1.cmp(&b1));
            value
        };
        // dbg!(&captures);
        let first = || -> u32 {
            let c = captures.first().unwrap().0.clone();
            if let Ok(i) = String::from(c.clone()).parse::<u32>() {
                return i;
            } else {
                return *str_to_int.get(&c).unwrap();
            }
        }();
        let second = || -> u32 {
            let c = captures.last().unwrap().0.clone();
            if let Ok(i) = String::from(c.clone()).parse::<u32>() {
                return i;
            } else {
                return *str_to_int.get(&c).unwrap();
            }
        }();
        // println!("{first}{second}");
        sum += format!("{first}{second}").parse::<u32>().unwrap();
    }
    dbg!(&sum);
}
