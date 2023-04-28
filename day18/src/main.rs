use std::collections::HashSet;

struct Bounds {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

// Parse each line as a (i32, i32, i32) tuple
fn parse_input(input: &str) -> (HashSet<(i32, i32, i32)>, Bounds) {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut z_min = 0;
    let mut z_max = 0;
    (
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse().unwrap();
                x_min = x_min.min(x - 1);
                x_max = x_max.max(x + 1);
                let y = parts.next().unwrap().parse().unwrap();
                y_min = y_min.min(y - 1);
                y_max = y_max.max(y + 1);
                let z = parts.next().unwrap().parse().unwrap();
                z_min = z_min.min(z - 1);
                z_max = z_max.max(z + 1);
                (x, y, z)
            })
            .collect(),
        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        })
}

fn part1(input: &str) -> usize {
    let (points, _) = parse_input(input);
    points.iter().fold(0, |acc, point| {
        let mut sides = 6;
        let check = vec![
            (point.0 + 1, point.1, point.2),
            (point.0 - 1, point.1, point.2),
            (point.0, point.1 + 1, point.2),
            (point.0, point.1 - 1, point.2),
            (point.0, point.1, point.2 + 1),
            (point.0, point.1, point.2 - 1),
        ];
        for side in check {
            if points.contains(&side) {
                sides -= 1;
            }
        }
        acc + sides
    })
}
fn part2(input: &str) -> usize {
    // Use breadth first search to find the number of points of lava
    // that can be pathed to from the source
    let (points, bounds) = parse_input(input);
    let mut queue = vec![(bounds.x_min,bounds.y_min,bounds.z_min)];
    let mut visited = HashSet::new();
    let mut count = 0;
    while !queue.is_empty() {
        let mut next = vec![];
        for point in queue {
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            let mut check = vec![];
            // Build a list of all the points around this one
            // within bounds
            if point.0 > bounds.x_min {
                check.push((point.0 - 1, point.1, point.2));
            }
            if point.0 < bounds.x_max {
                check.push((point.0 + 1, point.1, point.2));
            }
            if point.1 > bounds.y_min {
                check.push((point.0, point.1 - 1, point.2));
            }
            if point.1 < bounds.y_max {
                check.push((point.0, point.1 + 1, point.2));
            }
            if point.2 > bounds.z_min {
                check.push((point.0, point.1, point.2 - 1));
            }
            if point.2 < bounds.z_max {
                check.push((point.0, point.1, point.2 + 1));
            }
            for side in check {
                if !points.contains(&side) {
                    next.push(side);
                } else {
                    count += 1;
                }
            }
        }
        queue = next;
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "2,2,2\n\
    1,2,2\n\
    3,2,2\n\
    2,1,2\n\
    2,3,2\n\
    2,2,1\n\
    2,2,3\n\
    2,2,4\n\
    2,2,6\n\
    1,2,5\n\
    3,2,5\n\
    2,1,5\n\
    2,3,5";

    #[test]
    fn test_asdf() {
        assert_eq!(part1("1,1,1\n2,1,1"), 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 58);
    }
}
