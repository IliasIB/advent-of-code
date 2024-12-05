fn bracket_to_change(bracket: char) -> i32 {
    match bracket {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn part_1(brackets: &String) {
    let mut floor = 0;
    for bracket in brackets.chars() {
        floor += bracket_to_change(bracket);
    }
    println!("Part 1: {floor}");
}

fn part_2(brackets: &String) {
    let mut floor = 0;
    let mut counter = 1;
    for bracket in brackets.chars() {
        floor += bracket_to_change(bracket);
        if floor == -1 {
            break;
        }
        counter += 1;
    }
    println!("Part 2: {counter}");
}

pub fn day_1(brackets: &String) {
    part_1(&brackets);
    part_2(&brackets);
}
