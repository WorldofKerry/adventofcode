use super::*;

fn char_to_priority(c: char) -> u8 {
    let val = c as u8;
    if val < 'a' as u8 {
        return val - 'A' as u8 + 27;
    } else {
        return val - 'a' as u8 + 1;
    }
}

#[test]
fn main() {
    use super::*;

    let contents = read_input("d3.txt");

    let mut total: u32 = 0;

    // for line in contents.lines() {
    //     let len = line.len();
    //     let first = &line[..len / 2];
    //     let second = &line[len / 2..];

    //     for a in first.chars() {
    //         if second.contains(a) {
    //             let score = char_to_priority(a);
    //             total += score as u32;
    //             println!("match {} {}", a, score);
    //             break;
    //         }
    //     }
    //     println!("{} -> {}", first, second)
    // }

    let mut lines = contents.lines();

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        for a in first.chars() {
            if second.contains(a) && third.contains(a) {
                let score = char_to_priority(a);
                total += score as u32;
                break;
            }
        }
    }

    println!("total {}", total)
}
