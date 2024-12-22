fn parse(contents: &str) -> Vec<&str> {
    return contents.lines().collect::<Vec<&str>>();
}

fn get_numeric_horizontal(number: &char) -> isize {
    return match number {
        '7' | '4' | '1' => 0,
        '8' | '5' | '2' | '0' => 1,
        '9' | '6' | '3' | 'A' => 2,
        _ => unreachable!(),
    };
}

fn get_numeric_vertical(number: &char) -> isize {
    return match number {
        '7' | '8' | '9' => 0,
        '4' | '5' | '6' => 1,
        '1' | '2' | '3' => 2,
        '0' | 'A' => 3,
        _ => unreachable!(),
    };
}

fn get_directional_horizontal(number: &char) -> isize {
    return match number {
        '<' => 0,
        '^' | 'v' => 1,
        '>' | 'A' => 2,
        _ => unreachable!(),
    };
}

fn get_directional_vertical(number: &char) -> isize {
    return match number {
        '^' | 'A' => 0,
        '<' | 'v' | '>' => 1,
        _ => unreachable!(),
    };
}

fn convert_horizontal(amount: isize) -> String {
    if amount > 0 {
        return ">".repeat(amount.abs() as usize);
    } else if amount < 0 {
        return "<".repeat(amount.abs() as usize);
    } else {
        return "".to_string();
    }
}

fn convert_vertical(amount: isize) -> String {
    if amount > 0 {
        return "v".repeat(amount.abs() as usize);
    } else if amount < 0 {
        return "^".repeat(amount.abs() as usize);
    } else {
        return "".to_string();
    }
}

fn get_numeric_sequence(start: &char, end: &char) -> String {
    let vertical = get_numeric_vertical(end) - get_numeric_vertical(start);
    let horizontal = get_numeric_horizontal(end) - get_numeric_horizontal(start);

    if vertical > 0 {
        return convert_horizontal(horizontal) + &convert_vertical(vertical);
    } else {
        return convert_vertical(vertical) + &convert_horizontal(horizontal);
    }
}

fn get_directional_sequence(start: &char, end: &char) -> String {
    let vertical = get_directional_vertical(end) - get_directional_vertical(start);
    let horizontal = get_directional_horizontal(end) - get_directional_horizontal(start);

    if vertical < 0 {
        return convert_horizontal(horizontal) + &convert_vertical(vertical);
    } else {
        return convert_vertical(vertical) + &convert_horizontal(horizontal);
    }
}

fn part1(codes: &Vec<&str>) {
    let mut sequences1: Vec<String> = Vec::new();
    let mut sequences2: Vec<String> = Vec::new();
    let mut sequences3: Vec<String> = Vec::new();

    let mut current_numeric = 'A';
    let mut current_directional1 = 'A';
    let mut current_directional2 = 'A';
    for &code in codes {
        let mut numeric_sequence: String = "".to_string();
        for button in code.chars() {
            numeric_sequence.push_str(&get_numeric_sequence(&current_numeric, &button));
            numeric_sequence.push('A');
            current_numeric = button;
        }
        sequences1.push(numeric_sequence.clone());

        let mut directional_sequence1: String = "".to_string();
        for button in numeric_sequence.chars() {
            directional_sequence1
                .push_str(&get_directional_sequence(&current_directional1, &button));
            directional_sequence1.push('A');
            current_directional1 = button;
        }
        sequences2.push(directional_sequence1.clone());

        let mut directional_sequence2: String = "".to_string();
        for button in directional_sequence1.chars() {
            directional_sequence2
                .push_str(&get_directional_sequence(&current_directional2, &button));
            directional_sequence2.push('A');
            current_directional2 = button;
        }

        sequences3.push(directional_sequence2);
    }

    for i in 0..sequences1.len() {
        println!("{}", sequences1[i]);
        println!("{}", sequences2[i]);
        println!("{}", sequences3[i]);
        println!(
            "{} * {}",
            sequences3[i].len(),
            codes[i][0..3].parse::<usize>().unwrap()
        );
    }
}

pub fn day21(contents: &str) {
    let codes = parse(contents);
    part1(&codes);
}
