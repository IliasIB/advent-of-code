use std::fs;

pub fn read_lines(day: u8) -> String {
    let dir = format!("inputs/day{}.txt", day);
    let contents = fs::read_to_string(&dir).expect("Should have been able to read the file");
    return contents;
}
