type Coordinate = (usize, usize);
type Direction = (isize, isize);

struct RaceMap {
    start: Coordinate,
    end: Coordinate,
    map: Vec<Vec<char>>,
}

fn parse(contents: &str) -> (usize, RaceMap) {
    let contents_clean = contents.replace('\r', "");
    let contents_vec: Vec<&str> = contents_clean.split("\n\n").collect();

    let time_saved_minimum = contents_vec[0]
        .parse::<usize>()
        .expect("Invalid Cheat number");
    let map = contents_vec[1]
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                start = (x, y);
            } else if map[y][x] == 'E' {
                end = (x, y);
            }
        }
    }
    let race_map = RaceMap { start, end, map };
    return (time_saved_minimum, race_map);
}

fn get_race_track(race_map: &RaceMap) -> Vec<Coordinate> {
    let mut race_track: Vec<Coordinate> = Vec::new();
    let directions: Vec<Direction> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_node = race_map.start;

    while current_node != race_map.end {
        race_track.push(current_node);

        for direction in &directions {
            let new_node = (
                current_node.0.wrapping_add_signed(direction.0),
                current_node.1.wrapping_add_signed(direction.1),
            );

            match race_map.map[new_node.1][new_node.0] {
                '.' | 'E' => {
                    if !race_track.contains(&new_node) {
                        current_node = new_node;
                        break;
                    }
                }
                '#' => (),
                _ => unreachable!(),
            }
        }
    }
    race_track.push(race_map.end);

    return race_track;
}

fn part1(time_saved_minimum: &usize, race_map: &RaceMap) {
    let race_track = get_race_track(race_map);

    let mut cheat_amount = 0;
    let mut cheats: Vec<(Coordinate, Coordinate)> = Vec::new();
    for i in 0..(race_track.len() - (time_saved_minimum + 2)) {
        for j in (i + (*time_saved_minimum + 2))..race_track.len() {
            if (race_track[i].0.abs_diff(race_track[j].0))
                + (race_track[i].1.abs_diff(race_track[j].1))
                == 2
            {
                cheats.push((race_track[i], race_track[j]));
                cheat_amount += 1;
            }
        }
    }

    println!("Part 1: {cheat_amount}");
}

fn calculate_time_saved(start: &usize, end: &usize, distance: &usize) -> usize {
    let time_saved = (*end as isize) - (*start as isize) - (*distance as isize);
    if time_saved > 0 {
        return time_saved as usize;
    }
    return 0;
}

fn part2(time_saved_minimum: &usize, race_map: &RaceMap) {
    let race_track = get_race_track(race_map);

    let mut cheat_amount = 0;
    for i in 0..race_track.len() {
        for j in 0..race_track.len() {
            let distance = race_track[i].0.abs_diff(race_track[j].0)
                + race_track[i].1.abs_diff(race_track[j].1);
            if distance < 21 && calculate_time_saved(&i, &j, &distance) >= *time_saved_minimum {
                cheat_amount += 1;
            }
        }
    }

    println!("Part 2: {cheat_amount}");
}

pub fn day20(contents: &str) {
    let (time_saved_minimum, race_map) = parse(contents);
    part1(&time_saved_minimum, &race_map);
    part2(&time_saved_minimum, &race_map);
}
