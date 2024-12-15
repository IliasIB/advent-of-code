fn parse(contents: &String) -> (Vec<Vec<char>>, Vec<char>) {
    let contents_clean = contents.replace('\r', "");
    let input_vec: Vec<&str> = contents_clean.split("\n\n").collect();
    let map: Vec<Vec<char>> = input_vec[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves: Vec<char> = input_vec[1].replace('\n', "").chars().collect();
    (map, moves)
}

fn get_robot_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

fn is_moveable(
    map: &Vec<Vec<char>>,
    position: &(usize, usize),
    direction: &(isize, isize),
) -> bool {
    let new_position: (usize, usize) = (
        ((position.0 as isize) + direction.0) as usize,
        ((position.1 as isize) + direction.1) as usize,
    );

    match map[new_position.1][new_position.0] {
        '.' => true,
        '#' => false,
        'O' => is_moveable(map, &new_position, direction),
        _ => unreachable!(),
    }
}

fn move_box(map: &mut Vec<Vec<char>>, position: &(usize, usize), direction: &(isize, isize)) {
    let new_position: (usize, usize) = (
        ((position.0 as isize) + direction.0) as usize,
        ((position.1 as isize) + direction.1) as usize,
    );

    if map[new_position.1][new_position.0] == 'O' {
        move_box(map, &new_position, direction);
    }
    map[position.1][position.0] = '.';
    map[new_position.1][new_position.0] = 'O';
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

fn calculate_gps(map: &Vec<Vec<char>>) -> usize {
    let mut gps = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' || map[y][x] == '[' {
                gps += y * 100 + x;
            }
        }
    }
    gps
}

fn part1(map: &Vec<Vec<char>>, moves: &Vec<char>) {
    let mut current_position = get_robot_start(map);
    let mut current_map = map.clone();

    for robot_move in moves {
        let direction = get_direction(robot_move);
        let new_position = get_new_position(current_position, direction);

        if current_map[new_position.1][new_position.0] == '#' {
            continue;
        } else if current_map[new_position.1][new_position.0] == 'O' {
            if is_moveable(&current_map, &new_position, &direction) {
                move_box(&mut current_map, &new_position, &direction);
            } else {
                continue;
            }
        }

        current_map[current_position.1][current_position.0] = '.';
        current_map[new_position.1][new_position.0] = '@';
        current_position = new_position;
    }
    println!("Part 1: {}", calculate_gps(&current_map));
}

fn get_new_position(current_position: (usize, usize), direction: (isize, isize)) -> (usize, usize) {
    let new_position: (usize, usize) = (
        ((current_position.0 as isize) + direction.0) as usize,
        ((current_position.1 as isize) + direction.1) as usize,
    );
    new_position
}

fn get_direction(robot_move: &char) -> (isize, isize) {
    let direction: (isize, isize) = match robot_move {
        '^' => (0, -1),
        'v' => (0, 1),
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => unreachable!(),
    };
    direction
}

fn double_warehouse(warehouse: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_warehouse: Vec<Vec<char>> = Vec::new();
    for line in warehouse {
        let mut new_line: Vec<char> = Vec::new();
        for pos in line {
            match pos {
                '.' => {
                    new_line.push('.');
                    new_line.push('.');
                }
                '#' => {
                    new_line.push('#');
                    new_line.push('#');
                }
                'O' => {
                    new_line.push('[');
                    new_line.push(']');
                }
                '@' => {
                    new_line.push('@');
                    new_line.push('.');
                }
                _ => unreachable!(),
            }
        }
        new_warehouse.push(new_line);
    }
    new_warehouse
}

fn is_moveable_double_vertical(
    warehouse: &Vec<Vec<char>>,
    left: &(usize, usize),
    right: &(usize, usize),
    direction: &(isize, isize),
) -> bool {
    let new_position = get_new_position(*left, *direction);
    let new_position2 = get_new_position(*right, *direction);

    let is_pos1_moveable = match warehouse[new_position.1][new_position.0] {
        '#' => false,
        '.' => true,
        '[' | ']' => is_moveable_double(warehouse, &new_position, direction),
        _ => unreachable!(),
    };
    let is_pos2_moveable = match warehouse[new_position2.1][new_position2.0] {
        '#' => false,
        '.' => true,
        '[' | ']' => is_moveable_double(warehouse, &new_position2, direction),
        _ => unreachable!(),
    };

    return is_pos1_moveable && is_pos2_moveable;
}

fn is_moveable_double_horizontal(
    warehouse: &Vec<Vec<char>>,
    position: &(usize, usize),
    direction: &(isize, isize),
) -> bool {
    let new_position = get_new_position(*position, *direction);

    return match warehouse[new_position.1][new_position.0] {
        '#' => false,
        '.' => true,
        '[' | ']' => is_moveable_double(warehouse, &new_position, direction),
        _ => unreachable!(),
    };
}

