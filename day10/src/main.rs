#[derive(Debug, PartialEq, Clone)]
enum Inst {
    Noop,
    Addx(i32),
}

impl Inst {
    fn cycles(&self) -> i32 {
        match self {
            Inst::Noop => 1,
            Inst::Addx(_) => 2,
        }
    }
}

struct State<T: Iterator<Item = Inst>> {
    x: i32,
    cycles: i32,
    program: T,
    next: Option<Inst>,
    curr_cycle: i32,
    crt: Vec<char>,
}

impl<T: Iterator<Item = Inst>> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "State {{ x: {}, cycles: {}, next: {:?}, curr_cycle: {} }}", self.x, self.cycles, self.next, self.curr_cycle)
    }
}

impl<T: Iterator<Item = Inst>> State<T> {
    fn new(program: T) -> Self {
        Self { x: 1, cycles: 0, program, next: None, curr_cycle: 0, crt: vec![] }
    }

    fn start(&mut self) {
        if let None = self.next {
            if let Some(inst) = self.program.next() {
                self.cycles = inst.cycles()-1;
                self.next = Some(inst);
            }
        }
        self.curr_cycle += 1;
    }

    fn draw(&mut self) {
        let pos = (self.curr_cycle - 1) % 40;
        if pos >= self.x-1 && pos <= self.x+1 {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }
    }

    fn end(&mut self) {
        if self.cycles == 0 {
            // Time to execute the next instruction
            let inst = self.next.take().unwrap();
            match inst {
                Inst::Noop => {},
                Inst::Addx(arg) => self.x += arg,
            }
        } else {
            self.cycles -= 1;
        }
    }

    fn step(&mut self) -> bool {
        self.start();
        if self.next.is_none() {
            return false;
        }
        self.draw();
        self.end();
        true
    }

    fn step_to(&mut self, cycle: i32) -> bool {
        while self.curr_cycle <= cycle-1 {
            if !self.step() {
                panic!("ran out of instructions before reaching cycle {}", cycle);
            }
        }
        return true;
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

impl<T: Iterator<Item = Inst>> ToString for State<T> {
    fn to_string(&self) -> String {
        // Lines are 40 characters
        self.crt.chunks(40).map(|line| {
            line.iter().collect::<String>()
        }).collect::<Vec<String>>().join("\n")
    }
}

fn parse(input: &'static str) -> impl Iterator<Item = Inst> {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let inst = parts.next().unwrap();
        match inst {
            "noop" => Inst::Noop,
            "addx" => Inst::Addx(parts.next().unwrap().parse().unwrap()),
            _ => panic!("unknown instruction"),
        }
    })
}

fn part1<T: Iterator<Item = Inst>>(program: T) -> i32 {
    let mut part1 = State::new(program);
    [20, 60, 100, 140, 180, 220].map(|cycle| {
        part1.step_to(cycle);
        part1.x * cycle
    }).iter().sum()
}

fn part2<T: Iterator<Item = Inst>>(program: T) -> String {
    let mut part2 = State::new(program);
    part2.run();
    part2.to_string()
}

fn main() {
    println!("Part 1: {}", part1(parse(include_str!("input.txt"))));
    println!("Part 2:\n{}", part2(parse(include_str!("input.txt"))));
}

#[cfg(test)]
mod tests {
    const TEST1: &'static str = "noop\n\
    addx 3\n\
    addx -5";
    const TEST2: &'static str = include_str!("test.txt");

    const TEST3: &'static str = "##..##..##..##..##..##..##..##..##..##..\n\
    ###...###...###...###...###...###...###.\n\
    ####....####....####....####....####....\n\
    #####.....#####.....#####.....#####.....\n\
    ######......######......######......####\n\
    #######.......#######.......#######.....";

    #[test]
    fn test1() {
        let mut state = super::State::new(super::parse(TEST1));
        state.step();
        // Cycle 1: noop
        assert_eq!(state.x, 1);
        assert!(state.step());
        // Cycle 2: noop done, addx 3
        assert_eq!(state.x, 1);
        assert!(state.step());
        // Cycle 3: addx 3 on cycle 2
        assert_eq!(state.x, 4);
        assert!(state.step());
        // Cycle 4: addx 3 done, addx -5
        assert_eq!(state.x, 4);
        assert!(state.step());
        // Cycle 5: addx -5 on cycle 2
        assert_eq!(state.x, -1);
        state.step();
        // Cycle 6: addx -5 done
        assert_eq!(state.x, -1);
    }

    #[test]
    fn test2() {
        let mut state = super::State::new(super::parse(TEST2));
        state.step_to(20);
        assert_eq!(state.x, 21);
        state.step_to(60);
        assert_eq!(state.x, 19);
        state.step_to(100);
        assert_eq!(state.x, 18);
        state.step_to(140);
        assert_eq!(state.x, 21);
        state.step_to(180);
        assert_eq!(state.x, 16);
        state.step_to(220);
        assert_eq!(state.x, 18);
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(super::parse(TEST2)), 13_140);
    }

    #[test]
    fn test_part2() {
        let mut state = super::State::new(super::parse(TEST2));
        state.run();
        assert_eq!(state.to_string(), TEST3);
    }

}
