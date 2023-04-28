use lazy_regex::regex;
use std::collections::{VecDeque, HashSet};


#[derive(Clone)]
struct Blueprint {
    /// ore
    ore_cost: u16,
    /// ore
    clay_cost: u16,
    /// (ore, clay)
    obsidian_cost: (u16, u16),
    /// (ore, obsidian)
    geode_cost: (u16, u16),
    /// Most ore bots we need to build our most expensive bot in 1 turn
    max_ore_bots: u16,
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 9 obsidian.
impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        let re = regex!("Each (?P<item>\\w+) robot costs (?P<cost>\\d+) ore( and (?P<cost2>\\d+) clay)?( and (?P<cost3>\\d+) obsidian)?");
        let mut ore_cost = 0;
        let mut clay_cost = 0;
        let mut obsidian_cost = (0, 0);
        let mut geode_cost = (0, 0);
        for line in s.split(". ").filter(|s| !s.is_empty()) {
            let caps = re.captures(line).unwrap();
            let item = caps.name("item").unwrap().as_str();
            let cost = caps.name("cost").unwrap().as_str().parse().unwrap();
            match item {
                "ore" => ore_cost = cost,
                "clay" => clay_cost = cost,
                "obsidian" => {
                    let cost2 = caps.name("cost2").unwrap().as_str().parse().unwrap();
                    obsidian_cost = (cost, cost2);
                }
                "geode" => {
                    let cost3 = caps.name("cost3").unwrap().as_str().parse().unwrap();
                    geode_cost = (cost, cost3);
                }
                _ => panic!("unknown item: {}", item),
            }
        }
        Self {
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
            max_ore_bots: ore_cost.max(clay_cost).max(obsidian_cost.0).max(geode_cost.0),
        }
    }
}

#[derive(Clone)]
struct State {
    blueprint: Blueprint,
    ore: u16,
    ore_bots: u16,
    clay: u16,
    clay_bots: u16,
    obsidian: u16,
    obsidian_bots: u16,
    geodes: u16,
    geode_bots: u16,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("ore", &self.ore)
            .field("ore_bots", &self.ore_bots)
            .field("clay", &self.clay)
            .field("clay_bots", &self.clay_bots)
            .field("obsidian", &self.obsidian)
            .field("obsidian_bots", &self.obsidian_bots)
            .field("geodes", &self.geodes)
            .field("geode_bots", &self.geode_bots)
            .finish()
    }
}

impl State {
    fn new(blueprint: &Blueprint) -> Self {
        Self {
            blueprint: blueprint.clone(),
            ore: 0,
            ore_bots: 1,
            clay: 0,
            clay_bots: 0,
            obsidian: 0,
            obsidian_bots: 0,
            geodes: 0,
            geode_bots: 0,
        }
    }

    fn tick(&mut self, n: u16) {
        // ore
        if self.ore_bots > 0 {
            self.ore += self.ore_bots * n;
        }
        // clay
        if self.clay_bots > 0 {
            self.clay += self.clay_bots * n;
        }
        // obsidian
        if self.obsidian_bots > 0 {
            self.obsidian += self.obsidian_bots * n;
        }
        // geodes
        if self.geode_bots > 0 {
            self.geodes += self.geode_bots * n;
        }
    }

    // Time needed needs to round up
    fn time_to_build_ore(&self) -> u16 {
        let ore_cost = self.blueprint.ore_cost;
        let ore_bots = self.ore_bots;
        let ore = self.ore;
        if ore >= ore_cost {
            return 1;
        }
        let ore_needed = ore_cost - ore;
        ore_needed / ore_bots + 1 + (ore_needed % ore_bots != 0) as u16
    }

    fn time_to_build_clay(&self) -> u16 {
        let clay_cost = self.blueprint.clay_cost;
        let ore_bots = self.ore_bots;
        let ore = self.ore;
        if ore >= clay_cost {
            return 1;
        }
        let ore_needed = clay_cost - ore;
        ore_needed / ore_bots + 1 + (ore_needed % ore_bots != 0) as u16
    }

    fn time_to_build_obsidian(&self) -> Option<u16> {
        let (ore_cost, clay_cost) = self.blueprint.obsidian_cost;
        let ore_bots = self.ore_bots;
        let clay_bots = self.clay_bots;
        let ore = self.ore;
        let clay = self.clay;
        if ore_bots == 0 || clay_bots == 0 {
            return None;
        }
        let ore_time = if ore >= ore_cost {
            0
        } else {
            let ore_needed = ore_cost - ore;
            ore_needed / ore_bots + (ore_needed % ore_bots != 0) as u16
        };
        let clay_time = if clay >= clay_cost {
            0
        } else {
            let clay_needed = clay_cost - clay;
            clay_needed / clay_bots + (clay_needed % clay_bots != 0) as u16
        };
        Some(ore_time.max(clay_time)+1)
    }

    fn time_to_build_geode(&self) -> Option<u16> {
        let (ore_cost, obsidian_cost) = self.blueprint.geode_cost;
        let ore_bots = self.ore_bots;
        let obsidian_bots = self.obsidian_bots;
        let ore = self.ore;
        let obsidian = self.obsidian;
        if ore_bots == 0 || obsidian_bots == 0 {
            return None;
        }
        let ore_time = if ore >= ore_cost {
            0
        } else {
            let ore_needed = ore_cost - ore;
            ore_needed / ore_bots + (ore_needed % ore_bots != 0) as u16
        };
        let obsidian_time = if obsidian >= obsidian_cost {
            0
        } else {
            let obsidian_needed = obsidian_cost - obsidian;
            obsidian_needed / obsidian_bots + (obsidian_needed % obsidian_bots != 0) as u16
        };
        Some(ore_time.max(obsidian_time)+1)
    }

