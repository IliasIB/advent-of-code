#[derive(Debug)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

fn parse(contents: &String) -> Vec<ClawMachine> {
    let claw_machines: Vec<ClawMachine> = contents
        .replace('\r', "")
        .split("\n\n")
        .map(|machine_string| {
            let machine_vec = machine_string
                .lines()
                .map(|line| {
                    let pos_vec = line.split(": ").collect::<Vec<&str>>()[1]
                        .split(", ")
                        .map(|inc_str| inc_str[2..inc_str.len()].parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();
                    (pos_vec[0], pos_vec[1])
                })
                .collect::<Vec<(isize, isize)>>();
            ClawMachine {
                button_a: machine_vec[0],
                button_b: machine_vec[1],
                prize: machine_vec[2],
            }
        })
        .collect::<Vec<ClawMachine>>();
    claw_machines
}

fn is_count_valid(counts: (isize, isize), machine: &ClawMachine, check_max: bool) -> bool {
    if check_max && (counts.0 > 100 || counts.1 > 100) {
        return false;
    }
    if counts.0 < 0 || counts.1 < 0 {
        return false;
    }
    if machine.button_a.0 * counts.0 + machine.button_b.0 * counts.1 != machine.prize.0
        || machine.button_a.1 * counts.0 + machine.button_b.1 * counts.1 != machine.prize.1
    {
        return false;
    }
    return true;
}

fn button_count_calculation(claw_machine: &ClawMachine) -> (isize, isize) {
    let determinant = claw_machine.button_a.0 * claw_machine.button_b.1
        - claw_machine.button_a.1 * claw_machine.button_b.0;
    let count_a = (claw_machine.button_b.1 * claw_machine.prize.0
        - claw_machine.button_b.0 * claw_machine.prize.1)
        / determinant;
    let count_b = (claw_machine.button_a.0 * claw_machine.prize.1
        - claw_machine.button_a.1 * claw_machine.prize.0)
        / determinant;
    (count_a, count_b)
}

fn part1(claw_machines: &Vec<ClawMachine>) {
    let mut total_tokens: usize = 0;
    for claw_machine in claw_machines {
        let (count_a, count_b) = button_count_calculation(claw_machine);
        if is_count_valid((count_a, count_b), &claw_machine, true) {
            total_tokens += 3 * (count_a as usize) + (count_b as usize);
        }
    }
    println!("Part 1: {:?}", total_tokens);
}

fn part2(claw_machines: Vec<ClawMachine>) {
    let mut total_tokens: usize = 0;
    for mut claw_machine in claw_machines {
        claw_machine.prize = (
            claw_machine.prize.0 + 10000000000000,
            claw_machine.prize.1 + 10000000000000,
        );
        let (count_a, count_b) = button_count_calculation(&claw_machine);
        if is_count_valid((count_a, count_b), &claw_machine, false) {
            total_tokens += 3 * (count_a as usize) + (count_b as usize);
        }
    }
    println!("Part 2: {:?}", total_tokens);
}

pub fn day13(contents: &String) {
    let claw_machines = parse(contents);
    part1(&claw_machines);
    part2(claw_machines);
}
