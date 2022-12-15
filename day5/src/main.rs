use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Instruction {
            n: caps[1].parse().unwrap(),
            from: caps[2].parse().unwrap(),
            to: caps[3].parse().unwrap(),
        }
    }
}

impl Instruction {
    fn apply(&self, stacks: &mut Vec<Vec<char>>) {
        let len = stacks[self.from - 1].len();
        let mut stack = stacks[self.from - 1].drain((len - self.n)..).collect::<Vec<_>>();
        stack.reverse();
        stacks[self.to - 1].append(&mut stack);
    }

    fn apply9001(&self, stacks: &mut Vec<Vec<char>>) {
        let len = stacks[self.from - 1].len();
        let mut stack = stacks[self.from - 1].drain((len - self.n)..).collect::<Vec<_>>();
        stacks[self.to - 1].append(&mut stack);
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    // We need to take an input like:
    //     [d]
    // [a] [b] [c]
    //  1   2   3
    // And turn it into a map of stacks:
    // 1 => [a]
    // 2 => [b, d]
    // 3 => [c]
    let num_stacks = input.split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks = vec![Vec::new(); num_stacks];
    for line in input.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1).unwrap();
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    // Picture and input are separated by a blank line
    let mut parts = input.split("\n\n");
    let stacks = parse_stacks(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    (stacks, moves)
}

fn top_of_stack(input: &str) -> String {
    let (stacks, moves) = parse_input(input);
    let mut stacks = stacks;
    for move_ in moves {
        move_.apply(&mut stacks);
    }
    stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).join("")
}

fn top_of_stack2(input: &str) -> String {
    let (stacks, moves) = parse_input(input);
    let mut stacks = stacks;
    for move_ in moves {
        move_.apply9001(&mut stacks);
    }
    stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).join("")
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", top_of_stack(&input));
    println!("{}", top_of_stack2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    \n\
    [N] [C]    \n\
    [Z] [M] [P]\n\
     1   2   3 \n\
    \n\
    move 1 from 2 to 1\n\
    move 3 from 1 to 3\n\
    move 2 from 2 to 1\n\
    move 1 from 1 to 2";

    #[test]
    fn test_parse() {
        let (stacks, moves) = parse_input(INPUT);
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(moves, vec![
            Instruction { n: 1, from: 2, to: 1 },
            Instruction { n: 3, from: 1, to: 3 },
            Instruction { n: 2, from: 2, to: 1 },
            Instruction { n: 1, from: 1, to: 2 },
        ]);
    }

    #[test]
    fn test_apply() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        Instruction { n: 1, from: 2, to: 1 }.apply(&mut stacks);
        assert_eq!(stacks, vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]);
    }

    #[test]
    fn test_top_of_stack() {
        assert_eq!(top_of_stack(INPUT), "CMZ");
    }

    #[test]
    fn test_top_of_stack2() {
        assert_eq!(top_of_stack2(INPUT), "MCD");
    }

}
