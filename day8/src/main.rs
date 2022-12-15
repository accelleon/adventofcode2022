use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect()
}

fn part1(input: &Vec<Vec<u32>>) -> usize {
    // Need to check the rows and columns
    // If all left of the current number are less than the current number add 1
    // If all right of the current number are less than the current number add 1
    // If all above the current number are less than the current number add 1
    // If all below the current number are less than the current number add 1

    // We can skip the outside edges because they will always be visible
    let len = input.len();
    (1..len-1)
        .cartesian_product(1..len-1)
        .map(|(y, x)| {
            let height = input[y][x];
            // Get all heights in the 4 directions around the current point
            let col = input.iter().map(|row| row[x]).collect::<Vec<u32>>();
            let (up, down) = col.split_at(y);
            let (left, right) = input[y].split_at(x);
            let up = up.iter().rev().all(|h| *h < height);
            let left = left.iter().rev().all(|h| *h < height);
            let right = right.iter().skip(1).all(|h| *h < height);
            let down = down.iter().skip(1).all(|h| *h < height);
            [up, down, left, right].iter().any(|b| *b)
        })
        .filter(|b| *b)
        .count()
        + (len - 1) * 4
}

fn part2(input: &Vec<Vec<u32>>) -> usize {
    // Need to find the distances to the nearest point that is greater or equal to the current point
    let len = input.len();
    (0..len)
        .cartesian_product(0..len)
        .map(|(y, x)| {
            let height = input[y][x];
            // Get all heights in the 4 directions around the current point
            let col = input.iter().map(|row| row[x]).collect::<Vec<u32>>();
            let (up, down) = col.split_at(y);
            let (left, right) = input[y].split_at(x);
            let up = up.iter().rev().collect::<Vec<&u32>>();
            let left = left.iter().rev().collect::<Vec<&u32>>();
            let right = right.iter().skip(1).collect::<Vec<&u32>>();
            let down = down.iter().skip(1).collect::<Vec<&u32>>();
            [up, down, left, right].iter().map(|hs| {
                hs.iter().position(|h| **h >= height).map(|a| a + 1).unwrap_or(hs.len())
            }).product()
        })
        .max().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "30373\n\
    25512\n\
    65332\n\
    33549\n\
    35390";

    #[test]
    fn it_parses() {
        let parsed = super::parse(INPUT);
        assert_eq!(parsed, vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
    }

    #[test]
    fn it_solves_part1() {
        let parsed = super::parse(INPUT);
        assert_eq!(super::part1(&parsed), 21);
    }

    #[test]
    fn it_solves_part2() {
        let parsed = super::parse(INPUT);
        assert_eq!(super::part2(&parsed), 8);
    }
}
