const PART_1: bool = false;

static PART_1_DATA: &str = include_str!("input");

fn main() {
    if PART_1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut comp = Comp::default();
    comp.run_all(PART_1_DATA);
    println!("{}", comp.signal_total());
}

fn part2() {
    let mut comp = Comp::default();
    comp.run_all(PART_1_DATA);
    println!("{}", comp.output_sprites);
}

const COLS: usize = 40;
const ROWS: usize = 6;

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub struct Comp {
    pub cycle: i64,
    pub x: i64,
    pub signal: i64,
    pub sig_cycle: i64,
    pub found_signals: Vec<i64>,
    pub output_sprites: String,
}

impl Default for Comp {
    fn default() -> Self {
        Comp {
            cycle: 0,
            x: 1,
            signal: 0,
            sig_cycle: 20,
            found_signals: Vec::with_capacity(16),
            output_sprites: String::with_capacity(ROWS * COLS + COLS),
        }
    }
}

impl Comp {
    pub fn signal_total(&self) -> i64 {
        self.found_signals.iter().sum()
    }

    pub fn run_all(&mut self, insts: &str) {
        for inst in insts.lines() {
            self.run(inst.into());
        }
    }

    pub fn run(&mut self, inst: Inst) {
        let adj = match inst {
            Inst::Noop => {
                self.cycle();
                0
            }

            Inst::Add(n) => {
                self.cycle();
                self.cycle();
                n
            }
        };

        if self.cycle >= self.sig_cycle {
            let val = self.sig_cycle * self.x;
            // println!("Found signal {}*{} = {}", self.sig_cycle, self.x, val);
            self.found_signals.push(val);
            self.sig_cycle += 40;
        }

        self.x += adj;
    }

    fn cycle(&mut self) {
        const ON: char = '#';
        const OFF: char = '.';

        let pos = self.cycle % 40;

        if pos >= self.x - 1 && pos <= self.x + 1 {
            self.output_sprites.push(ON);
        } else {
            self.output_sprites.push(OFF);
        }

        self.cycle += 1;

        if pos == (COLS as i64) - 1 {
            self.output_sprites.push('\n');
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub enum Inst {
    Noop,
    Add(i64),
}

impl From<&str> for Inst {
    fn from(str: &str) -> Self {
        if str == "noop" {
            Inst::Noop
        } else if let Some(num) = str.strip_prefix("addx ") {
            Inst::Add(num.parse().unwrap())
        } else {
            panic!("Invalid instruction: {str}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut comp = Comp::default();
        comp.run_all(test_data());
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], comp.found_signals);
        assert_eq!(13140, comp.signal_total());

        assert_eq!(example_output().to_string(), comp.output_sprites)
    }

    fn test_data() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    fn example_output() -> &'static str {
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
    }
}
