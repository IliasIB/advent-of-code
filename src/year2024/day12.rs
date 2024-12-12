use std::collections::HashSet;

type Coord = (i32, i32);

fn parse(contents: &String) -> HashSet<(Coord, char)> {
    let mut farm: HashSet<(Coord, char)> = HashSet::new();
    let farm_chars: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    for y in 0..farm_chars.len() {
        for x in 0..farm_chars[0].len() {
            farm.insert(((x as i32, y as i32), farm_chars[y][x]));
        }
    }
    farm
}

fn get_region(farm: &mut HashSet<(Coord, char)>, land: (Coord, char)) -> HashSet<(Coord, char)> {
    let mut region: HashSet<(Coord, char)> = HashSet::new();
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    region.insert(land);

    for direction in directions {
        if let Some(new_land) =
            farm.take(&((land.0 .0 + direction.0, land.0 .1 + direction.1), land.1))
        {
            region.extend(get_region(farm, new_land));
        }
    }
    region
}

fn get_perimeter(region: &HashSet<(Coord, char)>) -> usize {
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut perimeter = 0;
    for land in region {
        let mut neighbours = 0;
        for direction in &directions {
            if let Some(_) =
                region.get(&((land.0 .0 + direction.0, land.0 .1 + direction.1), land.1))
            {
                neighbours += 1;
            }
        }
        perimeter += 4 - neighbours;
    }
    perimeter
}

fn part1(mut farm: HashSet<(Coord, char)>) {
    let mut regions: Vec<HashSet<(Coord, char)>> = Vec::new();

    while let Some(land) = farm.iter().next().cloned() {
        farm.remove(&land);
        regions.push(get_region(&mut farm, land));
    }

    let prices: usize = regions
        .iter()
        .map(|region| region.len() * get_perimeter(region))
        .sum();
    println!("Part 1: {:?}", prices);
}

fn get_sides_amount(region: &HashSet<(Coord, char)>) -> usize {
    let directions = vec![
        ((0, -1), (1, 0), (1, -1)),
        ((1, 0), (0, 1), (1, 1)),
        ((0, 1), (-1, 0), (-1, 1)),
        ((-1, 0), (0, -1), (-1, -1)),
    ];
    let mut sides = 0;
    for land in region {
        for direction in &directions {
            let first = region.get(&(
                (land.0 .0 + direction.0 .0, land.0 .1 + direction.0 .1),
                land.1,
            ));
            let second = region.get(&(
                (land.0 .0 + direction.1 .0, land.0 .1 + direction.1 .1),
                land.1,
            ));
            let third = region.get(&(
                (land.0 .0 + direction.2 .0, land.0 .1 + direction.2 .1),
                land.1,
            ));
            if first.is_none() && second.is_none() {
                sides += 1;
            } else if first.is_some() && second.is_some() && third.is_none() {
                sides += 1;
            }
        }
    }
    sides
}

fn part2(mut farm: HashSet<(Coord, char)>) {
    let mut regions: Vec<HashSet<(Coord, char)>> = Vec::new();

    while let Some(land) = farm.iter().next().cloned() {
        farm.remove(&land);
        regions.push(get_region(&mut farm, land));
    }

    let prices: usize = regions
        .iter()
        .map(|region| region.len() * get_sides_amount(region))
        .sum();
    println!("Part 2: {:?}", prices);
}

pub fn day12(contents: &String) {
    let farm = parse(contents);
    part1(farm.clone());
    part2(farm);
}
