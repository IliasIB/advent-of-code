pub fn day_2(contents: &String) {
    let mut dimensions = extract_dimensions(contents);

    part_1(&mut dimensions);
    part_2(dimensions);
}

fn part_2(dimensions: Vec<Vec<u32>>) {
    let mut ribbon: u32 = 0;
    for dimension in dimensions {
        ribbon += dimension[0] + dimension[1] + dimension[0] + dimension[1];
        ribbon += dimension[0] * dimension[1] * dimension[2];
    }
    println!("Part 2: {ribbon}");
}

fn part_1(dimensions: &mut Vec<Vec<u32>>) {
    let mut area: u32 = 0;
    for dimension in dimensions {
        dimension.sort();

        area += 2 * dimension[0] * dimension[1]
            + 2 * dimension[1] * dimension[2]
            + 2 * dimension[2] * dimension[0];
        area += dimension[0] * dimension[1];
    }
    println!("Part 1: {area}");
}

fn extract_dimensions(contents: &String) -> Vec<Vec<u32>> {
    let dimensions = contents
        .lines()
        .map(|line| {
            line.split('x')
                .map(|dimension| dimension.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    dimensions
}
