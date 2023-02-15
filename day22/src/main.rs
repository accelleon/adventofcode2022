use std::{vec::IntoIter, collections::HashMap};

// Workaround for the jankiness of iterator types (e.g. Map<Filter<Skip<Cycle<Rev<T>>>>> != Map<Filter<Skip<Cycle<T>>>>)
enum Either<T, U> {
    Left(T),
    Right(U),
}

impl <T, U, Item> Iterator for Either<T, U>
where
    T: Iterator<Item = Item>,
    U: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(iter) => iter.next(),
            Either::Right(iter) => iter.next(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Forward(u32),
    // Rotate 90 degrees to the left
    Left,
    // Rotate 90 degrees to the right
    Right,
}

fn parse_movements(input: &str) -> Vec<Move> {
    // 10R5L5R10L4R5L5
    let mut moves = Vec::new();
    let mut chars = input.trim().chars();
    let mut num: Option<u32> = None;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            let tmp = num.unwrap_or(0);
            num = Some(tmp * 10 + c.to_digit(10).unwrap());
        } else {
            if let Some(n) = num.take() {
                moves.push(Move::Forward(n));
            }
            match c {
                'L' => moves.push(Move::Left),
                'R' => moves.push(Move::Right),
                _ => panic!("Invalid move"),
            }
        }
    }
    // In case there is a number at the end
    if let Some(n) = num.take() {
        moves.push(Move::Forward(n));
    }
    moves
}

#[derive(Debug, PartialEq, Clone)]
enum MapSlot {
    Empty,
    Path,
    Rock,
}

fn parse_map(input: &str) -> Vec<Vec<MapSlot>> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                ' ' => MapSlot::Empty,
                '.' => MapSlot::Path,
                '#' => MapSlot::Rock,
                _ => panic!("Invalid map"),
            }
        }).collect()
    }).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Face {
    Bottom,
    Top,
    Left,
    Right,
    Front,
    Back,
}

