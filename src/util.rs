use std::fs;

pub fn read_lines(year: &u16, day: &u8, test_mode: bool) -> String {
    let mode = if test_mode { "samples" } else { "inputs" };
    let dir = format!("{}/{}/{:02}.txt", mode, year, day);
    let contents = fs::read_to_string(&dir).expect("Should have been able to read the file");
    return contents;
}
