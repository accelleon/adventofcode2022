use lazy_regex::regex;
use std::collections::HashSet;

#[inline]
fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

// Return an iterator of all points on the line between a exclusive and b inclusive
fn diagonal(a: (i32, i32), b: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x1;
    let mut y = y1;
    std::iter::from_fn(move || {
        if x == x2 && y == y2 {
            None
        } else {
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
            Some((x, y))
        }
    })
}

struct Range {
    sensor: (i32, i32),
    beacon: (i32, i32),
    dist: i32,
}

impl Range {
    fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Range {
        Range { sensor, beacon, dist: manhattan_dist(sensor, beacon) }
    }

    fn char(&self, p: (i32, i32)) -> u8 {
        if p == self.sensor {
            3
        } else if p == self.beacon {
            3
        } else if manhattan_dist(self.sensor, p) <= self.dist {
            2
        } else if manhattan_dist(self.sensor, p) == self.dist + 1 {
            1
        } else {
            0
        }
    }

    // The points are all points on the edges of a rectangle with corners:
    // (self.sensor.0, self.sensor.1 + dist+1)
    // (self.sensor.0 + dist+1, self.sensor.1)
    // (self.sensor.0, self.sensor.1 - dist+1)
    // (self.sensor.0 - dist+1, self.sensor.1)
    fn points(&self, min: (i32, i32), max: (i32, i32)) -> HashSet<(i32, i32)> {
        // Build a slice of the corners
        [
            (self.sensor.0, self.sensor.1 + self.dist + 1),
            (self.sensor.0 + self.dist + 1, self.sensor.1),
            (self.sensor.0, self.sensor.1 - self.dist - 1),
            (self.sensor.0 - self.dist - 1, self.sensor.1),
            (self.sensor.0, self.sensor.1 + self.dist + 1),
        ].windows(2).flat_map(|window| {
            diagonal(window[0], window[1]).filter(|p| p.0 >= min.0 && p.0 <= max.0 && p.1 >= min.1 && p.1 <= max.1)
        }).collect()
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Range {
        let re = regex!(r"x=([-\d]+), y=([-\d]+)[:\w\s]+x=([-\d]+), y=([-\d]+)");
        let caps = re.captures(s).unwrap();
        let sensor: (i32, i32) = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let beacon: (i32, i32) = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        Range::new(sensor, beacon)
    }
}

fn part1(input: &str) -> i32 {
    let ranges: Vec<_> = input.lines().filter(|line| !line.is_empty()).map(|line| Range::from(line)).collect();
    // Calculate the min and max x values covered by the ranges
    let y = 2_000_000;
    let max_x = ranges.iter().map(|r| r.sensor.0 + r.dist).max().unwrap();
    let min_x = ranges.iter().map(|r| r.sensor.0 - r.dist).min().unwrap();
    // Build a set of all points at y that can't contain a beacon
    (min_x..=max_x).filter(|x| {
        ranges.iter().map(|r| r.char((*x, y))).max().unwrap() == 2
    }).count() as i32
}

fn part2(input: &str) -> i64 {
    let ranges: Vec<_> = input.lines().filter(|line| !line.is_empty()).map(|line| Range::from(line)).collect();
    ranges.iter().find_map(|r|
        r.points((0,0), (4_000_000, 4_000_000)).into_iter().find(|p| {
            ranges.iter().map(|r| r.char(*p)).max().unwrap() == 1
        })
    ).map(|(x, y)| x as i64 * 4_000_000 + y as i64).unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
    Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
    Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
    Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
    Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
    Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
    Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
    Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
    Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
    Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
    Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
    Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
    Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_diagonal() {
        let p1 = (0, 0);
        let p2 = (4, 4);
        let ps: Vec<_> = diagonal(p1, p2).collect();
        assert_eq!(ps, vec![(1, 1), (2, 2), (3, 3),(4, 4)]);
    }

    #[test]
    fn test_parse() {
        let ranges: Vec<_> = TEST.lines().filter(|line| !line.is_empty()).map(|line| Range::from(line)).collect();
        // calculate min and max x and y values
        let max_x = ranges.iter().map(|r| r.sensor.0 + r.dist).max().unwrap();
        let min_x = ranges.iter().map(|r| r.sensor.0 - r.dist).min().unwrap();
        let max_y = ranges.iter().map(|r| r.sensor.1 + r.dist).max().unwrap();
        let min_y = ranges.iter().map(|r| r.sensor.1 - r.dist).min().unwrap();

        // Print out the map
        for y in min_y..=max_y {
            print!("{:3} ", y);
            for x in min_x..=max_x {
                print!("{}", match ranges.iter().map(|r| r.char((x, y))).max().unwrap() {
                    0 => '.',
                    1 => '#',
                    2 => 'X',
                    3 => 'O',
                    _ => panic!(),
                });
            }
            println!();
        }
    }

    #[test]
    fn test_part1() {
        let ranges: Vec<_> = TEST.lines().filter(|line| !line.is_empty()).map(|line| Range::from(line)).collect();
        // Calculate the min and max x values covered by the ranges
        let y = 10;
        let max_x = ranges.iter().map(|r| r.sensor.0 + r.dist).max().unwrap();
        let min_x = ranges.iter().map(|r| r.sensor.0 - r.dist).min().unwrap();
        // Build a set of all points at y that can't contain a beacon
        let n = (min_x..=max_x).filter(|x| {
            ranges.iter().map(|r| r.char((*x, y))).max().unwrap() == 2
        }).count() as i32;
        // Return the number of points with y=10
        assert_eq!(
            n,
            26
        );
    }

    #[test]
    fn test_part2() {
        let ranges: Vec<_> = TEST.lines().filter(|line| !line.is_empty()).map(|line| Range::from(line)).collect();
        // Build a vector of all points that occur on at least 2 sensor boundaries
        let freq = ranges.iter().find_map(|r|
            r.points((0,0), (20, 20)).into_iter().find(|p| {
                ranges.iter().map(|r| r.char(*p)).max().unwrap() == 1
            })
        ).map(|(x, y)| x as i64 * 4_000_000 + y as i64).unwrap();
        assert_eq!(freq, 56000011);
    }
}
