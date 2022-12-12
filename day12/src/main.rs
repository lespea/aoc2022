#![allow(unused)]

use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeIndexable;
use petgraph::{Directed, Graph, Undirected};
use std::io::Read;

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
    let (g, start, end) = build_graph(PART_1_DATA);
    // println!("{g:?}");
    let steps = astar(&g, start, |e| e == end, |_| 1, |_| 0);

    let ans = steps.map(|a| a.0);

    println!("{ans:?}");
}

fn part2() {}

#[inline]
fn want(b1: u8, b2: u8) -> bool {
    b2 <= b1 || (b1 < b2 && b1 == b2 - 1) || (b1 > b2 && b1 == b2 + 1)
}

pub fn build_graph(data: &str) -> (Graph<(usize, usize), u8, Directed>, NodeIndex, NodeIndex) {
    let (mut start, mut end) = (None, None);

    let mut g = Graph::new();

    data.lines()
        .enumerate()
        .fold(None::<Vec<(NodeIndex, u8)>>, |upper, (idx_row, cur)| {
            let mut cur_nodes = Vec::with_capacity(upper.as_ref().map(|v| v.len()).unwrap_or(256));

            cur.bytes()
                .enumerate()
                .fold(None::<(NodeIndex, u8)>, |left, (idx_col, node_b)| {
                    let loc = (idx_row, idx_col);

                    let cur_node = g.add_node(loc);

                    let height = match node_b {
                        b'S' => {
                            assert!(start.replace(cur_node).is_none());
                            0
                        }

                        b'E' => {
                            assert!(end.replace(cur_node).is_none());
                            b'z' - b'a'
                        }

                        b'a'..=b'z' => node_b - b'a',

                        _ => unreachable!(),
                    };

                    if let Some((n, prev_height)) = left {
                        if want(height, prev_height) {
                            g.add_edge(cur_node, n, 1);
                        }
                        if want(prev_height, height) {
                            g.add_edge(n, cur_node, 1);
                        }
                    }

                    if let Some((n, prev_height)) = upper.as_ref().and_then(|v| v.get(idx_col)) {
                        if want(height, *prev_height) {
                            g.add_edge(cur_node, *n, 1);
                        }
                        if want(*prev_height, height) {
                            g.add_edge(*n, cur_node, 1);
                        }
                    }

                    let cur_info = (cur_node, height);

                    cur_nodes.push(cur_info);
                    Some(cur_info)
                });

            Some(cur_nodes)
        });

    (g, start.unwrap(), end.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::astar;

    #[test]
    fn example() {
        let (g, start, end) = build_graph(example_data());
        // println!("{g:?}");
        let steps = astar(&g, start, |e| e == end, |_| 1, |_| 0);
        assert_eq!(31, steps.unwrap().0);
    }

    fn example_data() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }
}
