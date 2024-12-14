use std::{collections::HashMap, usize};

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

fn parse(contents: &String) -> ((usize, usize), Vec<Robot>) {
    let lines: Vec<&str> = contents.lines().collect();
    let dimensions: Vec<usize> = lines[0]
        .split(" ")
        .map(|dim| dim.parse::<usize>().unwrap())
        .collect();
    let dimensions = (dimensions[0], dimensions[1]);

    let robots: Vec<Robot> = lines[1..lines.len()]
        .iter()
        .map(|&robot_line| {
            let robot_vec: Vec<(isize, isize)> = robot_line
                .split(" ")
                .map(|num_str| {
                    let num_vec: Vec<isize> = num_str[2..num_str.len()]
                        .split(',')
                        .map(|x| x.parse::<isize>().unwrap())
                        .collect();
                    (num_vec[0], num_vec[1])
                })
                .collect();
            Robot {
                position: (robot_vec[0].0 as usize, robot_vec[0].1 as usize),
                velocity: robot_vec[1],
            }
        })
        .collect();
    (dimensions, robots)
}

fn calculate_position(robot: &Robot, seconds: usize, dimensions: (usize, usize)) -> (usize, usize) {
    (
        ((robot.position.0 as isize) + (seconds as isize) * robot.velocity.0)
            .rem_euclid(dimensions.0 as isize) as usize,
        ((robot.position.1 as isize) + (seconds as isize) * robot.velocity.1)
            .rem_euclid(dimensions.1 as isize) as usize,
    )
}

fn calculate_safety_factor(
    positions: HashMap<(usize, usize), usize>,
    dimensions: (usize, usize),
) -> usize {
    let mut safety_factors: Vec<usize> = vec![0, 0, 0, 0];
    let half_point = (dimensions.0 / 2, dimensions.1 / 2);
    for (position, amount) in positions {
        if position.0 < half_point.0 && position.1 < half_point.1 {
            safety_factors[0] += amount;
        } else if position.0 < half_point.0 && position.1 > half_point.1 {
            safety_factors[1] += amount;
        } else if position.0 > half_point.0 && position.1 < half_point.1 {
            safety_factors[2] += amount;
        } else if position.0 > half_point.0 && position.1 > half_point.1 {
            safety_factors[3] += amount;
        }
    }

    let safety_factor: usize = safety_factors.iter().product();
    safety_factor
}

fn print_positions(dimensions: (usize, usize), positions: &HashMap<(usize, usize), usize>) {
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            if let Some(amount) = positions.get(&(x, y)) {
                print!("{}", amount);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn part1(dimensions: (usize, usize), robots: &Vec<Robot>) {
    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();

    for robot in robots {
        let new_position = calculate_position(&robot, 100, dimensions);
        *positions.entry(new_position).or_default() += 1;
    }
    println!("Part 1: {}", calculate_safety_factor(positions, dimensions));
}

fn calculate_tree_score(
    positions: HashMap<(usize, usize), usize>,
    dimensions: (usize, usize),
) -> usize {
    let mut tree_score = 0;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            if positions.contains_key(&(x, y))
                && positions.contains_key(&(x + 1, y))
                && positions.contains_key(&(x + 2, y))
                && positions.contains_key(&(x + 3, y))
                && positions.contains_key(&(x + 4, y))
            {
                tree_score += 1;
            }
        }
    }
    tree_score
}

fn part2(dimensions: (usize, usize), robots: &Vec<Robot>) {
    let mut tree_frame: usize = 0;
    let mut highest_tree_score: usize = 0;
    for i in 0..10000 {
        let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
        for robot in robots {
            let new_position = calculate_position(robot, i, dimensions);
            *positions.entry(new_position).or_default() += 1;
        }
        let tree_score = calculate_tree_score(positions, dimensions);
        if tree_score > highest_tree_score {
            tree_frame = i;
            highest_tree_score = tree_score;
        }
    }

    let mut tree_positions: HashMap<(usize, usize), usize> = HashMap::new();
    for robot in robots {
        let new_position = calculate_position(&robot, tree_frame, dimensions);
        *tree_positions.entry(new_position).or_default() += 1;
    }
    print_positions(dimensions, &tree_positions);
    println!("{tree_frame}");
}

pub fn day14(contents: &String) {
    let (dimensions, robots) = parse(contents);
    part1(dimensions, &robots);
    part2(dimensions, &robots);
}
