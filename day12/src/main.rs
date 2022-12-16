#![allow(unused)]

use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeIndexable;
use petgraph::{Directed, Graph, Undirected};
use std::io::Read;

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
    let (g, start, end, _) = build_graph(PART_1_DATA);
    let ans = find_fastest(&g, start, end);
    println!("{ans:?}");
}

fn part2() {
    let (g, _, end, starts) = build_graph(PART_1_DATA);
    let ans = find_fastest_groups(&g, starts, end);
    println!("{ans:?}");
}

#[inline]
fn want(b1: u8, b2: u8) -> bool {
    b2 <= b1 || b1 + 1 == b2
}

type HillGraph = Graph<(usize, usize), u8, Directed>;

pub fn build_graph(data: &str) -> (HillGraph, NodeIndex, NodeIndex, Vec<NodeIndex>) {
    let (mut start, mut end) = (None, None);
    let mut starts = Vec::with_capacity(256);

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

                    if height == 0 {
                        starts.push(cur_node);
                    }

                    cur_nodes.push(cur_info);
                    Some(cur_info)
                });

            Some(cur_nodes)
        });

    (g, start.unwrap(), end.unwrap(), starts)
}

pub fn find_fastest(g: &HillGraph, start: NodeIndex, end: NodeIndex) -> Option<usize> {
    astar(&g, start, |e| e == end, |_| 1, |_| 0).map(|a| a.0)
}

pub fn find_fastest_groups(g: &HillGraph, start: Vec<NodeIndex>, end: NodeIndex) -> Option<usize> {
    start.iter().flat_map(|s| find_fastest(g, *s, end)).min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::astar;

    #[test]
    fn example() {
        let (g, start, end, starts) = build_graph(example_data());
        assert_eq!(31, find_fastest(&g, start, end).unwrap());
        assert_eq!(29, find_fastest_groups(&g, starts, end).unwrap());
    }

    fn example_data() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }
}
