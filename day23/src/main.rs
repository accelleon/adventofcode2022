use std::collections::{HashMap, HashSet};
use itertools::{Itertools, TupleWindows};

fn parse(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

type Direction = ([(i32, i32); 3], (i32, i32));
const DIRECTIONS: [Direction; 4] = [
    // North
    ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
    // South
    ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
    // West
    ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
    // East
    ([(1, -1), (1, 0), (1, 1)], (1, 0)),
];

struct Map<T> where T: Iterator<Item = (&'static Direction, &'static Direction, &'static Direction, &'static Direction)> {
    elves: HashSet<(i32, i32)>,
    dir_iter: T,
}

impl<T> Map<T> where T: Iterator<Item = (&'static Direction, &'static Direction, &'static Direction, &'static Direction)> {
    fn new(elves: HashSet<(i32, i32)>, dir_iter: T) -> Self {
        Self {
            elves,
            dir_iter,
        }
    }

    fn step(&mut self) -> bool {
        let mut new_spots = HashMap::new();
        let round_dir = self.dir_iter.next().unwrap();
        // Build an array from the above tuple so we can iterate over it
        let dirs = [
            round_dir.0,
            round_dir.1,
            round_dir.2,
            round_dir.3,
        ];
        // Move each elf according to the directions
        for elf in self.elves.iter() {
            if dirs.iter().any(|(points, _)| points.iter().any(|(x, y)| self.elves.contains(&(elf.0 + x, elf.1 + y)))) {
                new_spots.insert(
                    elf.clone(),
                    dirs.iter()
                        .find(|(points, _)| {
                            points.iter().all(|(x, y)| !self.elves.contains(&(elf.0 + x, elf.1 + y)))
                        })
                        .map(|(_, (dx, dy))| {
                            (elf.0 + dx, elf.1 + dy)
                        }).unwrap_or(*elf)
                );
            }
        }
        // If no elves moved, we're done
        if new_spots.is_empty() {
            return false;
        }
        // Ensure no elves proposing to move to the same spot
        // If they are, move them back to their original spot
        let news = new_spots.values().cloned().collect::<Vec<_>>();
        for new in news.iter() {
            if new_spots.values().filter(|x| **x == *new).count() > 1 {
                new_spots.iter_mut().filter(|(_, v)| **v == *new).for_each(|(k, v)| *v = *k);
            }
        }

        for (old, new) in new_spots.iter() {
            self.elves.remove(&old);
            self.elves.insert(*new);
        }
        true
    }

    fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        self.elves.iter().fold(
            ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
            |((min_x, min_y), (max_x, max_y)), (x, y)| {
                (
                    (min_x.min(*x), min_y.min(*y)),
                    (max_x.max(*x), max_y.max(*y)),
                )
            }
        )
    }

    fn empty_in_bounds(&self) -> i32 {
        // Return the number of empty spots in the bounding box
        let ((min_x, min_y), (max_x, max_y)) = self.bounding_box();
        (min_x..=max_x).flat_map(|x| (min_y..=max_y).map(move |y| (x, y))).filter(|p| !self.elves.contains(p)).count() as i32
    }
}

fn part1(input: &str) -> i32 {
    let elves = parse(input);
    let dir_iter: TupleWindows<_, (_, _, _, _)> = DIRECTIONS.iter().cycle().tuple_windows();
    let mut map = Map::new(elves, dir_iter);
    for _ in 0..10 {
        map.step();
    }
    map.empty_in_bounds()
}

fn part2(input: &str) -> i32 {
    let elves = parse(input);
    let dir_iter: TupleWindows<_, (_, _, _, _)> = DIRECTIONS.iter().cycle().tuple_windows();
    let mut map = Map::new(elves, dir_iter);
    let mut round = 1;
    while map.step() {
        round += 1;
    }
    round
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use itertools::TupleWindows;
    use std::{collections::HashSet, hash::Hash};

    use super::*;

    const TEST1: &'static str =
   ".....\n\
    ..##.\n\
    ..#..\n\
    .....\n\
    ..##.\n\
    .....";

    const TEST2: &'static str = 
   "..............\n\
    ..............\n\
    .......#......\n\
    .....###.#....\n\
    ...#...#.#....\n\
    ....#...##....\n\
    ...#.###......\n\
    ...##.#.##....\n\
    ....#..#......\n\
    ..............\n\
    ..............\n\
    ..............";

    fn my_eq<T>(a: &HashSet<T>, b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    #[test]
    fn test_windows() {
        let mut dir_iter: TupleWindows<_, (_, _, _, _)> = DIRECTIONS.iter().cycle().tuple_windows();
        assert_eq!(dir_iter.next().unwrap(), (&DIRECTIONS[0], &DIRECTIONS[1], &DIRECTIONS[2], &DIRECTIONS[3]));
        assert_eq!(dir_iter.next().unwrap(), (&DIRECTIONS[1], &DIRECTIONS[2], &DIRECTIONS[3], &DIRECTIONS[0]));
        assert_eq!(dir_iter.next().unwrap(), (&DIRECTIONS[2], &DIRECTIONS[3], &DIRECTIONS[0], &DIRECTIONS[1]));
        assert_eq!(dir_iter.next().unwrap(), (&DIRECTIONS[3], &DIRECTIONS[0], &DIRECTIONS[1], &DIRECTIONS[2]));
        assert_eq!(dir_iter.next().unwrap(), (&DIRECTIONS[0], &DIRECTIONS[1], &DIRECTIONS[2], &DIRECTIONS[3]));
    }

    #[test]
    fn test_step() {
        let elves = parse(TEST1);
        let dir_iter: TupleWindows<_, (_, _, _, _)> = DIRECTIONS.iter().cycle().tuple_windows();
        let mut map = Map::new(elves, dir_iter);
        assert!(my_eq(&map.elves, &[(2, 1), (3, 1), (2, 2), (2, 4), (3, 4)]));
        map.step();
        assert!(my_eq(&map.elves, &[(2, 0), (3, 0), (2, 2), (2, 4), (3, 3)]));
        map.step();
        assert!(my_eq(&map.elves, &[(2, 1), (3, 1), (1, 2), (2, 5), (4, 3)]));
        map.step();
        assert!(my_eq(&map.elves, &[(2, 0), (4, 1), (0, 2), (2, 5), (4, 3)]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST2), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST2), 20);
    }
}
