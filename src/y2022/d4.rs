use super::*;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

/// a-b,c-d
/// where a, b, c, d are numbers
fn parse_line(line: &str) -> (Range, Range) {
    let mut ranges = line.split(',');
    let first = ranges.next().unwrap();
    let second = ranges.next().unwrap();

    let mut first = first.split('-');
    let mut second = second.split('-');

    let first_start = first.next().unwrap().parse::<u32>().unwrap();
    let first_end = first.next().unwrap().parse::<u32>().unwrap();

    let second_start = second.next().unwrap().parse::<u32>().unwrap();
    let second_end = second.next().unwrap().parse::<u32>().unwrap();

    (
        Range {
            start: first_start,
            end: first_end,
        },
        Range {
            start: second_start,
            end: second_end,
        },
    )
}

fn check_inclusive(a: &Range, b: &Range) -> bool {
    if a.start >= b.start && a.end <= b.end {
        return true;
    }
    if b.start >= a.start && b.end <= a.end {
        return true;
    }
    false
}

fn check_overlap(a: &Range, b: &Range) -> bool {
    if a.end >= b.start && a.end <= b.end {
        return true;
    }
    if b.end >= a.start && b.end <= a.end {
        return true;
    }
    false
}

#[test]
fn main() {
    use super::*;

    let contents = read_input("d4.txt");

    let mut lines = contents.lines();

    let mut counter = 0;

    while let Some(first) = lines.next() {
        let ranges = parse_line(first);
        let overlap = check_overlap(&ranges.0, &ranges.1);
        println!("{:?} {:?} {}", ranges.0, ranges.1, overlap);
        if overlap {
            counter += 1;
        }
    }

    println!("{}", counter)
}
