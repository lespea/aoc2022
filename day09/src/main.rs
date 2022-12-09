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

    let mut knot = Knot::default();
    pos.insert(knot.tail);

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

        if x_diff + y_diff > 1 {
            match (x_diff, y_diff) {
                (1, 1) => (),

                (2, _) => {
                    self.y = head.y;
                    if self.x < head.x {
                        self.x += 1;
                    } else {
                        self.x -= 1;
                    }
                }

                (_, 2) => {
                    self.x = head.x;
                    if self.y < head.y {
                        self.y += 1;
                    } else {
                        self.y -= 1;
                    }
                }

                _ => panic!("Invalid position: {self:?}/{head:?}"),
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
struct Knot {
    head: Coord,
    tail: Coord,
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
            self.tail.adjust_tail(self.head);
            tail_pos.insert(self.tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {}
}
