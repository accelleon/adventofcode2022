use std::{collections::HashSet, ops::Add};
use itertools::Itertools;

type Point = (i32, i32);
type Points = HashSet<Point>;

pub fn for_each_window_mut<T, F>(slice: &mut [T], size: usize, mut function: F)
where
    F: FnMut(&mut [T])
{
    for start in 0..=(slice.len().saturating_sub(size)) {
        function(&mut slice[start..][..size]);
    }
}

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    fn point(&self) -> Point {
        match self {
            Self::Up(_) => (0, -1),
            Self::Down(_) => (0, 1),
            Self::Left(_) => (-1, 0),
            Self::Right(_) => (1, 0),
        }
    }

    fn inner(&self) -> i32 {
        match *self {
            Self::Up(n) => n,
            Self::Down(n) => n,
            Self::Left(n) => n,
            Self::Right(n) => n,
        }
    }
}

struct State {
    points: Points,
    head: Point,
    tail: Point,
}

impl State {
    fn new() -> Self {
        let mut points = Points::new();
        points.insert((0, 0));
        Self {
            points,
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let (dx, dy) = direction.point();
    
        for _ in 0..direction.inner() {
            let (x, y) = self.head;
            self.head = (x + dx, y + dy);
            self.update_tail();
        }
    }

    fn update_tail(&mut self) {
        // Tail must never be further than 1 away from the head
        // Only move diagonally if the head is not in the same row or column
        let (x, y) = self.head;
        let (tx, ty) = self.tail;
        // If we're adjacent including diagonally skip
        if (x - tx).abs() <= 1 && (y - ty).abs() <= 1 {
            return;
        }
        let (dx, dy) = if x == tx {
            (0, (y - ty).signum())
        } else if y == ty {
            ((x - tx).signum(), 0)
        } else {
            ((x - tx).signum(), (y - ty).signum())
        };
        self.tail = (tx + dx, ty + dy);
        // We record the points visited
        self.points.insert(self.tail);
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            if let Some((dir, n)) = line.split_whitespace().collect_tuple() {
                let n = n.parse().unwrap();
                match dir {
                    "U" => Direction::Up(n),
                    "D" => Direction::Down(n),
                    "L" => Direction::Left(n),
                    "R" => Direction::Right(n),
                    _ => panic!("Invalid direction"),
                }
            } else {
                panic!("Invalid line");
            }
        }).collect()
}

fn part1(input: &str) -> i32 {
    parse(input)
    .iter()
    .fold(State::new(), |mut state, direction| {
        state.move_head(direction);
        state
    }).points.len() as i32
}

fn part2(input: &str) -> i32 {
    let (_, visited) = parse(input)
    .iter()
    .fold((vec![(0, 0); 10], Points::new()), |(mut points, mut visited), direction| {
        let (dx, dy) = direction.point();
        for _ in 0..direction.inner() {
            // Update the head
            let h = points.get_mut(0).unwrap();
            let (ox, oy) = *h;
            *h = (ox + dx, oy + dy);
            for_each_window_mut(&mut points, 2, |ps| {
                let (x, y) = *ps.get(0).unwrap();
                let tail = ps.get_mut(1).unwrap();
                let (tx, ty) = *tail;
                // If we're adjacent including diagonally skip
                if (x - tx).abs() <= 1 && (y - ty).abs() <= 1 {
                    return;
                }
                let (dx, dy) = if x == tx {
                    (0, (y - ty).signum())
                } else if y == ty {
                    ((x - tx).signum(), 0)
                } else {
                    ((x - tx).signum(), (y - ty).signum())
                };
                *tail = (tx + dx, ty + dy);
            });
            visited.insert(*points.get(9).unwrap());
        }
        (points, visited)
    });
    visited.len() as i32
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "R 4\n\
    U 4\n\
    L 3\n\
    D 1\n\
    R 4\n\
    D 1\n\
    L 5\n\
    R 2";

    const INPUT2: &'static str = "R 5\n\
    U 8\n\
    L 8\n\
    D 3\n\
    R 17\n\
    D 10\n\
    L 25\n\
    U 20";

    use super::*;
    #[test]
    fn test_parse() {
        let directions = parse(INPUT);
        assert_eq!(directions.len(), 8);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 36);
    }
}
