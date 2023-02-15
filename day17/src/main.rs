use std::{iter, collections::VecDeque};

// Bitarray structure
struct BitArray {
    bits: Vec<u64>,
    size: usize,
}

impl BitArray {
    fn new(size: usize) -> BitArray {
        let bits = vec![0; (size + 63) / 64];
        BitArray { bits, size }
    }

    fn get(&self, index: usize) -> bool {
        let word = index / 64;
        let bit = index % 64;
        (self.bits[word] & (1 << bit)) != 0
    }

    fn set(&mut self, index: usize, value: bool) {
        let word = index / 64;
        let bit = index % 64;
        if value {
            self.bits[word] |= 1 << bit;
        } else {
            self.bits[word] &= !(1 << bit);
        }
    }

    fn len(&self) -> usize {
        self.size
    }
}

// Repeating iterator over a bitarray
struct Pattern {
    pattern: BitArray,
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Pattern {
        let mut pattern = BitArray::new(s.len());
        for (i, c) in s.chars().enumerate() {
            // Set bit if move right, clear if move left
            pattern.set(i, c == '>');
        }
        Pattern { pattern }
    }
}

impl IntoIterator for Pattern {
    type Item = bool;
    type IntoIter = PatternIterator;

    fn into_iter(self) -> PatternIterator {
        PatternIterator {
            pattern: self.pattern,
            index: 0,
        }
    }
}

struct PatternIterator {
    pattern: BitArray,
    index: usize,
}

impl Iterator for PatternIterator {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.index >= self.pattern.len() {
            self.index = 0;
        }
        let value = self.pattern.get(self.index);
        self.index += 1;
        Some(value)
    }
}

struct Rock {
    width: u8,
    data: &'static [u8],
}

// Store rocks as an array of bits
// The first bit is the bottom right corner
const ROCKS: [Rock; 5] = [
    Rock {
        width: 4,
        data: &[0b1111],
    },
    Rock {
        width: 3,
        data: &[0b010, 0b111, 0b010],
    },
    Rock {
        width: 3,
        data: &[0b001, 0b001, 0b111],
    },
    Rock {
        width: 1,
        data: &[0b1, 0b1, 0b1, 0b1],
    },
    Rock {
        width: 2,
        data: &[0b11, 0b11],
    },
];

struct RockIterator {
    index: usize,
}

impl Iterator for RockIterator {
    type Item = &'static Rock;

    fn next(&mut self) -> Option<&'static Rock> {
        if self.index >= ROCKS.len() {
            self.index = 0;
        }
        let rock = &ROCKS[self.index];
        self.index += 1;
        Some(rock)
    }
}

struct Field {
    data: VecDeque<u8>,
    rocks: RockIterator,
    pattern: PatternIterator,
}

impl Field {
    fn new(pattern: &str) -> Field {
        let mut data = VecDeque::new();
        // Push the floor
        data.push_front(255);
        Field {
            data,
            rocks: RockIterator { index: 0 },
            pattern: Pattern::from(pattern).into_iter(),
        }
    }

    fn print(&self) {
        for row in self.data.iter().rev() {
            for i in 0..7 {
                if row & (1 << 7-i) != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn print_with_rock(&self, rock: &VecDeque<u8>) {
        for (row, rock_row) in self.data.iter().zip(rock.iter().chain(iter::repeat(&0))) {
            for i in 0..7 {
                if row & (1 << 7-i) != 0 {
                    print!("#");
                } else if rock_row & (1 << 7-i) != 0 {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // Add a rock to the field
    // Since we always spawn rocks 3 blocks above the highest block
    // We can simulate the first 3 steps before dealing with the actual field
    fn add_rock(&mut self) {
        let rock = self.rocks.next().unwrap();
        let next4 = self.pattern.by_ref().take(4).collect::<Vec<bool>>();

        // The rock starts 2 from the left and 3 above the highest block
        // X coordinate is the bottom right corner of the rock
        // We can cheat and simulate the next 4 jets before dealing with the field
        let width = rock.width;
        let mut x = 7 - 2 - width;
        for right in next4 {
            if right && x > 0 {
                x -= 1;
            } else if !right && x + width < 7 {
                x += 1;
            }
        }

        // At this point we have the x coordinate of the rock
        // Our next step is to move down
        // But we need to start checking for collisions

        // Push the height of the rock onto the field
        (0..rock.data.len()).for_each(|_| self.data.push_front(0) );
        // Build the bitmap to collision check
        let mut rock = rock.data.iter().map(|b| b << x).collect::<VecDeque<u8>>();
        // We advance the rock by pushing 0s onto the front
        rock.push_front(0);

        loop {
            // Check for collision
            if self.data.iter().zip(rock.iter()).any(|(a, b)| a & b != 0) {
                // We collided moving down
                // Re-add the row we popped and add the rock to the map
                rock.pop_front();
                self.data.iter_mut().zip(rock.iter()).for_each(|(a, b)| *a |= b);
                // And remove all rows until we encounter our first non-zero
                while self.data.front() == Some(&0) {
                    self.data.pop_front();
                }
                break;
            } else {
                // No collision, move down
                rock.push_front(0);
                // Now we need to check for collision on the next jet stream
                let next = self.pattern.next().unwrap();
                // Create a copy of the rock so we can revert
                let mut new_rock = rock.clone();
                // Avoid over/underflows
                if next && x > 0 {
                    // Move left
                    x -= 1;
                    new_rock.iter_mut().for_each(|b| *b <<= 1);
                } else if !next && x + width < 7 {
                    // Move right
                    x += 1;
                    new_rock.iter_mut().for_each(|b| *b >>= 1);
                }
                // Check for collision
                // If we collided we do nothing
                if self.data.iter().zip(new_rock.iter()).any(|(a, b)| a & b != 0) {
                    // We didn't collide, move the rock
                    rock = new_rock;
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut field = Field::new(input);
    for _ in 0..2022 {
        field.add_rock();
    }
    // Return the height
    field.data.len()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_pattern() {
        let pattern = Pattern::from(">>><");
        assert_eq!(pattern.pattern.get(0), true);
        assert_eq!(pattern.pattern.get(1), true);
        assert_eq!(pattern.pattern.get(2), true);
        assert_eq!(pattern.pattern.get(3), false);
    }

    #[test]
    fn test_sim() {
        let mut field = Field::new(TEST);
        field.add_rock();
        assert_eq!(field.data.get(0), Some(&0b0011110));
        field.add_rock();
        assert_eq!(field.data.get(0), Some(&0b0001000));
        assert_eq!(field.data.get(1), Some(&0b0011100));
        assert_eq!(field.data.get(2), Some(&0b0001000));
        field.add_rock();
        assert_eq!(field.data.get(0), Some(&0b0010000));
        assert_eq!(field.data.get(1), Some(&0b0010000));
        assert_eq!(field.data.get(2), Some(&0b1111000));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 3068);
    }
}
