fn parse(contents: &String) -> ((usize, usize, usize), Vec<usize>) {
    let contents_clean = contents.replace('\r', "");
    let contents_vec: Vec<&str> = contents_clean.split("\n\n").collect();

    let register_vec: Vec<usize> = contents_vec[0]
        .split('\n')
        .map(|line| {
            let line_vec: Vec<&str> = line.split(": ").collect();
            line_vec[1].parse::<usize>().unwrap()
        })
        .collect();
    let registers = (register_vec[0], register_vec[1], register_vec[2]);

    let program: Vec<usize> = contents_vec[1].split(": ").collect::<Vec<&str>>()[1]
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    (registers, program)
}

fn combo(registers: &(usize, usize, usize), operand: &usize) -> usize {
    match operand {
        0 | 1 | 2 | 3 => *operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        7 => panic!(),
        _ => unreachable!(),
    }
}

fn part1(mut registers: (usize, usize, usize), program: &Vec<usize>) {
    let mut instruction_pointer = 0;
    let mut output: Vec<String> = Vec::new();
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        execute(
            &mut registers,
            &mut instruction_pointer,
            &mut output,
            opcode,
            operand,
        );
    }
    println!("Part 1: {}", output.join(","));
}

fn execute(
    registers: &mut (usize, usize, usize),
    instruction_pointer: &mut usize,
    output: &mut Vec<String>,
    opcode: usize,
    operand: usize,
) {
    match opcode {
        // adv
        0 => {
            registers.0 = registers.0 / usize::pow(2, combo(&*registers, &operand) as u32);
            *instruction_pointer += 2;
        }
        // bxl
        1 => {
            registers.1 = registers.1 ^ operand;
            *instruction_pointer += 2;
        }
        // bst
        2 => {
            registers.1 = combo(&*registers, &operand) % 8usize;
            *instruction_pointer += 2;
        }
        // jnz
        3 => {
            if registers.0 == 0 {
                *instruction_pointer += 2;
            } else {
                *instruction_pointer = operand;
            }
        }
        // bxc
        4 => {
            registers.1 = registers.1 ^ registers.2;
            *instruction_pointer += 2;
        }
        // out
        5 => {
            output.push((combo(&*registers, &operand) % 8).to_string());
            *instruction_pointer += 2;
        }
        // bdv
        6 => {
            registers.1 = registers.0 / usize::pow(2, combo(&*registers, &operand) as u32);
            *instruction_pointer += 2;
        }
        // cdv
        7 => {
            registers.2 = registers.0 / usize::pow(2, combo(&*registers, &operand) as u32);
            *instruction_pointer += 2;
        }
        _ => unreachable!(),
    }
}

fn part2(program: &Vec<usize>) {
    let mut instruction_pointer;
    let mut output: Vec<String> = Vec::new();
    let program_string = program
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let mut register_a = 0;

    let mut digits = vec![0; program.len()];
    while output.join(",") != program_string {
        output.clear();
        register_a = digits
            .iter()
            .enumerate()
            .map(|(i, digit)| 8usize.pow(i as u32) * digit)
            .sum();

        let mut registers = (register_a, 0, 0);
        instruction_pointer = 0;
        while instruction_pointer < program.len() {
            let opcode = program[instruction_pointer];
            let operand = program[instruction_pointer + 1];
            execute(
                &mut registers,
                &mut instruction_pointer,
                &mut output,
                opcode,
                operand,
            );
        }
        for i in (0..program.len()).rev() {
            if output.len() < i {
                digits[i] += 1;
                break;
            }
            if output[i] != program[i].to_string() {
                digits[i] += 1;
                break;
            }
        }
    }

    println!("Part 2: {}", register_a);
}

pub fn day17(contents: &String) {
    let (registers, program) = parse(contents);
    part1(registers, &program);
    part2(&program);
}