fn parse_map2(input: &str) -> HashMap<Face, Vec<Vec<MapSlot>>> {
    let mut out = HashMap::new();
    let map = parse_map(input);

    // Break the map into 6 50x50 sections
    // Lets start with the top middle section (50, 0)
    // Find the first path slot on the first row
    let pos = map[0].iter().position(|slot| *slot == MapSlot::Path).unwrap();
    // Copy (pos, 0) to (pos + 50, 50)
    out.insert(Face::Front, map.iter().take(50).map(|row| {
        row.iter().skip(pos).take(50).cloned().collect()
    }).collect());

    // The next section is the top right section of the map (100, 0)
    // And will be the Right face of the cube
    let pos = pos + 50;
    out.insert(Face::Right, map.iter().take(50).map(|row| {
        row.iter().skip(pos).take(50).cloned().collect()
    }).collect());

    // The next section is the middle middle of the map (50, 50)
    // And will be the Bottom face of the cube
    out.insert(Face::Bottom, map.iter().skip(50).take(50).map(|row| {
        row.iter().skip(50).take(50).cloned().collect()
    }).collect());

    // Below the middle middle is the bottom middle of the map (50, 100)
    // And will be the Back face of the cube
    out.insert(Face::Back, map.iter().skip(100).take(50).map(|row| {
        row.iter().skip(50).take(50).cloned().collect()
    }).collect());

    // To the left of that section is the left face of the cube (0, 100)
    out.insert(Face::Left, map.iter().skip(50).take(50).map(|row| {
        row.iter().take(50).cloned().collect()
    }).collect());

    // Finally below that again is the top face of the cube (0, 150)
    out.insert(Face::Top, map.iter().skip(150).take(50).map(|row| {
        row.iter().take(50).cloned().collect()
    }).collect());

    // In theory there should be no more empty spaces
    assert!(map.iter().skip(200).all(|row| row.iter().all(|slot| *slot == MapSlot::Empty)));

    out
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<MapSlot>>,
    moves: IntoIter<Move>,
    pos: (usize, usize),
    dir: (i32, i32),
    visited: HashMap<(usize, usize), char>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        // Split input into map and moves, separated by a blank line
        let mut parts = input.split("\n\n");
        let map = parse_map(parts.next().unwrap());
        let moves = parse_movements(parts.next().unwrap()).into_iter();
        // Find starting position, which is the first path slot on the first row
        let pos = map[0].iter().position(|slot| *slot == MapSlot::Path).unwrap();
        Map {
            map,
            moves,
            pos: (pos, 0),
            // Start facing right
            dir: (1, 0),
            visited: HashMap::new(),
        }
    }
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, slot) in row.iter().enumerate() {
                if x == self.pos.0 && y == self.pos.1 {
                    print!("X");
                } else {
                    if let Some(c) = self.visited.get(&(x, y)) {
                        print!("{}", c);
                    } else {
                        match slot {
                            MapSlot::Empty => print!(" "),
                            MapSlot::Path => print!("."),
                            MapSlot::Rock => print!("#"),
                        }
                    }
                }
            }
            println!();
        }
    }

    fn move_forward(&mut self, n: u32) {
        let (dx, dy) = self.dir;
        let (x, y) = self.pos;

        // Build an iterator that moves forward n steps
        // Wrapping around to the next *not Empty* if we hit the edge
        let mut dir = match (dx, dy) {
            (dx, 0) => {
                Either::Left(
                    {
                        if dx > 0 {
                            Either::Left(self.map[y].iter().enumerate().cycle().skip(x + 1))
                        } else {
                            Either::Right(self.map[y].iter().enumerate().rev().cycle().skip(self.map[y].len() - x))
                        }
                    }
                        .filter(|(_, slot)| **slot != MapSlot::Empty)
                        .map(|(x, slot)| ((x, y), slot))
                )
            }
            (0, dy) => {
                Either::Right(
                    {
                        if dy > 0 {
                            Either::Left(self.map.iter().map(|row| row.get(x).unwrap_or(&MapSlot::Empty)).enumerate().cycle().skip(y + 1))
                        } else {
                            Either::Right(self.map.iter().map(|row| row.get(x).unwrap_or(&MapSlot::Empty)).enumerate().rev().cycle().skip(self.map.len() - y))
                        }
                    }
                        .filter(|(_, slot)| **slot != MapSlot::Empty)
                        .map(|(y, slot)| ((x, y), slot))
                )
            }
            _ => panic!("Invalid direction"),
        };

        // Take n steps
        for _ in 0..n {
            let ((x, y), slot) = dir.next().unwrap();
            match slot {
                MapSlot::Path => {
                    self.pos = (x, y);
                }
                MapSlot::Rock => {
                    break;
                }
                MapSlot::Empty => {
                    self.print();
                    panic!("Invalid move");
                },
            }
        }
    }

    fn turn(&mut self, dir: Move) {
        match dir {
            Move::Left => {
                self.dir = match self.dir {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!("Invalid direction"),
                }
            }
            Move::Right => {
                self.dir = match self.dir {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => panic!("Invalid direction"),
                }
            }
            _ => panic!("Invalid turn"),
        }
    }

    fn do_move(&mut self, cmd: Move) {
        match cmd {
            Move::Forward(n) => self.move_forward(n),
            Move::Left | Move::Right => self.turn(cmd),
        }
    }

    fn do_moves(&mut self) {
        while let Some(cmd) = self.moves.next() {
            self.do_move(cmd);
        }
    }
}

fn part1(input: &str) -> usize {
    let mut map = Map::from(input);
    map.do_moves();
    // Output is 1000 * y + 4 * x + direction
    // direction: right: 0, down: 1, left: 2, up: 3
    let dir = match map.dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!()
    };
    (map.pos.1 + 1) * 1000 + (map.pos.0 + 1) * 4 + dir
}

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = 
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse_movements() {
        let moves = parse_movements("10R5L5R10L4R5L5");
        assert_eq!(moves.len(), 13);
        assert_eq!(moves[0], Move::Forward(10));
        assert_eq!(moves[1], Move::Right);
        assert_eq!(moves[2], Move::Forward(5));
        assert_eq!(moves[3], Move::Left);
        assert_eq!(moves[4], Move::Forward(5));
        assert_eq!(moves[5], Move::Right);
        assert_eq!(moves[6], Move::Forward(10));
    }

    #[test]
    fn test_parse() {
        let map = Map::from(TEST);
        assert_eq!(map.moves.len(), 13);
        assert_eq!(map.pos, (8, 0));
        assert_eq!(map.dir, (1, 0));
        assert_eq!(map.map, vec![
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Rock],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Rock, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Rock, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Rock,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Rock],
            vec![MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Rock, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Path,  MapSlot::Path,  MapSlot::Rock,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Rock,  MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path,  MapSlot::Path, MapSlot::Path, MapSlot::Rock, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Rock, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Rock, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Rock, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path],
            vec![MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Empty, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Path, MapSlot::Rock, MapSlot::Path],
        ]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 6032);
    }

    #[test]
    fn test_parse2() {
        let mapstr = TEST.split("\n\n").nth(0).unwrap();
        let map = parse_map2(mapstr);
    }
}
