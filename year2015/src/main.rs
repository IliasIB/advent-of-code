use std::env::args;

use day1::day_1;
use day2::day_2;
use util::read_lines;

mod day1;
mod day2;
mod util;

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].parse() {
        Ok(day) if (1u8..=24).contains(&day) => {
            println!("Running day {}", { day });
            let contents = read_lines(day);
            match day {
                1 => day_1(&contents),
                2 => day_2(&contents),
                _ => unreachable!(),
            }
        }
        _ => {
            println!("Provide day argument between 1 and 24");
        }
    }
}

#[test]
fn all_files_are_ok_open_test() {
    for day in 1..=24 {
        drop(read_lines(day));
    }
}
