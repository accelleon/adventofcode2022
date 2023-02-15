use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
};

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

enum Either2<A, B> {
    A(A),
    B(B),
}

impl <A, B, Item> Iterator for Either2<A, B>
where
    A: Iterator<Item = Item>,
    B: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either2::A(iter) => iter.next(),
            Either2::B(iter) => iter.next(),
        }
    }
}

enum Either4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl <A, B, C, D, Item> Iterator for Either4<A, B, C, D>
where
    A: Iterator<Item = Item>,
    B: Iterator<Item = Item>,
    C: Iterator<Item = Item>,
    D: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either4::A(iter) => iter.next(),
            Either4::B(iter) => iter.next(),
            Either4::C(iter) => iter.next(),
            Either4::D(iter) => iter.next(),
        }
    }
}

// Helper functions to rotate bits in an u128 containing data of len < 128
#[inline]
fn rotate_l(x: u128, len: usize, n: usize) -> u128 {
    (x << n) | (x >> (len - n))
}

#[inline]
fn rotate_r(x: u128, len: usize, n: usize) -> u128 {
    (x >> n) | (x << (len - n))
}

struct Map {
    // One vec for each direction
    map: [Vec<u128>; 5],
    // Cache of blizzard locations at minute n
    cache: HashMap<usize, Vec<u128>>,
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),
    // Least common multiple of the x and y dimensions
    // Used to determine when we've looped back to the start of the blizzards
    lcm: usize,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut map = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let lines = input.lines().filter(|line| !line.is_empty());
        let max_y = lines.clone().count();
        let max_x = lines.clone().next().unwrap().chars().count();
        let mut start = (0, 0);
        let mut end = (0, max_y - 1);
        for (y, line) in lines.enumerate() {
            let mut north = 0;
            let mut south = 0;
            let mut east = 0;
            let mut west = 0;
            let mut wall = 0;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '^' => north |= 1 << max_x - x,
                    'v' => south |= 1 << max_x - x,
                    // Taking out the wall makes rotations easier
                    '>' => east |= 1 << max_x - x - 1,
                    '<' => west |= 1 << max_x - x - 1,
                    '#' => wall |= 1 << max_x - x,
                    _ => {
                        if y == 0 {
                            start.0 = x;
                        } else if y == max_y - 1 {
                            end.0 = x;
                        }
                    },
                }
            }
            // Don't push the first or last rows to blizzards
            map[4].push(wall);
            if y == 0 || y == max_y - 1 {
                continue;
            }
            map[0].push(north);
            map[1].push(south);
            map[2].push(east);
            map[3].push(west);
        }
        Map { map, max_x, max_y, cache: HashMap::new(), lcm: lcm(max_x-2, max_y-2), start, end }
    }
}

impl Map {
    /// Return a bitmap of blizzard locations at minute n
    fn _map_at(&self, n: usize) -> Vec<u128> {
        // Walls take up the top, bottom, left and right edges
        // So rotate the blizzards minus those lengths
        let len = self.max_x - 2;
        let ylen = self.max_y - 2;
        self.map.iter().enumerate().map(|(i, blizzards)| {
            if i == 4 {
                Either2::A(blizzards.iter().map(|blizzard| *blizzard))
            } else {
                // Pad the blizzards with a 0 before and after
                Either2::B(iter::once(0).chain(match i {
                        // North, return a cyclic iter starting at len - n % len and taking len elements advancing forwards 1
                        0 => Either4::A(blizzards.iter().cycle().skip(n % ylen).take(ylen).map(|blizzard| *blizzard)),
                        // South, return a cyclic iter starting at len - n % len and taking len elements advancing backwards 1
                        1 => Either4::B(blizzards.iter().cycle().skip(ylen - (n % ylen)).take(ylen).map(|blizzard| *blizzard)),
                        // Rotate right left 1 to accommodate the wall
                        // East, return an iterator of rotated right bits
                        2 => Either4::C(blizzards.iter().map(|blizzard| rotate_r(*blizzard, len, n % len) << 1)),
                        // West, return an iterator of rotated left bits
                        3 => Either4::D(blizzards.iter().map(|blizzard| rotate_l(*blizzard, len, n % len) << 1)),
                        // Wall, return an iterator of the same blizzard
                        _ => unreachable!(),
                    }).chain(iter::once(0)))
            }
        }).fold(vec![0; self.max_y], |mut out, blizzards| {
            out.iter_mut().zip(blizzards).for_each(|(out, blizzard)| {
                *out |= blizzard;
            });
            out
        })
    }

