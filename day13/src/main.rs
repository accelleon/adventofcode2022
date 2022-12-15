use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
            (Packet::Int(a), Packet::List(b)) => {
                // Convert Int to List and compare
                let a = vec![Packet::Int(*a)];
                a.partial_cmp(b)
            }
            (Packet::List(a), Packet::Int(b)) => {
                // Convert Int to List and compare
                let b = vec![Packet::Int(*b)];
                a.partial_cmp(&b)
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(b)) => {
                // Convert Int to List and compare
                let a = vec![Packet::Int(*a)];
                a.cmp(b)
            }
            (Packet::List(a), Packet::Int(b)) => {
                // Convert Int to List and compare
                let b = vec![Packet::Int(*b)];
                a.cmp(&b)
            }
        }
    }
}

fn extract_bracket(s: &str) -> Option<&str> {
    // Extracts a full bracket statement
    // Must be able to handle nested brackets
    let mut n = 0; // Depth
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            n += 1;
        } else if c == ']' {
            n -= 1;
        }
        if n == 0 {
            return s.get(..(i+1));
        }
    }
    None
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        if s.starts_with('[') {
            let s = extract_bracket(s).unwrap();
            let s = s.get(1..s.len() - 1).unwrap();
            let mut v = Vec::new();
            let mut i = 0;
            while i < s.len() {
                let c = s.chars().nth(i).unwrap();
                if c == '[' {
                    let s = extract_bracket(s.get(i..).unwrap()).unwrap();
                    v.push(Packet::from(s));
                    i += s.len();
                } else if c.is_digit(10) {
                    let s = s.get(i..).unwrap();
                    let s = s.split(|c: char| !c.is_ascii_digit()).next().unwrap();
                    v.push(Packet::from(s));
                    i += s.len();
                } else {
                    i += 1;
                }
            }
            Packet::List(v)
        } else {
            Packet::Int(s.parse().unwrap())
        }
    }
}

fn part1(s: &str) -> usize {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect::<Vec<Packet>>()
        .chunks(2)
        .enumerate()
        .filter(|(_, c)| c[0] < c[1])
        .map(|(i, _)| i+1)
        .sum()
}

fn part2(s: &str) -> usize {
    let mut packets = s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect::<Vec<Packet>>();

    // Push 2 divider packets
    // [[2]] and [[6]]
    packets.push("[[2]]".into());
    packets.push("[[6]]".into());

    packets.sort();

    // Return product of the index of the two divider packets
    let i = packets.iter().position(|p| p == &Packet::from("[[2]]")).unwrap();
    let j = packets.iter().position(|p| p == &Packet::from("[[6]]")).unwrap();
    (i+1) * (j+1)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "[1,1,3,1,1]\n\
    [1,1,5,1,1]\n\
    \n\
    [[1],[2,3,4]]\n\
    [[1],4]\n\
    \n\
    [9]\n\
    [[8,7,6]]\n\
    \n\
    [[4,4],4,4]\n\
    [[4,4],4,4,4]\n\
    \n\
    [7,7,7,7]\n\
    [7,7,7]\n\
    \n\
    []\n\
    [3]\n\
    \n\
    [[[]]]\n\
    [[]]\n\
    \n\
    [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_bracket() {
        assert_eq!(extract_bracket("[1,2,3]"), Some("[1,2,3]"));
        assert_eq!(extract_bracket("[1,[2,3]]"), Some("[1,[2,3]]"));
        assert_eq!(extract_bracket("[1,[2,[3,4]]]"), Some("[1,[2,[3,4]]]"));
    }

    #[test]
    fn test_parse_part() {
        let part = Packet::from("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(part, Packet::List(vec![
            Packet::Int(1),
            Packet::List(vec![
                Packet::Int(2),
                Packet::List(vec![
                    Packet::Int(3),
                    Packet::List(vec![
                        Packet::Int(4),
                        Packet::List(vec![
                            Packet::Int(5),
                            Packet::Int(6),
                            Packet::Int(0),
                        ]),
                    ]),
                ]),
            ]),
            Packet::Int(8),
            Packet::Int(9),
        ]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 140);
    }
}
