use std::collections::HashMap;

fn parse(contents: &str) -> (HashMap<(&str, &str), bool>, HashMap<&str, Vec<&str>>) {
    let connections = contents
        .lines()
        .map(|line| {
            let nodes = line.split('-').collect::<Vec<&str>>();
            (nodes[0], nodes[1])
        })
        .collect::<Vec<(&str, &str)>>();

    let mut network_map: HashMap<(&str, &str), bool> = HashMap::new();
    let mut t_connected: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from, to) in connections {
        network_map.insert((from, to), true);
        network_map.insert((to, from), true);

        if from.chars().nth(0).unwrap() == 't' {
            t_connected.entry(from).or_default().push(to);
        } else if to.chars().nth(0).unwrap() == 't' {
            t_connected.entry(to).or_default().push(from);
        }
    }

    return (network_map, t_connected);
}

fn part1(network_map: &HashMap<(&str, &str), bool>, t_connected: &HashMap<&str, Vec<&str>>) {
    let mut triples: Vec<(&str, &str, &str)> = Vec::new();

    for (&t_pc, connections) in t_connected {
        for i in 0..connections.len() - 1 {
            for j in i + 1..connections.len() {
                if let Some(_) = network_map.get(&(connections[i], connections[j])) {
                    triples.push((t_pc, connections[i], connections[j]));
                }
            }
        }
    }

    println!("Part 1: {}", triples.len());
}

// fn bron_kerbosch(
//     result: HashSet<String>,
//     mut possibilities: HashSet<String>,
//     mut excluded: HashSet<String>,
//     neighbours_map: &HashMap<String, HashSet<String>>,
// ) -> Option<HashSet<String>> {
//     if possibilities.is_empty() && excluded.is_empty() {
//         return Some(result);
//     }
//     for possibility in possibilities {
//         let new_result: HashSet<String> = result.clone();
//         new_result.insert(possibility);
//         let neighbours = neighbours_map.get(&possibility).unwrap();
//         let new_possibilities: HashSet<String> =
//             possibilities.intersection(neighbours).cloned().collect();
//         let new_excluded: HashSet<String> = excluded.intersection(neighbours).cloned().collect();
//         if let Some(found_result) = bron_kerbosch(result, possibilities, excluded, neighbours_map) {
//             return Some(found_result);
//         }

//         possibilities.remove(&possibility);
//         excluded.insert(possibility);
//     }
//     return None;
// }

// fn part2(network_map: &HashMap<(&str, &str), bool>) {
//     let mut neighbours_map: HashMap<String, HashSet<String>> = HashMap::new();
//     let mut nodes: HashSet<String> = HashSet::new();
//     for (&(from, to), _) in network_map {
//         neighbours_map.entry(from).or_default().insert(to);
//         nodes.insert(from);
//         nodes.insert(to);
//     }
// }

pub fn day23(contents: &str) {
    let (network_map, t_connected) = parse(contents);
    part1(&network_map, &t_connected);
}
