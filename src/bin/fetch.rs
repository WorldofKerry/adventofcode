use std::env;

use ureq::Error;

/// Gets input via http
fn get_input(year: u32, day: u32) -> Result<String, Error> {
    let body = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .call()?
    .into_string()?;
    Ok(body)
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => match args[1].parse::<u32>() {
            Ok(y) => match args[2].parse::<u32>() {
                Ok(d) => {
                    println!("Fetching input for year {} day {}", y, d);
                    let input = get_input(y, d).unwrap();
                    println!("{}", input);
                }
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!("{:?}", args),
    };
}
