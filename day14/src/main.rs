use std::collections::HashSet;

fn parse_rocks(input: &str) -> HashSet<(i32, i32)> {
    // Each line is a continuous rock
    input.lines().flat_map(|line| {
        // Each rock is a series of points
        line.split(" -> ").map(|rock| {
            // Each point is a comma-separated pair of coordinates
            let mut coords = rock.split(',').map(|coord| coord.parse::<i32>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| {
            // Return a set of all points between the two points
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let mut points = HashSet::new();
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    points.insert((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    points.insert((x, y1));
                }
            }
            points
        })
        .flatten()
        .collect::<Vec<_>>()
    })
    .collect()
}

fn part1(input: &str) -> usize {
    let mut set = parse_rocks(input);
    let mut count = 0;

    // Find the height y value
    // Past this we consider the sand lost
    let max_y = set.iter().map(|(_, y)| y).max().unwrap() + 1;

    // Sand starts at 500,0
    // Move down first
    // If we hit something move diagonally left
    // If we cannot then move diagonally right

    let mut x = 500;
    let mut y = 0;

    while y < max_y {
        // Can we move down?
        if !set.contains(&(x, y + 1)) {
            y += 1;
        } else {
            // Can we move diagonally left?
            if !set.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else {
                // Can we move diagonally right?
                if !set.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                } else {
                    // We settled
                    count += 1;
                    set.insert((x, y));
                    x = 500;
                    y = 0;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut set = parse_rocks(input);
    let mut count = 0;

    // Find the height y value
    // Past this we consider the sand lost
    let max_y = set.iter().map(|(_, y)| y).max().unwrap() + 2;

    // Sand starts at 500,0
    // Move down first
    // If we hit something move diagonally left
    // If we cannot then move diagonally right

    let mut x = 500;
    let mut y = 0;

    while !set.contains(&(500, 0)) {
        // Floor at max_y
        if y == max_y-1 {
            count += 1;
            set.insert((x, y));
            x = 500;
            y = 0;
        }

        // Can we move down?
        if !set.contains(&(x, y + 1)) {
            y += 1;
        } else {
            // Can we move diagonally left?
            if !set.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else {
                // Can we move diagonally right?
                if !set.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                } else {
                    // We settled
                    count += 1;
                    set.insert((x, y));
                    x = 500;
                    y = 0;
                }
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


#[cfg(test)]
mod test {
    use super::*;

    const TEST: &'static str = "498,4 -> 498,6 -> 496,6\n\
    503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse_rocks() {
        let rocks = parse_rocks(TEST);
        assert_eq!(rocks.len(), 20);
        assert!(rocks.contains(&(498, 4)));
        assert!(rocks.contains(&(498, 5)));
        assert!(rocks.contains(&(498, 6)));
        assert!(rocks.contains(&(497, 6)));
        assert!(rocks.contains(&(496, 6)));
        assert!(rocks.contains(&(503, 4)));
        assert!(rocks.contains(&(502, 4)));
        assert!(rocks.contains(&(502, 5)));
        assert!(rocks.contains(&(502, 6)));
        assert!(rocks.contains(&(502, 7)));
        assert!(rocks.contains(&(502, 8)));
        assert!(rocks.contains(&(502, 9)));
        assert!(rocks.contains(&(501, 9)));
        assert!(rocks.contains(&(500, 9)));
        assert!(rocks.contains(&(499, 9)));
        assert!(rocks.contains(&(498, 9)));
        assert!(rocks.contains(&(497, 9)));
        assert!(rocks.contains(&(496, 9)));
        assert!(rocks.contains(&(495, 9)));
        assert!(rocks.contains(&(494, 9)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 93);
    }
}
