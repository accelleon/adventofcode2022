#![feature(unboxed_closures, fn_traits)]
use lazy_regex::regex;
use std::{cell::RefCell, collections::VecDeque};

#[derive(Debug)]
enum OpSide {
    Old,
    X(u64),
}

impl OpSide {
    fn eval(&self, old: u64) -> u64 {
        match self {
            OpSide::Old => old as u64,
            OpSide::X(x) => *x as u64,
        }
    }
}

impl From<&str> for OpSide {
    fn from(s: &str) -> OpSide {
        if s == "old" {
            OpSide::Old
        } else {
            OpSide::X(s.parse().unwrap())
        }
    }
}

// Operation: new = old * old
#[derive(Debug)]
struct Op {
    left: OpSide,
    right: OpSide,
    op: fn(u64, u64) -> u64,
}

impl Op {
    fn eval1(&self, old: u64) -> u64 {
        (self.op)(self.left.eval(old), self.right.eval(old))/3
    }
    fn eval2(&self, old: u64) -> u64 {
        (self.op)(self.left.eval(old), self.right.eval(old))
    }
}

// "Operation: new = old * old"
impl From<&str> for Op {
    fn from(s: &str) -> Op {
        let re = regex!(r"Operation: new = (old|\d+) ([*+-/]) (old|\d+)");
        let captures = re.captures(s).unwrap();
        let mut parts = captures
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str());

        let left = parts.next().unwrap().into();
        let op = parts.next().unwrap();
        let right = parts.next().unwrap().into();
        let op = match op {
            "*" => |x, y| x * y,
            "+" => |x, y| x + y,
            "-" => |x, y| x - y,
            "/" => |x, y| x / y,
            _ => panic!("Unknown operator {}", op),
        };
        Op {
            left,
            right,
            op,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Test {
    if_true: u64,
    if_false: u64,
    x: u64,
}

impl Test {
    fn eval(&self, x: u64) -> u64 {
        if x % self.x == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

// Test: divisible by x
//   If true: throw to monkey 3
//   If false: throw to monkey 1
impl<'a, T> From<T> for Test 
where T: Iterator<Item = &'a str> {
    fn from(mut s: T) -> Test {
        let re = [
            regex!(r"Test: divisible by (\d+)"),
            regex!(r"If true: throw to monkey (\d+)"),
            regex!(r"If false: throw to monkey (\d+)"),
        ];

        let caps = re.iter().map(|re| 
            re
                .captures(s.next().unwrap()).unwrap()
                .get(1).unwrap()
                .as_str()
                .parse().unwrap()
        ).collect::<Vec<_>>();

        Test {
            x: caps[0],
            if_true: caps[1],
            if_false: caps[2],
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: Test,
    inspected: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Op, test: Test) -> Monkey {
        Monkey {
            items: items.into(),
            operation,
            test,
            inspected: 0,
        }
    }

    fn inspect1(&mut self) -> Option<(usize, u64)> {
        match self.items.pop_front() {
            Some(i) => {
                let i = self.operation.eval1(i);
                self.inspected += 1;
                Some((
                    self.test.eval(i) as usize,
                    i,
                ))
            },
            None => None,
        }
    }

    fn inspect2(&mut self) -> Option<(usize, u64)> {
        match self.items.pop_front() {
            Some(i) => {
                let i = self.operation.eval2(i);
                self.inspected += 1;
                Some((
                    self.test.eval(i) as usize,
                    i,
                ))
            },
            None => None,
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Monkey {
        // Don't care about the first line
        let mut lines = s.lines().skip(1);
        // Starting items: 52, 62, 94, 96, 52, 87, 53, 60
        let items = lines
            .next().unwrap()
            .split(": ")
            .nth(1).unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation: Op = lines.next().unwrap().into();
        // Take the next 3 lines
        let test: Test = lines.into();
        Monkey::new(items, operation, test)
    }
}

fn parse(s: &str) -> Vec<RefCell<Monkey>> {
    // Split by 2 newline
    s.split("\n\n").map(|s| RefCell::new(s.into())).collect()
}

fn part1(input: &str) -> u64 {
    let monkeys = parse(input);
    let len = monkeys.len();
    // Run 20 rounds
    for _ in 0..20 {
        // 1 round
        for i in 0..len {
            let mut monkey = monkeys[i].borrow_mut();
            while let Some((next, item)) = monkey.inspect1() {
                monkeys[next].borrow_mut().add_item(item);
            }
        }
    }
    
    // Find the top 2 monkeys with most inspected items
    let mut top = monkeys
        .iter()
        .map(|m| m.borrow().inspected).collect::<Vec<_>>();
    top.sort();
    // Take the last 2 items
    top.iter().rev().take(2).product()
}

fn part2(input: &str) -> u64 {
    let monkeys = parse(input);
    let com_mul: u64 = monkeys.iter().map(|m| m.borrow().test.x).product();
    let len = monkeys.len();
    // Run 10,000 rounds
    for _ in 0..10_000 {
        // 1 round
        for i in 0..len {
            let mut monkey = monkeys[i].borrow_mut();
            while let Some((next, item)) = monkey.inspect2() {
                monkeys[next].borrow_mut().add_item(item % com_mul);
            }
        }
    }
    
    // Find the top 2 monkeys with most inspected items
    let mut top = monkeys
        .iter()
        .map(|m| m.borrow().inspected).collect::<Vec<_>>();
    top.sort();
    // Take the last 2 items
    top.iter().rev().take(2).product()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &'static str = 
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_op() {
        assert_eq!(Op::from("  Operation: new = old * 19").eval1(1), 6);
        assert_eq!(Op::from("  Operation: new = old + 19").eval1(1), 6);
        assert_eq!(Op::from("  Operation: new = old * old").eval1(9), 27);
        assert_eq!(Op::from("  Operation: new = old / 19").eval1(57), 1);
    }

    #[test]
    fn test_parse_test() {
        let test = Test::from("  Test: divisible by 23\nIf true: throw to monkey 2\nIf false: throw to monkey 3\n".lines());
        assert_eq!(test.eval(1), 3);
        assert_eq!(test.eval(23), 2);
    }

    #[test]
    fn test_parse_monkey() {
        let monkey = Monkey::from("Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n");
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.operation.eval1(1), 6);
        assert_eq!(monkey.test.eval(1), 3);
        assert_eq!(monkey.test.eval(23), 2);
    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse(TEST);
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_1round() {
        let monkeys = parse(TEST);
        let len = monkeys.len();
        let mult: u64 = monkeys.iter().map(|m| m.borrow().test.x).product();
        for i in 0..len {
            let mut monkey = monkeys[i].borrow_mut();
            while let Some((next, item)) = monkey.inspect1() {
                monkeys[next].borrow_mut().add_item(item);
            }
        }
        assert_eq!(monkeys[0].borrow().items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].borrow().items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 2713310158);
    }
}
