use std::env::args;

use aoc::*;

fn main() {
    let (year, day, test_mode) = parse_args();
    let contents = util::read_lines(&year, &day, test_mode);
    match year {
        2015 => year2015(&day, &contents),
        2024 => year2024(&day, &contents),
        _ => panic!("Given year does not exist"),
    }
}

fn parse_args() -> (u16, u8, bool) {
    let year = args()
        .nth(1)
        .expect("No year given")
        .parse::<u16>()
        .expect("Not a valid year");
    let day = args()
        .nth(2)
        .expect("No day given")
        .parse::<u8>()
        .expect("Not a valid day");
    let test_mode = if let Some(_) = args().nth(3) {
        true
    } else {
        false
    };
    (year, day, test_mode)
}

fn year2015(day: &u8, contents: &String) {
    match day {
        1 => year2015::day01::day_1(contents),
        _ => panic!("Given day does not exist"),
    }
}

fn year2024(day: &u8, contents: &String) {
    match day {
        1 => year2024::day01::day1(contents),
        2 => year2024::day02::day2(contents),
        // 3 => year2024::day03::day3(contents),
        // 4 => year2024::day04::day4(contents),
        // 5 => year2024::day05::day5(contents),
        // 6 => year2024::day06::day6(contents),
        // 7 => year2024::day07::day7(contents),
        // 8 => year2024::day08::day8(contents),
        // 9 => year2024::day09::day9(contents),
        // 10 => year2024::day10::day10(contents),
        11 => year2024::day11::day11(contents),
        12 => year2024::day12::day12(contents),
        13 => year2024::day13::day13(contents),
        14 => year2024::day14::day14(contents),
        15 => year2024::day15::day15(contents),
        16 => year2024::day16::day16(contents),
        17 => year2024::day17::day17(contents),
        18 => year2024::day18::day18(contents),
        19 => year2024::day19::day19(contents),
        20 => year2024::day20::day20(contents),
        21 => year2024::day21::day21(contents),
        22 => year2024::day22::day22(contents),
        // 23 => year2024::day23::day23(contents),
        // 24 => year2024::day24::day24(contents),
        // 25 => year2024::day25::day25(contents),
        _ => panic!("Given day does not exist"),
    }
}
