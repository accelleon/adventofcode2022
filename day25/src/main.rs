#![feature(int_roundings)]
use std::{
    ops, iter::Sum, fmt,
};

#[derive(Debug)]
struct Snafu(i64);

impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        // Base 5 number except we range from -2 - 2
        let mut num = 0;
        let mut fd = 5_i64.pow(s.len() as u32 - 1);
        for c in s.chars() {
            num += match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Invalid character {}", c),
            } * fd;
            fd /= 5;
        }
        Snafu(num)
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut num = self.0 as f64;
        let mut f = 1;
        let mut i = 0;
        let mut fd = num;
        while fd > 0.0 {
            fd = (fd / 5.0).round();
            f *= 5;
            i += 1;
        }

        for _ in 0..i {
            f /= 5;
            let d = (num / f as f64).round() as i64;
            num -= (d * f) as f64;
            write!(out, "{}", match d {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!("Invalid digit {}", d),
            })?;
        }

        Ok(())
    }
}

impl ops::Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

impl Sum<Self> for Snafu {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Snafu(0), |a, b| a + b)
    }
}

fn part1(input: &str) -> Snafu {
    input.lines().filter(|l| !l.is_empty()).map(|l| Snafu::from(l)).sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1=-0-2\n\
    12111\n\
    2=0=\n\
    21\n\
    2=01\n\
    111\n\
    20012\n\
    112\n\
    1=-1=\n\
    1-12\n\
    12\n\
    1=\n\
    122";

    #[test]
    fn test_snafu() {
        let s = Snafu::from("1=-0-2");
        assert_eq!(s.0, 1747);
        assert_eq!(s.to_string(), "1=-0-2");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST).to_string(), "2=-1=0");
    }
}
