use std::collections::HashMap;

type PatternsMap = HashMap<char, Vec<String>>;
type DesignCache = HashMap<String, usize>;

fn parse(contents: &String) -> (PatternsMap, Vec<String>) {
    let contents_clean = contents.replace('\r', "");
    let contents_vec: Vec<&str> = contents_clean.split("\n\n").collect();

    let pattern_strings: Vec<String> = contents_vec[0]
        .split(", ")
        .map(|design| design.to_string())
        .collect();
    let mut patterns: PatternsMap = HashMap::new();
    for pattern in pattern_strings {
        patterns
            .entry(pattern.chars().nth(0).unwrap())
            .or_default()
            .push(pattern);
    }
    let designs: Vec<String> = contents_vec[1]
        .lines()
        .map(|design| design.to_string())
        .collect();

    (patterns, designs)
}

fn is_possible(design: &String, patterns_map: &PatternsMap) -> bool {
    if let Some(patterns) = patterns_map.get(&design.chars().nth(0).unwrap()) {
        for pattern in patterns {
            if pattern.len() > design.len() || design[0..pattern.len()] != *pattern {
                continue;
            } else if pattern.len() == design.len()
                || is_possible(
                    &design[pattern.len()..design.len()].to_string(),
                    patterns_map,
                )
            {
                return true;
            }
        }
    }
    return false;
}

fn part1(patterns_map: &PatternsMap, designs: &Vec<String>) {
    let mut amount_possible = 0;

    for design in designs {
        if is_possible(design, patterns_map) {
            amount_possible += 1;
        }
    }
    println!("Part 1: {}", amount_possible);
}

fn count_possible(
    design: &String,
    patterns_map: &PatternsMap,
    design_cache: &mut DesignCache,
) -> usize {
    if let Some(pattern_count) = design_cache.get(design) {
        return *pattern_count;
    }
    let mut amount_possible = 0;
    if let Some(patterns) = patterns_map.get(&design.chars().nth(0).unwrap()) {
        for pattern in patterns {
            if pattern.len() > design.len() || design[0..pattern.len()] != *pattern {
                continue;
            } else if pattern.len() == design.len() {
                amount_possible += 1;
            } else {
                amount_possible += count_possible(
                    &design[pattern.len()..design.len()].to_string(),
                    patterns_map,
                    design_cache,
                )
            }
        }
    }
    design_cache
        .entry(design.to_string())
        .or_insert(amount_possible);
    return amount_possible;
}

fn part2(patterns_map: &PatternsMap, designs: &Vec<String>) {
    let mut amount_possible = 0;
    let mut design_cache: DesignCache = HashMap::new();

    for design in designs {
        amount_possible += count_possible(design, patterns_map, &mut design_cache);
    }
    println!("Part 2: {}", amount_possible);
}

pub fn day19(contents: &String) {
    let (patterns_map, designs) = parse(contents);
    part1(&patterns_map, &designs);
    part2(&patterns_map, &designs);
}
