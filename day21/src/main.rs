use std::collections::HashMap;
use lazy_regex::regex;

struct Operation<'a> {
    left: &'a str,
    right: &'a str,
    op: fn(i64, i64) -> i64,
    // These are used to calculate the other side of the operation
    // given the output and one side of the operation
    /// (c, a) -> b
    calc_right: fn(i64, i64) -> i64,
    /// (c, b) -> a
    calc_left: fn(i64, i64) -> i64,
}

enum Monkey<'a> {
    Me,
    Number(i64),
    Operation(Operation<'a>),
}

impl<'a> Monkey<'a> {
    fn eval(&self, map: &HashMap<&str, Monkey<'_>>) -> i64 {
        match self {
            Monkey::Me => panic!("Monkey::Me should not be evaluated"),
            Monkey::Number(n) => *n,
            Monkey::Operation(op) => {
                let left = map.get(op.left).unwrap().eval(map);
                let right = map.get(op.right).unwrap().eval(map);
                (op.op)(left, right)
            }
        }
    }

    fn eval2(&self, map: &HashMap<&str, Monkey<'_>>) -> Option<i64> {
        match self {
            Monkey::Me => None,
            Monkey::Number(n) => Some(*n),
            Monkey::Operation(op) => {
                if let Some(left) = map.get(op.left).unwrap().eval2(map) {
                    if let Some(right) = map.get(op.right).unwrap().eval2(map) {
                        return Some((op.op)(left, right));
                    }
                }
                None
            }
        }
    }
}

fn parse_monkeys<'a>(s: &'a str) -> HashMap<&'a str, Monkey<'a>> {
    let mut monkeys = HashMap::new();
    let re = regex!(r"(\w+): (\w+|\d+)(?: ([*+-/]) (\w+))?");
    
    for line in s.lines().filter(|l| !l.is_empty()) {
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        if let Some(op) = caps.get(3) {
            let op = op.as_str();
            let right = caps.get(4).unwrap().as_str();
            monkeys.insert(name, Monkey::Operation(Operation {
                left,
                right,
                op: match op {
                    "+" => |a, b| a + b,
                    "-" => |a, b| a - b,
                    "*" => |a, b| a * b,
                    "/" => |a, b| a / b,
                    _ => unreachable!(),
                },
                calc_right: match op {
                    "+" => |c, a| c - a,
                    "-" => |c, a| a - c,
                    "*" => |c, a| c / a,
                    "/" => |c, a| a / c,
                    _ => unreachable!(),
                },
                calc_left: match op {
                    "+" => |c, b| c - b,
                    "-" => |c, b| c + b,
                    "*" => |c, b| c / b,
                    "/" => |c, b| c * b,
                    _ => unreachable!(),
                },
            }));
        } else {
            monkeys.insert(name, Monkey::Number(left.parse().unwrap()));
        }
    }
    monkeys
}

// Match either:
// root: pppw + sjmn
// dbpl: 5
fn parse_part2<'a>(s: &'a str) -> HashMap<&'a str, Monkey<'a>> {
    let mut monkeys = HashMap::new();
    let re = regex!(r"(\w+): (\w+|\d+)(?: ([*+-/]) (\w+))?");
    
    for line in s.lines().filter(|l| !l.is_empty()) {
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        if name == "humn" {
            monkeys.insert(name, Monkey::Me);
        } else if let Some(op) = caps.get(3) {
            let op = op.as_str();
            let right = caps.get(4).unwrap().as_str();
            if name == "root" {
                monkeys.insert(name, Monkey::Operation(Operation {
                    left,
                    right,
                    op: |a, b| (a == b) as i64,
                    calc_right: |_, a| a,
                    calc_left: |_, b| b,
                }));
            } else {
                monkeys.insert(name, Monkey::Operation(Operation {
                    left,
                    right,
                    op: match op {
                        "+" => |a, b| a + b,
                        "-" => |a, b| a - b,
                        "*" => |a, b| a * b,
                        "/" => |a, b| a / b,
                        _ => unreachable!(),
                    },
                    calc_right: match op {
                        "+" => |c, a| c - a,
                        "-" => |c, a| a - c,
                        "*" => |c, a| c / a,
                        "/" => |c, a| a / c,
                        _ => unreachable!(),
                    },
                    calc_left: match op {
                        "+" => |c, b| c - b,
                        "-" => |c, b| c + b,
                        "*" => |c, b| c / b,
                        "/" => |c, b| c * b,
                        _ => unreachable!(),
                    },
                }));
            }
        } else {
            monkeys.insert(name, Monkey::Number(left.parse().unwrap()));
        }
    }
    monkeys
}


// Work our way down the tree, calculating the values as we go that don't depend on the value of "humn"
fn inverse_node(node: &Monkey, monkeys: &HashMap<&'_ str, Monkey<'_>>, output: i64) -> Option<i64> {
    match node {
        &Monkey::Operation(ref op) => {
            return {
                let left = monkeys[op.left].eval2(monkeys);
                let right = monkeys[op.right].eval2(monkeys);
                match (left, right) {
                    // We have the left value and need the right
                    (Some(left), None) => inverse_node(&monkeys[op.right], monkeys, (op.calc_right)(output, left)),
                    // We have the right value and need the left
                    (None, Some(right)) => inverse_node(&monkeys[op.left], monkeys, (op.calc_left)(output, right)),
                    _ => unreachable!(),
                }
            }
        }
        &Monkey::Me => Some(output),
        _ => panic!("This should be an eval not inverse_node")
    }
}

fn eval_me(input: &str) -> i64 {
    let monkeys = parse_part2(input);
    // The root node ignores output, so we can just pass 0
    inverse_node(&monkeys["root"], &monkeys, 0).unwrap()
}

fn eval_root(input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    monkeys["root"].eval(&monkeys)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", eval_root(input));
    println!("Part 2: {}", eval_me(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "root: pppw + sjmn\n\
    dbpl: 5\n\
    cczh: sllz + lgvd\n\
    zczc: 2\n\
    ptdq: humn - dvpt\n\
    dvpt: 3\n\
    lfqf: 4\n\
    humn: 5\n\
    ljgn: 2\n\
    sjmn: drzm * dbpl\n\
    sllz: 4\n\
    pppw: cczh / lfqf\n\
    lgvd: ljgn * ptdq\n\
    drzm: hmdt - zczc\n\
    hmdt: 32";

    #[test]
    fn test_eval_root() {
        assert_eq!(eval_root(TEST), 152);
    }

    #[test]
    fn test_eval_me() {
        assert_eq!(eval_me(TEST), 301);
    }
}
