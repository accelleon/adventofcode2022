use lazy_regex::regex;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Debug)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn parse_valve(s: &str) -> (String, Valve) {
    let re = regex!(r"Valve (\w+) has flow rate=(\d+); .+(?:valve[s]?) (.+)");
    let caps = re.captures(s).unwrap();
    let name = caps[1].to_owned();
    let tunnels = caps[3].split(", ").map(|s| s.to_owned()).collect();
    (
        name,
        Valve {
            flow_rate: caps[2].parse().unwrap(),
            tunnels,
        }
    )
}

fn astar(start: &str, end: &str, valves: &HashMap<String, Valve>) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    while let Some((valve, cost)) = queue.pop_front() {
        if valve == end {
            return cost;
        }
        if visited.contains(&valve) {
            continue;
        }
        visited.insert(valve);
        for valve in valves[valve].tunnels.iter() {
            queue.push_back((valve, cost + 1));
        }
    }
    unreachable!()
}

fn dist_matrix(valves: &HashMap<String, Valve>) -> Vec<Vec<(i64, i64)>> {
    // Convert our hashmap to a matrix of distances of relevant valves

    // Filter out valves with flow_rate == 0 but keep our starting valve
    let mut non_zero: Vec<(&String, &Valve)> = valves.iter().filter(|(n, v)| *n == "AA" || v.flow_rate > 0).collect();
    non_zero.sort_unstable_by_key(|(n, _)| *n);

    let mut matrix = vec![vec![(std::i64::MAX, 0); non_zero.len()]; non_zero.len()];

    for (i, (name1, _)) in non_zero.iter().enumerate() {
        for (j, (name2, v2)) in non_zero.iter().enumerate() {
            if i == j {
                matrix[i][j] = (0, 0);
                continue;
            }
            // Tuple of (cost, flow_rate)
            matrix[i][j] = (astar(name1, name2, valves) as i64, v2.flow_rate as i64);
        }
    }

    matrix
}

fn best_path(matrix: &Vec<Vec<(i64, i64)>>, i: usize, mask: Option<u64>, mut max_cost: i64) -> i64 {
    let mask = mask.unwrap_or_else(|| (1 << matrix.len()) - 1 & !(1 << i));
    let node_val;

    if matrix[0][i].1 > 0 {
        max_cost -= 1;
        node_val = matrix[0][i].1 * max_cost;
    } else {
        node_val = 0;
    }

    let mut best = 0;
    for j in 0..matrix.len() {
        if mask & (1 << j) == 0 {
            continue;
        }
        // Add 1 to turn the valve
        let cost = matrix[i][j].0;
        if cost < max_cost {
            let new_mask = mask & !(1 << j);
            let path = best_path(matrix, j, Some(new_mask), max_cost - cost);
            best = best.max(path);
        }
    }

    best + node_val
}

fn all_paths(matrix: &Vec<Vec<(i64, i64)>>, i: usize, mask: Option<u64>, mut max_cost: i64) -> Vec<(i64, u64)> {
    let mask = mask.unwrap_or_else(|| (1 << matrix.len()) - 1 & !(1 << i));
    let node_val;

    if matrix[0][i].1 > 0 {
        max_cost -= 1;
        node_val = matrix[0][i].1 * max_cost;
    } else {
        node_val = 0;
    }

    let mut paths = vec![(node_val, mask)];
    for j in 0..matrix.len() {
        if mask & (1 << j) == 0 {
            continue;
        }
        // Add 1 to turn the valve
        let cost = matrix[i][j].0;
        if cost < max_cost {
            let new_mask = mask & !(1 << j);
            let mut path = all_paths(matrix, j, Some(new_mask), max_cost - cost);
            path.iter_mut().for_each(|p| p.0 += node_val);
            paths.extend(path);
        }
    }
    paths
}

// Moving from one valve to another takes one minute.
// Opening a valve takes one minute.
// Opening a valve increases pressure released by flow rate * minutes left.
// Part 1: Find the most pressure you can release
fn part1(input: &str) -> i64 {
    // Build a map of valves
    let valves: HashMap<String, Valve> = input.lines().filter(|s| !s.is_empty()).map(parse_valve).collect();
    let matrix = dist_matrix(&valves);
    best_path(&matrix, 0, None, 30)
}

fn part2(input: &str) -> i64 {
    // Build a map of valves
    let valves: HashMap<String, Valve> = input.lines().filter(|s| !s.is_empty()).map(parse_valve).collect();
    let matrix = dist_matrix(&valves);
    let paths = all_paths(&matrix, 0, None, 26);
    // Filter out paths that visit all nodes
    let paths: Vec<_> = paths.iter().filter(|p| p.1 != 0).collect();    

    // Find 2 paths when added together have the most pressure but don't overlap visited nodes
    let mut best = 0;
    // Also mask out the first node
    let valve_mask = (1 << matrix.len()) - 1 & !(1 << 0);
    for i in 0..paths.len() {
        for j in i+1..paths.len() {
            if (!paths[i].1 & !paths[j].1) & valve_mask == 0 {
                best = best.max(paths[i].0 + paths[j].0);
            }
        }
    }
    best
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
    Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
    Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
    Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
    Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
    Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
    Valve HH has flow rate=22; tunnel leads to valve GG\n\
    Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
    Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parse() {
        let valves: HashMap<String, Valve> = TEST.lines().filter(|s| !s.is_empty()).map(parse_valve).collect();
        assert_eq!(valves["AA"], Valve { flow_rate: 0, tunnels: vec![
            "DD".to_owned(),
            "II".to_owned(),
            "BB".to_owned(),
        ]});
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 1651);
    }

    #[test]
    fn test_paths() {
        let valves: HashMap<String, Valve> = TEST.lines().filter(|s| !s.is_empty()).map(parse_valve).collect();
        let matrix = dist_matrix(&valves);
        println!("{:?}", all_paths(&matrix, 0, None, 30));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 1707);
    }
}