fn is_moveable_double(
    warehouse: &Vec<Vec<char>>,
    position: &(usize, usize),
    direction: &(isize, isize),
) -> bool {
    let (left, right) = get_left_right(warehouse, position);
    return match direction {
        (0, -1) | (0, 1) => is_moveable_double_vertical(warehouse, &left, &right, direction),
        (1, 0) => is_moveable_double_horizontal(warehouse, &right, direction),
        (-1, 0) => is_moveable_double_horizontal(warehouse, &left, direction),
        _ => unreachable!(),
    };
}

fn move_double_vertical(
    warehouse: &mut Vec<Vec<char>>,
    left: &(usize, usize),
    right: &(usize, usize),
    direction: &(isize, isize),
) {
    let new_left = get_new_position(*left, *direction);
    let new_right = get_new_position(*right, *direction);

    if warehouse[new_left.1][new_left.0] == '[' || warehouse[new_left.1][new_left.0] == ']' {
        move_double(warehouse, &new_left, direction);
    }
    if warehouse[new_right.1][new_right.0] == '[' || warehouse[new_right.1][new_right.0] == ']' {
        move_double(warehouse, &new_right, direction);
    }

    warehouse[new_left.1][left.0] = '[';
    warehouse[new_right.1][right.0] = ']';
    warehouse[left.1][left.0] = '.';
    warehouse[right.1][right.0] = '.';
}

fn move_double_left(
    warehouse: &mut Vec<Vec<char>>,
    left: &(usize, usize),
    right: &(usize, usize),
    direction: &(isize, isize),
) {
    let new_position = (left.0 - 1, left.1);

    if warehouse[new_position.1][new_position.0] == ']' {
        move_double(warehouse, &new_position, direction);
    }

    warehouse[left.1][left.0 - 1] = '[';
    warehouse[right.1][right.0 - 1] = ']';
    warehouse[right.1][right.0] = '.';
}

fn move_double_right(
    warehouse: &mut Vec<Vec<char>>,
    left: &(usize, usize),
    right: &(usize, usize),
    direction: &(isize, isize),
) {
    let new_position = (right.0 + 1, right.1);

    if warehouse[new_position.1][new_position.0] == '[' {
        move_double(warehouse, &new_position, direction);
    }

    warehouse[left.1][left.0 + 1] = '[';
    warehouse[right.1][right.0 + 1] = ']';
    warehouse[left.1][left.0] = '.';
}

fn move_double(
    warehouse: &mut Vec<Vec<char>>,
    position: &(usize, usize),
    direction: &(isize, isize),
) {
    let (left, right) = get_left_right(warehouse, position);
    match direction {
        (0, 1) => move_double_vertical(warehouse, &left, &right, direction),
        (0, -1) => move_double_vertical(warehouse, &left, &right, direction),
        (1, 0) => move_double_right(warehouse, &left, &right, direction),
        (-1, 0) => move_double_left(warehouse, &left, &right, direction),
        _ => unreachable!(),
    };
}

fn get_left_right(
    warehouse: &Vec<Vec<char>>,
    position: &(usize, usize),
) -> ((usize, usize), (usize, usize)) {
    let left: (usize, usize);
    let right: (usize, usize);
    if warehouse[position.1][position.0] == '[' {
        left = (position.0, position.1);
        right = (position.0 + 1, position.1);
    } else {
        left = (position.0 - 1, position.1);
        right = (position.0, position.1);
    }
    (left, right)
}

fn part2(warehouse: &Vec<Vec<char>>, moves: &Vec<char>) {
    let mut current_warehouse = double_warehouse(warehouse);
    let mut current_position = get_robot_start(&current_warehouse);

    for robot_move in moves {
        let direction = get_direction(robot_move);
        let new_position = get_new_position(current_position, direction);

        match current_warehouse[new_position.1][new_position.0] {
            '#' => continue,
            '[' | ']' => {
                if is_moveable_double(&current_warehouse, &new_position, &direction) {
                    move_double(&mut current_warehouse, &new_position, &direction);
                } else {
                    continue;
                }
            }
            _ => (),
        }

        current_warehouse[current_position.1][current_position.0] = '.';
        current_warehouse[new_position.1][new_position.0] = '@';
        current_position = new_position;
    }
    println!("Part 2: {}", calculate_gps(&current_warehouse));
}

pub fn day15(contents: &String) {
    let (map, moves) = parse(contents);
    part1(&map, &moves);
    part2(&map, &moves);
}
