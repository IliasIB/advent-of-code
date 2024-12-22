use std::collections::HashMap;

fn parse(contents: &str) -> Vec<usize> {
    return contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap_or_default())
        .collect();
}

fn mix_and_prune(secret_number: &mut usize, new_number: usize) {
    *secret_number = *secret_number ^ new_number;
    *secret_number = *secret_number % 16777216;
}

fn generate_secret_number(seed: &usize, cache: &mut HashMap<usize, usize>) -> usize {
    let mut secret_number = *seed;
    let mut new_number: usize;
    for _ in 0..2000 {
        if let Some(number) = cache.get(&secret_number) {
            secret_number = *number;
            continue;
        }
        let initial_number = secret_number;

        new_number = secret_number * 64;
        mix_and_prune(&mut secret_number, new_number);

        new_number = secret_number / 32;
        mix_and_prune(&mut secret_number, new_number);

        new_number = secret_number * 2048;
        mix_and_prune(&mut secret_number, new_number);

        cache.insert(initial_number, secret_number);
    }
    secret_number
}

fn part1(seeds: &Vec<usize>, cache: &mut HashMap<usize, usize>) {
    let mut secret_numbers: Vec<usize> = Vec::new();
    for seed in seeds {
        let secret_number = generate_secret_number(seed, cache);
        secret_numbers.push(secret_number);
    }

    println!("{}", secret_numbers.iter().sum::<usize>());
}

fn generate_prices(seed: &usize, cache: &mut HashMap<usize, usize>) -> Vec<usize> {
    let mut prices: Vec<usize> = Vec::new();
    let mut secret_number = *seed;
    let mut new_number: usize;
    for _ in 0..2000 {
        if let Some(number) = cache.get(&secret_number) {
            secret_number = *number;
            prices.push(secret_number % 10);
            continue;
        }
        let initial_number = secret_number;

        new_number = secret_number * 64;
        mix_and_prune(&mut secret_number, new_number);

        new_number = secret_number / 32;
        mix_and_prune(&mut secret_number, new_number);

        new_number = secret_number * 2048;
        mix_and_prune(&mut secret_number, new_number);

        prices.push(secret_number % 10);

        cache.insert(initial_number, secret_number);
    }
    prices
}

fn part2(seeds: &Vec<usize>, cache: &mut HashMap<usize, usize>) {
    let mut changes_map: HashMap<(isize, isize, isize, isize), HashMap<usize, usize>> =
        HashMap::new();
    for seed in seeds {
        let prices = generate_prices(seed, cache);
        let mut current_price = prices[0];
        let mut changes: Vec<isize> = vec![current_price as isize];
        for i in 0..prices.len() - 1 {
            let change = (prices[i + 1] as isize) - (current_price as isize);
            changes.push(change);
            current_price = prices[i + 1]
        }

        for i in 1..changes.len() - 3 {
            let value = prices[i + 3];
            let sequence = (changes[i], changes[i + 1], changes[i + 2], changes[i + 3]);
            let map_for_sequence = changes_map.entry(sequence).or_default();

            if let None = map_for_sequence.get(seed) {
                map_for_sequence.insert(*seed, value);
            }
        }
    }

    let valid_sequences = changes_map
        .iter()
        .map(|(sequence, seq_map)| (*sequence, seq_map.iter().map(|(_, &value)| value).sum()))
        .collect::<Vec<((isize, isize, isize, isize), usize)>>();
    // println!("{:?}", changes_map);
    println!(
        "{:?}",
        valid_sequences.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap()
    );
}

pub fn day22(contents: &str) {
    let seeds = parse(contents);
    let mut cache: HashMap<usize, usize> = HashMap::new();
    part1(&seeds, &mut cache);
    part2(&seeds, &mut cache);
}
