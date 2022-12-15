use std::collections::{HashSet, VecDeque};

struct Board {
    board: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Board {
    fn successors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        // Return positions that are at most 1 lower than the current position
        let mut v = Vec::new();
        let h = self.board[y][x] - 1;
        if y > 0 && self.board[y - 1][x] >= h {
            v.push((x, y - 1));
        }
        if y < self.board.len() - 1 && self.board[y + 1][x] >= h {
            v.push((x, y + 1));
        }
        if x > 0 && self.board[y][x - 1] >= h {
            v.push((x - 1, y));
        }
        if x < self.board[y].len() - 1 && self.board[y][x + 1] >= h {
            v.push((x + 1, y));
        }
        v
    }

    fn part1(&self) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.end, 0));
        while let Some(((x, y), cost)) = queue.pop_front() {
            if (x, y) == self.start {
                return cost;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            for (x, y) in self.successors((x, y)) {
                queue.push_back(((x, y), cost + 1));
            }
        }
        unreachable!()
    }

    fn part2(&self) -> usize {
        // Instead of starting at the start, start at the end
        // goal is the first height of 'a' we reach
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.end, 0));
        while let Some(((x, y), cost)) = queue.pop_front() {
            if self.board[y][x] == b'a' {
                return cost;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            for (x, y) in self.successors((x, y)) {
                queue.push_back(((x, y), cost + 1));
            }
        }
        unreachable!()
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let board = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let mut v = line.as_bytes().to_vec();
                // S marks start, height 'a'
                // E marks end, height 'z'
                v.iter_mut().enumerate().for_each(|(x, c)| {
                    if *c == b'S' {
                        *c = b'a';
                        start = (x, y);
                    } else if *c == b'E' {
                        *c = b'z';
                        end = (x, y);
                    }
                });
                v
            })
            .collect::<Vec<_>>();
        Board { board, start, end }
    }
}

fn main() {
    let board = Board::from(include_str!("input.txt"));
    println!("Part 1: {}", board.part1());
    println!("Part 2: {}", board.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "Sabqponm\n\
    abcryxxl\n\
    accszExk\n\
    acctuvwj\n\
    abdefghi";

    #[test]
    fn test_parse() {
        let board = Board::from(TEST);
        assert_eq!(board.start, (0, 0));
        assert_eq!(board.end, (5, 2));
    }

    #[test]
    fn test_part1() {
        let board = Board::from(TEST);
        assert_eq!(board.part1(), 31);
    }

    #[test]
    fn test_part2() {
        let board = Board::from(TEST);
        assert_eq!(board.part2(), 29);
    }
}
