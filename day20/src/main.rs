fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .enumerate()
        .map(|(i, x)| (i, x))
        .collect()
}

fn rotato(mut input: Vec<(usize, i64)>) -> Vec<(usize, i64)>{
    let len = input.len();
    for orig_idx in 0..len {
        let index = input.iter().position(|&x| x.0 == orig_idx).unwrap();
        let mut new_index = input[index].1 + index as i64;
        new_index = new_index.rem_euclid(len as i64 - 1);
        let tmp = input.remove(index);
        input.insert(new_index as usize, tmp);
    }
    input
}

fn part1(input: &str) -> i64 {
    let input = parse_input(input);
    let input = rotato(input);
    // Find the index of 0
    let zero = input.iter().position(|&x| x.1 == 0).unwrap();
    // Return the sum of the values at zero + 1000, zero + 2000, zero + 3000
    // Wrapping around if necessary
    let x1 = input[(zero + 1000) % input.len()].1;
    let x2 = input[(zero + 2000) % input.len()].1;
    let x3 = input[(zero + 3000) % input.len()].1;
    x1 + x2 + x3
}

fn part2(input: &str) -> i64 {
    let mut input = parse_input(input);
    input.iter_mut().for_each(|x| x.1 *= 811_589_153);
    for _ in 0..10 {
        input = rotato(input);
    }
    // Find the index of 0
    let zero = input.iter().position(|&x| x.1 == 0).unwrap();
    // Return the sum of the values at zero + 1000, zero + 2000, zero + 3000
    // Wrapping around if necessary
    let x1 = input[(zero + 1000) % input.len()].1;
    let x2 = input[(zero + 2000) % input.len()].1;
    let x3 = input[(zero + 3000) % input.len()].1;
    x1 + x2 + x3
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "1\n\
    2\n\
    -3\n\
    3\n\
    -2\n\
    0\n\
    4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 1_623_178_306);
    }
}
