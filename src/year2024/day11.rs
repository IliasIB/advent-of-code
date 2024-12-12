use std::collections::HashMap;

fn parse(contents: &String) -> Vec<usize> {
    let numbers = contents
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return numbers;
}

fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut current_stones = HashMap::with_capacity(stones.len());
    for (&number, &amount) in stones {
        match number {
            0 => {
                *current_stones.entry(1).or_insert(0) += amount;
            }
            number if number.ilog10() % 2 == 1 => {
                let digits = 10usize.pow((number.ilog10() + 1) / 2);
                *current_stones.entry(number % digits).or_default() += amount;
                *current_stones.entry(number / digits).or_default() += amount;
            }
            _ => *current_stones.entry(number * 2024).or_default() += amount,
        }
    }
    current_stones
}

fn get_stone_count(stones: &mut HashMap<usize, usize>, blinks: usize) -> usize {
    for _ in 0..blinks {
        *stones = blink(stones);
    }
    stones.values().sum::<usize>()
}

fn part1(numbers: Vec<usize>) {
    let mut numbers_map: HashMap<usize, usize> = HashMap::new();

    for number in numbers {
        numbers_map.insert(number, 1);
    }
    let count = get_stone_count(&mut numbers_map, 25);
    println!("Part 1: {}", count);
}

fn part2(numbers: Vec<usize>) {
    let mut numbers_map: HashMap<usize, usize> = HashMap::new();

    for number in numbers {
        numbers_map.insert(number, 1);
    }
    let count = get_stone_count(&mut numbers_map, 75);

    println!("Part 2: {}", count);
}

pub fn day11(contents: &String) {
    let numbers = parse(contents);
    part1(numbers.clone());
    part2(numbers);
}