    fn build_ore(&mut self) {
        let time = self.time_to_build_ore();
        self.tick(time);
        self.ore -= self.blueprint.ore_cost;
        self.ore_bots += 1;
    }

    fn build_clay(&mut self) {
        let time = self.time_to_build_clay();
        self.tick(time);
        self.ore -= self.blueprint.clay_cost;
        self.clay_bots += 1;
    }

    fn build_obsidian(&mut self) {
        let time = self.time_to_build_obsidian().unwrap();
        self.tick(time);
        let (ore_cost, clay_cost) = self.blueprint.obsidian_cost;
        self.ore -= ore_cost;
        self.clay -= clay_cost;
        self.obsidian_bots += 1;
    }

    fn build_geode(&mut self) {
        let time = self.time_to_build_geode().unwrap();
        self.tick(time);
        let (ore_cost, obsidian_cost) = self.blueprint.geode_cost;
        self.ore -= ore_cost;
        self.obsidian -= obsidian_cost;
        self.geode_bots += 1;
    }

    // Return a list of decisions that can be made from this state with the time left and their time cost
    // We'll skip any decisions that would build more bots than resource we need to build them in 1 turn
    // Optimization #0: Don't produce more of a resource than we can consume in 1 turn
    // Optimization #2: don't build bots of a lesser resource once we have acquired the higher resource
    fn permutations(&self, time: u16) -> Option<Vec<(u16, State)>> {
        let mut permutations = Vec::new();
        let ore_time = self.time_to_build_ore();
        if ore_time < time && self.ore_bots < self.blueprint.max_ore_bots {
            let mut state = self.clone();
            state.build_ore();
            permutations.push((ore_time, state));
        }
        let clay_time = self.time_to_build_clay();
        if clay_time < time && self.clay_bots < self.blueprint.obsidian_cost.1 {
            let mut state = self.clone();
            state.build_clay();
            permutations.push((clay_time, state));
        }
        if let Some(obsidian_time) = self.time_to_build_obsidian() {
            if obsidian_time < time && self.obsidian_bots < self.blueprint.geode_cost.1 {
                let mut state = self.clone();
                state.build_obsidian();
                permutations.push((obsidian_time, state));
            }
        }
        if let Some(geode_time) = self.time_to_build_geode() {
            if geode_time < time {
                let mut state = self.clone();
                state.build_geode();
                permutations.push((geode_time, state));
            }
        }
        if permutations.is_empty() {
            None
        } else {
            Some(permutations)
        }
    }

    fn hash_key(&self, time: u16) -> u128 {
        let mut hash = time as u128;
        hash |= (self.ore as u128) << 8;
        hash |= (self.clay as u128) << 16;
        hash |= (self.obsidian as u128) << 24;
        hash |= (self.geodes as u128) << 32;
        hash |= (self.ore_bots as u128) << 40;
        hash |= (self.clay_bots as u128) << 48;
        hash |= (self.obsidian_bots as u128) << 56;
        hash |= (self.geode_bots as u128) << 64;
        hash
    }
}

// Run a breadth first search to find the most geodes this blueprint can make in the alloted time
// Optimization #1: Don't explore states we've already seen
fn blueprint_best(bp: &Blueprint, time: u16) -> u16 {
    let mut queue = VecDeque::new();
    let state = State::new(bp);
    let mut visited = HashSet::new();
    let mut best = 0;

    queue.push_back((state, time));
    while let Some((state, remaining)) = queue.pop_front() {
        // If we've already seen this state, skip it
        if !visited.insert(state.hash_key(remaining)) {
            continue;
        }
        // Also also in theory if we have an absurd amount of a single resource
        // We're probably going the wrong way
        if state.ore > 100 || state.clay > 100 || state.obsidian > 100 {
            continue;
        }
        // In theory once we have obsidian
        if let Some(permutations) = state.permutations(remaining) {
            for (cost, new) in permutations {
                queue.push_back((new, remaining - cost));
            }
        }
        // No more permutations, mine for the rest of the time
        let mut state = state.clone();
        state.tick(remaining);
        best = best.max(state.geodes);
    }
    best
}

fn part1(input: &str) -> u64 {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Blueprint::from(s))
        .enumerate()
        .map(|(i, bp)| blueprint_best(&bp, 24) as u64 * (i as u64 + 1))
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Blueprint::from(s))
        .take(3)
        .map(|bp| blueprint_best(&bp, 32) as u64)
        .product()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = "Blueprint 1: \
    Each ore robot costs 4 ore. \
    Each clay robot costs 2 ore. \
    Each obsidian robot costs 3 ore and 14 clay. \
    Each geode robot costs 2 ore and 7 obsidian.\n\
  Blueprint 2: \
    Each ore robot costs 2 ore. \
    Each clay robot costs 3 ore. \
    Each obsidian robot costs 3 ore and 8 clay. \
    Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_parse() {
        let blueprint = Blueprint::from("Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.");
        assert_eq!(blueprint.ore_cost, 4);
        assert_eq!(blueprint.clay_cost, 2);
        assert_eq!(blueprint.obsidian_cost, (3, 14));
        assert_eq!(blueprint.geode_cost, (2, 7));
    }

    #[test]
    fn test_permutations() {
        let blueprint = Blueprint::from("Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.");
        let state = State::new(&blueprint);
        let permutations = state.permutations(24).unwrap();
        println!("{:?}", permutations);
        assert_eq!(permutations.len(), 2);
    }

    #[test]
    fn test_best() {
        let blueprint = Blueprint::from("Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.");
        assert_eq!(blueprint_best(&blueprint, 24), 9);
    }

    #[test]
    fn test_best2() {
        let blueprint = Blueprint::from("Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.");
        assert_eq!(blueprint_best(&blueprint, 32), 62);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 3472);
    }
}