use std::{
    collections::{HashMap, VecDeque},
    usize,
};

type Coord = (usize, usize);

fn parse(contents: &String) -> Vec<Vec<char>> {
    let maze: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    return maze;
}

fn get_maze_parameters(maze: &Vec<Vec<char>>) -> (Coord, Coord) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            match maze[y][x] {
                'S' => {
                    start = (x, y);
                }
                'E' => {
                    end = (x, y);
                }
                _ => (),
            };
        }
    }
    (start, end)
}

fn part1(maze: &Vec<Vec<char>>) {
    let (start, end) = get_maze_parameters(maze);
    let shortest_distance = bfs_search(start, end, maze);

    println!("Part 1: {}", shortest_distance);
}

fn bfs_search(start: Coord, end: Coord, maze: &Vec<Vec<char>>) -> usize {
    let mut distances: HashMap<Coord, usize> = HashMap::new();
    distances.insert(start, 0);
    let directions: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_nodes: VecDeque<(Coord, (isize, isize))> = VecDeque::from([(start, (1, 0))]);

    while !current_nodes.is_empty() {
        let current_node = current_nodes.pop_front().unwrap();

        for direction in &directions {
            let mut cost = 1;
            if direction != &current_node.1 {
                cost = 1001;
            }
            let new_node = (
                current_node.0 .0.wrapping_add_signed(direction.0),
                current_node.0 .1.wrapping_add_signed(direction.1),
            );
            if maze[new_node.1][new_node.0] == '#' {
                continue;
            }
            let new_distance = distances.get(&current_node.0).unwrap() + cost;
            let current_distance = distances.get(&new_node).or(Some(&usize::MAX)).unwrap();
            if current_distance > &new_distance {
                distances.insert(new_node, new_distance);
                current_nodes.push_front((new_node, *direction));
            }
        }
    }
    return *distances.get(&end).unwrap();
}

pub fn day16(contents: &String) {
    let maze = parse(contents);
    part1(&maze);
}
