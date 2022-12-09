use std::collections::HashSet;

const PART_1: bool = true;

static PART_1_DATA: &str = include_str!("input");

fn main() {
    if PART_1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut pos = HashSet::with_capacity(1024);
    pos.insert(Coord::default());

    let mut knot = Knot::default();

    for line in PART_1_DATA.lines() {
        knot.do_moves(line, &mut pos);
    }

    println!("{}", pos.len());
}

fn part2() {}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_head(&mut self, dir: Dir) {
        use Dir::*;

        match dir {
            Up => self.y += 1,
            Right => self.x += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
        }
    }

    fn adjust_tail(&mut self, head: Coord) {
        let x_diff = (self.x - head.x).abs();
        let y_diff = (self.y - head.y).abs();

        if x_diff > 1 {
            if self.x < head.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }

            if y_diff > 0 {
                if self.y < head.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
        } else if y_diff > 1 {
            if self.y < head.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }

            if x_diff > 0 {
                if self.x < head.x {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            }
        }
    }
}

const TAIL_N: usize = 9;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
struct Knot {
    head: Coord,
    tails: [Coord; TAIL_N],
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        use Dir::*;

        match c {
            'u' | 'U' => Up,
            'r' | 'R' => Right,
            'd' | 'D' => Down,
            'l' | 'L' => Left,
            _ => panic!("Unknown direction: {c}"),
        }
    }
}

struct Move {
    dir: Dir,
    count: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        Move {
            dir: s.chars().next().unwrap().try_into().unwrap(),
            count: s[2..].parse().unwrap(),
        }
    }
}

impl Knot {
    fn do_moves(&mut self, entry: &str, tail_pos: &mut HashSet<Coord>) {
        let m = Move::from(entry);

        for _ in 0..(m.count) {
            self.head.move_head(m.dir);
            self.tails[0].adjust_tail(self.head);
            for i in 1..TAIL_N {
                self.tails[i].adjust_tail(self.tails[i - 1]);
            }
            tail_pos.insert(self.tails[TAIL_N - 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {}
}
