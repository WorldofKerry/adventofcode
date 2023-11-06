use super::*;

#[test]
fn main() {
    let contents = read_input("d1.txt");

    let mut total = 0;
    let mut sums: Vec<u32> = vec![];

    for line in contents.lines() {
        println!("line: {}", line);
        if line.is_empty() {
            sums.push(total);
            total = 0;
        } else {
            total += str::parse::<u32>(line).unwrap();
        }
    }
    sums.sort_by(|a, b| b.cmp(a));

    println!("With text:\n{}", sums[0] + sums[1] + sums[2]);
}
