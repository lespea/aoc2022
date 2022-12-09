use ndarray::{Array, ArrayBase, OwnedRepr};

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
    println!("{}", Forest::new(PART_1_DATA).count_vis());
}

fn part2() {
    println!("{}", Forest::new(PART_1_DATA).scenic_score());
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Tree {
    height: u8,
    visible: bool,
    scenic_view: usize,
}

struct Forest {
    trees: ArrayBase<OwnedRepr<Tree>, ndarray::Dim<[usize; 2]>>,
}

impl Forest {
    pub fn new(data: &str) -> Self {
        let size = data.lines().next().unwrap().len();

        let trees: Vec<Tree> = data
            .lines()
            .flat_map(|line| {
                line.bytes().map(|b| Tree {
                    height: b - b'0',
                    visible: false,
                    scenic_view: 0,
                })
            })
            .collect();

        let mut forest = Forest {
            trees: Array::from_shape_vec((size, size), trees).unwrap(),
        };

        forest.calc_vis();
        forest.update_scenic_score();

        forest
    }

    // fn reset_vis(&mut self) {
    //     for tree in self.trees.iter_mut() {
    //         tree.visible = false;
    //     }
    // }

    fn calc_vis(&mut self) {
        for mut row in self.trees.rows_mut() {
            let mut max = None;
            for tree in row.iter_mut() {
                Forest::adjust(&mut max, tree);
            }

            max = None;
            for tree in row.iter_mut().rev() {
                Forest::adjust(&mut max, tree);
            }
        }

        for mut col in self.trees.columns_mut() {
            let mut max = None;
            for tree in col.iter_mut() {
                Forest::adjust(&mut max, tree);
            }

            max = None;
            for tree in col.iter_mut().rev() {
                Forest::adjust(&mut max, tree);
            }
        }
    }

    fn adjust(max: &mut Option<u8>, tree: &mut Tree) {
        match max {
            Some(h) => {
                if tree.height > *h {
                    tree.visible = true;
                    max.replace(tree.height);
                }
            }

            None => {
                tree.visible = true;
                max.replace(tree.height);
            }
        }
    }

    pub fn update_scenic_score(&mut self) {
        let t = &mut self.trees;
        let n = t.nrows();

        for col in 0..n {
            for row in 0..n {
                let height = t.get((row, col)).unwrap().height;

                let mut left = 0;
                let mut up = 0;
                let mut right = 0;
                let mut down = 0;

                for c in (0..col).rev() {
                    left += 1;
                    if t.get((row, c)).unwrap().height >= height {
                        break;
                    }
                }
                for c in (col + 1)..n {
                    right += 1;
                    if t.get((row, c)).unwrap().height >= height {
                        break;
                    }
                }

                for r in (0..row).rev() {
                    up += 1;
                    if t.get((r, col)).unwrap().height >= height {
                        break;
                    }
                }
                for r in (row + 1)..n {
                    down += 1;
                    if t.get((r, col)).unwrap().height >= height {
                        break;
                    }
                }

                t.get_mut((row, col)).unwrap().scenic_view = left * up * right * down;
            }
        }
    }

    pub fn count_vis(&self) -> usize {
        self.trees.iter().filter(|t| t.visible).count()
    }

    pub fn scenic_score(&self) -> usize {
        self.trees.iter().map(|t| t.scenic_view).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Write;

    fn test_data() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn examples() {
        let f = Forest::new(test_data());

        let mut h = String::with_capacity(10);
        let mut s = String::with_capacity(10);

        for row in f.trees.rows() {
            for t in row.iter() {
                write!(&mut h, "{}", t.height).unwrap();
                write!(&mut s, "{}", t.scenic_view).unwrap();
            }
            h.write_char('\n').unwrap();
            s.write_char('\n').unwrap();
        }

        println!("{}\n\n{}", h, s);

        assert_eq!(21, f.count_vis());
        assert_eq!(8, f.scenic_score())
    }
}
