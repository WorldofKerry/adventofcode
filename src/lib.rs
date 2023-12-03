use std::env;

fn get_year_day() -> (u32, u32) {
    // target/debug/####-##
    let args: Vec<String> = env::args().collect();
    let tail = args[0].split("/").last().unwrap();
    let year_day = tail.split("-").collect::<Vec<&str>>();
    let year = year_day[0].parse::<u32>().unwrap();
    let day = year_day[1].parse::<u32>().unwrap();
    (year, day)
}

fn get_input_from_date(year: u32, day: u32) -> String {
    // Opens file at inputs/{year}/{day}.txt
    let path = &format!("inputs/{year}/{day}.txt");
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            // Make empty file if not found
            match std::fs::write(path, "") {
                Ok(_) => panic!("Paste data to {}", path),
                Err(err) => panic!("{}", err),
            }
        }
    }
}

pub fn get_input() -> String {
    let (year, day) = get_year_day();
    get_input_from_date(year, day)
}
