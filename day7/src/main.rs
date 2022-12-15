use std::{collections::HashMap, hash::Hash};

fn sizes(input: &str) -> HashMap<String, usize> {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _name] => {
                let size: usize = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let name = affected[..=idx].join("/");
                    *sizes.entry(name).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    sizes
}

fn part1(sizes: &HashMap<String, usize>) -> usize {
    sizes.values().filter(|&&size| size < 100000).sum()
}

fn part2(sizes: &HashMap<String, usize>) -> usize {
    let avail = 70_000_000 - *sizes.get("/").unwrap();
    *sizes.values().filter(|&&size| avail + size >= 30_000_000).min().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let input = sizes(&input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "$ cd /\n\
    $ ls\n\
    dir a\n\
    14848514 b.txt\n\
    8504156 c.dat\n\
    dir d\n\
    $ cd a\n\
    $ ls\n\
    dir e\n\
    29116 f\n\
    2557 g\n\
    62596 h.lst\n\
    $ cd e\n\
    $ ls\n\
    584 i\n\
    $ cd ..\n\
    $ cd ..\n\
    $ cd d\n\
    $ ls\n\
    4060174 j\n\
    8033020 d.log\n\
    5626152 d.ext\n\
    7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&sizes(INPUT)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&sizes(INPUT)), 24933642);
    }
}
