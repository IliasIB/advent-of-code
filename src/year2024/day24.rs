use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Operation {
    input_1: String,
    input_2: String,
    operator: Operator,
    output: String,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Operator, ()> {
        match s {
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "XOR" => Ok(Operator::XOR),
            _ => Err(()),
        }
    }
}

fn parse(contents: &str) -> (Vec<(String, bool)>, Vec<Operation>) {
    let contents_clean = contents.replace('\r', "");
    let contents_vec: Vec<&str> = contents_clean.split("\n\n").collect();

    let inputs: Vec<(String, bool)> = contents_vec[0]
        .lines()
        .map(|line| {
            let line_vec: Vec<&str> = line.split(": ").collect();
            (
                line_vec[0].to_string(),
                line_vec[1].parse::<u32>().unwrap_or_default() != 0,
            )
        })
        .collect();

    let operations: Vec<Operation> = contents_vec[1]
        .lines()
        .map(|line| {
            let operation_vec: Vec<&str> = line.split(" -> ").collect();
            let operator_vec: Vec<&str> = operation_vec[0].split(' ').collect();
            Operation {
                input_1: operator_vec[0].to_string(),
                input_2: operator_vec[2].to_string(),
                operator: Operator::from_str(operator_vec[1]).expect("Invalid operator"),
                output: operation_vec[1].to_string(),
            }
        })
        .collect();

    (inputs, operations)
}

fn get_input(
    input: &str,
    outputs: &mut HashMap<String, bool>,
    operation_map: &HashMap<&str, &Operation>,
) -> bool {
    if let Some(output) = outputs.get(input) {
        return *output;
    } else {
        return solve_operation(outputs, operation_map.get(input).unwrap(), operation_map);
    }
}

fn solve_operation(
    outputs: &mut HashMap<String, bool>,
    operation: &Operation,
    operation_map: &HashMap<&str, &Operation>,
) -> bool {
    let input1 = get_input(&operation.input_1, outputs, operation_map);
    let input2 = get_input(&operation.input_2, outputs, operation_map);

    let output = match operation.operator {
        Operator::AND => input1 && input2,
        Operator::OR => input1 || input2,
        Operator::XOR => input1 ^ input2,
    };

    outputs.insert(operation.output.to_string(), output);

    return output;
}

fn part1(inputs: &Vec<(String, bool)>, operations: &Vec<Operation>) {
    let mut outputs: HashMap<String, bool> = HashMap::new();
    let mut operation_map: HashMap<&str, &Operation> = HashMap::new();

    for input in inputs {
        outputs.insert(input.0.to_string(), input.1);
    }

    for operation in operations {
        operation_map.insert(&operation.output, operation);
    }

    let mut final_operations: Vec<&Operation> = operations
        .iter()
        .filter(|&operation| operation.output.starts_with('z'))
        .collect();
    final_operations.sort_by(|&a, &b| a.output.cmp(&b.output));

    let mut final_output: String = "".to_string();

    for final_operation in final_operations {
        let operation_output = solve_operation(&mut outputs, final_operation, &operation_map);
        let bool_number = (operation_output as i32).to_string();
        final_output.insert_str(0, &bool_number);
    }

    println!(
        "Part 1: {}",
        isize::from_str_radix(&final_output, 2).unwrap()
    );
}

pub fn day24(contents: &str) {
    let (inputs, operations) = parse(contents);
    part1(&inputs, &operations);
}
