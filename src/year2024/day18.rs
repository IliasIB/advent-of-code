use std::{cmp::Reverse, collections::BinaryHeap};

type Coordinate = (usize, usize);
type Direction = (isize, isize);
type Memory = Vec<Vec<char>>;

#[derive(PartialEq, Eq)]
struct Node {
    distance: usize,
    x: usize,
    y: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn parse(contents: &String) -> (usize, Vec<Coordinate>) {
    let contents_clean = contents.replace('\r', "");
    let memory_vec: Vec<&str> = contents_clean.split("\n\n").collect();

    let size = memory_vec[0].parse::<usize>().unwrap();
    let byte_positions: Vec<Coordinate> = memory_vec[1]
        .lines()
        .map(|line| {
            let line_vec: Vec<usize> = line
                .split(',')
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect();
            (line_vec[0], line_vec[1])
        })
        .collect();
    (size, byte_positions)
}

fn create_memory(size: &usize, byte_positions: &Vec<(usize, usize)>, amount: usize) -> Memory {
    let mut memory = vec![vec!['.'; *size]; *size];
    for i in 0..amount {
        let (x, y) = byte_positions[i];
        memory[y][x] = '#';
    }
    memory
}

fn get_directions(coordinate: Coordinate, size: usize) -> Vec<Direction> {
    let mut neighbours: Vec<Direction> = Vec::new();
    if coordinate.0 > 0 {
        neighbours.push((-1, 0));
    }
    if coordinate.1 > 0 {
        neighbours.push((0, -1));
    }
    if coordinate.0 < size - 1 {
        neighbours.push((1, 0));
    }
    if coordinate.1 < size - 1 {
        neighbours.push((0, 1));
    }
    neighbours
}

fn dijkstra(memory: &Memory) -> Vec<Vec<Option<Coordinate>>> {
    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; memory.len()]; memory.len()];
    let mut previous: Vec<Vec<Option<Coordinate>>> = vec![vec![None; memory.len()]; memory.len()];
    distances[0][0] = 0;
    heap.push(Reverse(Node {
        distance: 0,
        x: 0,
        y: 0,
    }));

    while let Some(Reverse(Node { distance, x, y })) = heap.pop() {
        for direction in get_directions((x, y), memory.len()) {
            let neighbour = (
                x.wrapping_add_signed(direction.0),
                y.wrapping_add_signed(direction.1),
            );
            if memory[neighbour.1][neighbour.0] == '#' {
                continue;
            }
            let new_distance = distance + 1;
            if distances[neighbour.1][neighbour.0] > new_distance {
                distances[neighbour.1][neighbour.0] = new_distance;
                previous[neighbour.1][neighbour.0] = Some((x, y));
                heap.push(Reverse(Node {
                    distance: new_distance,
                    x: neighbour.0,
                    y: neighbour.1,
                }));
            }
        }
    }
    previous
}

fn get_path(paths: &Vec<Vec<Option<Coordinate>>>, end: Coordinate) -> Vec<Coordinate> {
    let mut path: Vec<Coordinate> = Vec::new();
    let mut current = end;
    path.push(current);

    while let Some(previous) = paths[current.1][current.0] {
        path.push(previous);
        current = previous;
    }
    path
}

fn print_memory(memory: &Memory, path: &Vec<Coordinate>) {
    for y in 0..memory.len() {
        for x in 0..memory.len() {
            if path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", memory[y][x]);
            }
        }
        print!("\n");
    }
}

fn part1(size: &usize, byte_positions: &Vec<Coordinate>) {
    let memory = create_memory(size, byte_positions, 1024);
    let paths = dijkstra(&memory);
    let path = get_path(&paths, (size - 1, size - 1));

    print_memory(&memory, &path);
    println!("Part 1: {}", path.len() - 1);
}

fn part2(size: &usize, byte_positions: &Vec<Coordinate>) {
    let mut byte_amount = 1024;

    loop {
        let memory = create_memory(size, byte_positions, byte_amount);
        let paths = dijkstra(&memory);

        if let None = paths[size - 1][size - 1] {
            break;
        }

        let path = get_path(&paths, (size - 1, size - 1));

        while !path.contains(&byte_positions[byte_amount]) {
            byte_amount += 1;
        }
        byte_amount += 1;
    }

    println!("Part 2: {:?}", byte_positions[byte_amount - 1]);
}

pub fn day18(contents: &String) {
    let (size, byte_positions) = parse(contents);
    part1(&size, &byte_positions);
    part2(&size, &byte_positions);
}
