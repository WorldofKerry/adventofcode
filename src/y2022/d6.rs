use std::collections::VecDeque;

#[test]
fn main() {
    use super::*;

    let contents = read_input("d6.txt");

    let mut lines = contents.lines();

    while let Some(first) = lines.next() {
        let mut chars = first.chars().enumerate();

        let mut visited: VecDeque<char> = VecDeque::new();
        while let Some(pair) = chars.next() {
            if visited.len() == 13 && !visited.contains(&pair.1) {
                println!("{} {:?}", pair.0 + 1, visited);
                return;
            }
            // println!("{} {:?}", pair.1, visited);
            while visited.len() == 14 || visited.contains(&pair.1) {
                visited.pop_front();
            }
            // println!("{} {:?}", pair.1, visited);
            visited.push_back(pair.1);
        }
    }
}