    fn map_at(&mut self, n: usize) -> &Vec<u128> {
        // Cache the blizzard locations at each minute
        // Accounting for the fact that the states repeat every lcm(max_x-2, max_y-2) minutes
        let n = n % self.lcm;
        if !self.cache.contains_key(&n) {
            self.cache.insert(n, self._map_at(n));
        }
        self.cache.get(&n).unwrap()
    }

    fn permutations(&mut self, (x, y): (usize, usize), n: usize) -> Vec<(usize, usize)> {
        let max_x = self.max_x;
        let max_y = self.max_y;
        let blizzards = self.map_at(n);
        [
            (0, 0),
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ].iter().filter(|(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;
            x >= 0 && x < max_x as isize && y >= 0 && y < max_y as isize && blizzards[y as usize] & (1 << max_x - x as usize) == 0
        }).map(|(dx, dy)| {
            ((x as isize + dx) as usize, (y as isize + dy) as usize)
        }).collect()
    }

    fn shortest_path(&mut self, start: (usize, usize), end: (usize, usize), startn: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut min_cost = usize::MAX;
        queue.push_back((start, startn));
        while let Some(((x, y), n)) = queue.pop_front() {
            // Break early if we've exceeded the minimum cost we've already found
            if n > min_cost {
                continue;
            }
            if (x, y) == end {
                min_cost = min_cost.min(n);
                continue;
            }
            if visited.contains(&((x, y), n)) {
                continue;
            }
            visited.insert(((x, y), n));
            for (x, y) in self.permutations((x, y), n + 1) {
                queue.push_back(((x, y), n + 1));
            }
        }
        min_cost
    }

    fn draw(&mut self, n: usize) {
        let max_y = self.max_y;
        let max_x = self.max_x;
        let blizzards = self.map_at(n);
        for y in 0..max_y {
            for x in 0..max_x {
                if blizzards[y] & (1 << max_x - x) != 0 {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut map = Map::from(include_str!("input.txt"));
    println!("Part 1: {}", map.shortest_path(map.start, map.end, 0));

    // Part 2: path to the end, back to the start, then back to the end
    let path1 = map.shortest_path(map.start, map.end, 0);
    let path2 = map.shortest_path(map.end, map.start, path1);
    let path3 = map.shortest_path(map.start, map.end, path2);
    println!("Part 2: {}", path3);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = 
   "#.#####\n\
    #.....#\n\
    #>....#\n\
    #.....#\n\
    #...v.#\n\
    #.....#\n\
    #####.#";

    const TEST2: &'static str =
   "#.######\n\
    #>>.<^<#\n\
    #.<..<<#\n\
    #>v.><>#\n\
    #<^v^^>#\n\
    ######.#";

    #[test]
    fn test_parse() {
        let map = Map::from(TEST1);
        assert_eq!(map.start, (1, 0));
        assert_eq!(map.end, (5, 6));
        assert_eq!(map.max_x, 7);
        assert_eq!(map.max_y, 7);
    }

    #[test]
    fn test_blizzard2() {
        let mut blizzards = Map::from(TEST1);
        blizzards.draw(0);
        blizzards.draw(1);
    }

    #[test]
    fn test_permutations() {
        let mut map = Map::from(TEST2);
        assert!(map.permutations((1, 0), 1).contains(&(1, 0)));
        assert!(map.permutations((1, 1), 2).contains(&(1, 2)));
        assert!(map.permutations((1, 2), 3).contains(&(1, 2)));
        assert!(map.permutations((1, 2), 4).contains(&(1, 1)));
        assert!(map.permutations((1, 1), 5).contains(&(2, 1)));
        assert!(map.permutations((2, 1), 6).contains(&(3, 1)));
        assert!(map.permutations((3, 1), 7).contains(&(3, 2)));
        assert!(map.permutations((3, 2), 8).contains(&(2, 2)));
        assert!(map.permutations((2, 2), 9).contains(&(2, 1)));
        // Wrong path
        assert!(map.permutations((1, 2), 10).contains(&(1, 1)));
        // Right path
        assert!(map.permutations((2, 1), 10).contains(&(3, 1)));
        assert!(map.permutations((3, 1), 11).contains(&(3, 1)));
        assert!(map.permutations((3, 1), 12).contains(&(3, 2)));
    }

    #[test]
    fn test_part1() {
        let mut map = Map::from(TEST2);
        assert_eq!(map.shortest_path(map.start, map.end, 0), 18);
    }

    #[test]
    fn test_part2() {
        let mut map = Map::from(TEST2);
        // Part 2: path to the end, back to the start, then back to the end
        let path1 = map.shortest_path(map.start, map.end, 0);
        assert_eq!(path1, 18);
        let path2 = map.shortest_path(map.end, map.start, path1);
        assert_eq!(path2, 41);
        let path3 = map.shortest_path(map.start, map.end, path2);
        assert_eq!(path3, 54);
    }
}
