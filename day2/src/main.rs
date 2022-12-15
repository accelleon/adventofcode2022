
#[derive(Clone)]
struct Shape(i32);

impl From<&char> for Shape {
    fn from(c: &char) -> Self {
        match c {
            'A' | 'X' => Shape(1),
            'B' | 'Y' => Shape(2),
            'C' | 'Z' => Shape(3),
            _ => panic!("Invalid shape"),
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Shape {
    // Return Greater if self beats other (self == 1 ? other == 3 : self > other)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.0, other.0) {
            (3, 1) => Some(std::cmp::Ordering::Less),
            (1, 3) => Some(std::cmp::Ordering::Greater),
            (x, y) => x.partial_cmp(&y),
        }
    }
}

impl Shape {
    // Return the shape that wins against this
    fn winner(&self) -> Self {
        match self.0 {
            1 => Shape(2),
            2 => Shape(3),
            3 => Shape(1),
            _ => panic!("Invalid shape"),
        }
    }

    // Return the shape that loses against this
    fn loser(&self) -> Self {
        match self.0 {
            1 => Shape(3),
            2 => Shape(1),
            3 => Shape(2),
            _ => panic!("Invalid shape"),
        }
    }
}

struct Game {
    player: Shape,
    opponent: Shape,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let opponent = Shape::from(&chars.next().unwrap());
        Game {
            opponent: opponent.clone(),
            // X - we should play the losing shape
            // Y - we should play the drawing shape
            // Z - we should play the winning shape
            player: match chars.next().unwrap() {
                'X' => opponent.loser(),
                'Y' => opponent,
                'Z' => opponent.winner(),
                _ => panic!("Invalid shape"),
            },
        }
    }
}

impl Game {
    fn score(&self) -> i32 {
        match self.player.partial_cmp(&self.opponent) {
            Some(std::cmp::Ordering::Greater) => self.player.0 + 6,
            Some(std::cmp::Ordering::Less) => self.player.0,
            _ => self.player.0 + 3,
        }
    }
}

fn calc_score(input: &str) -> i32 {
    // Input is a line of shape pairs, e.g. "A X"
    // Map lines to pairs of shapes
    let pairs = input
        .replace(" ", "")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Remove space between chars
            let mut chars = line.chars();
            (Shape::from(&chars.next().unwrap()), Shape::from(&chars.next().unwrap()))
        })
        .collect::<Vec<_>>();

    // Calculate score
    // First Shape is opponent, second is player
    // - Shape you selected is worth its inner value
    // - If a shape beats the other, its 6 points
    // - If a draw its 3 points
    pairs
        .iter()
        .map(|(a, b)| match b.partial_cmp(&a) {
            Some(std::cmp::Ordering::Greater) => b.0 + 6,
            Some(std::cmp::Ordering::Less) => b.0,
            _ => b.0 + 3,
        })
        .sum()
}

fn calc_score2(input: &str) -> i32 {
    // Input is a line of shape pairs, e.g. "A X"
    // Map lines to pairs of shapes
    let pairs = input
        .replace(" ", "")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Game::from(line))
        .collect::<Vec<_>>();

    // Calculate score
    pairs.iter().map(|game| game.score()).sum()
}

fn main() {
    // Read in input from input.txt
    let input = include_str!("input.txt");
    println!("{}", calc_score(&input));
    println!("{}", calc_score2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = 
        "A Y\n\
        B X\n\
        C Z";

        assert_eq!(calc_score(input), 15);
    }

    #[test]
    fn it_works2() {
        let input = 
        "A Y\n\
        B X\n\
        C Z";

        assert_eq!(calc_score2(input), 12);
    }
}
