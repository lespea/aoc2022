use id_tree::{InsertBehavior, Node, NodeId, Tree};
use std::fmt::{Display, Formatter};

const PART_1: bool = false;

static PART_1_DATA: &str = include_str!("input");

fn main() {
    let fs = FS::new(PART_1_DATA);

    if false {
        fs.print();
    }

    if PART_1 {
        part1(fs);
    } else {
        part2(fs);
    }
}

fn part1(fs: FS) {
    println!("{}", fs.sum_dirs_under(100_000));
}

fn part2(fs: FS) {
    println!("{}", fs.find_del_dir(70_000_000, 30_000_000));
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
enum Entry {
    Dir(FsEntry),
    File(FsEntry),
}

impl Entry {
    fn size(&self) -> i64 {
        match self {
            Entry::Dir(d) => d.size,
            Entry::File(d) => d.size,
        }
    }

    fn dir_size(&self) -> Option<i64> {
        match self {
            Entry::Dir(d) => Some(d.size),
            _ => None,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Dir(d) => write!(f, "Dir {} ({})", d.name, d.size),
            Entry::File(d) => write!(f, "{} ({})", d.name, d.size),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Default)]
struct FsEntry {
    name: String,
    size: i64,
}

#[derive(PartialEq, Clone, Debug)]
struct FS {
    tree: Tree<Entry>,
}

impl FS {
    pub fn new(lines: &str) -> Self {
        let mut tree = Self { tree: Tree::new() };

        tree.tree
            .insert(
                Node::new(Entry::Dir(FsEntry {
                    name: "/".to_string(),
                    size: 0,
                })),
                InsertBehavior::AsRoot,
            )
            .expect("Couldn't insert root node");

        tree.parse_cmds(lines);

        let root_id = tree.tree.root_node_id().unwrap().clone();
        FS::compute_dir_sizes(&mut tree.tree, root_id);

        tree
    }

    fn parse_cmds(&mut self, lines: &str) {
        let tree = &mut self.tree;

        let root_id = tree.root_node_id().unwrap().clone();
        let mut cur_id = root_id.clone();

        for line in lines.lines() {
            if let Some(cmd) = line.strip_prefix("$ ") {
                if let Some(dir) = cmd.strip_prefix("cd ") {
                    cur_id = match dir {
                        "/" => root_id.clone(),
                        ".." => tree.get(&cur_id).unwrap().parent().unwrap().clone(),
                        _ => FS::subdir(tree, &cur_id, dir),
                    };
                } else {
                    match cmd {
                        "ls" => (),
                        _ => panic!("Unknown command: {cmd}"),
                    }
                }
            } else if let Some(dir) = line.strip_prefix("dir ") {
                // Add empty subdir
                FS::subdir(tree, &cur_id, dir);
            } else {
                let (num, idx) = atoi::FromRadix10::from_radix_10(line.as_bytes());
                if idx == 0 {
                    panic!("Unknown entry: {line}")
                }

                let node = Node::new(Entry::File(FsEntry {
                    name: line[idx + 1..].to_string(),
                    size: num,
                }));

                tree.insert(node, InsertBehavior::UnderNode(&cur_id))
                    .unwrap();
            }
        }
    }

    fn subdir(tree: &mut Tree<Entry>, cur_id: &NodeId, name: &str) -> NodeId {
        let node_id = tree
            .get(cur_id)
            .unwrap()
            .children()
            .iter()
            .find(|&id| match tree.get(id).unwrap().data() {
                Entry::Dir(fe) if fe.name == name => true,

                Entry::File(fe) if fe.name == name => {
                    panic!("Tried CDing to the file {name}")
                }
                _ => false,
            })
            .cloned();

        node_id.unwrap_or_else(|| {
            let node = Node::new(Entry::Dir(FsEntry {
                name: name.to_string(),
                size: 0,
            }));

            tree.insert(node, InsertBehavior::UnderNode(cur_id))
                .unwrap()
        })
    }

    fn compute_dir_sizes(tree: &mut Tree<Entry>, entry: NodeId) {
        #[allow(clippy::needless_collect)]
        let to_check: Vec<NodeId> = tree
            .children_ids(&entry)
            .unwrap()
            .filter(|&id| matches!(tree.get(id).unwrap().data(), Entry::Dir(_)))
            .cloned()
            .collect();

        for id in to_check.into_iter() {
            FS::compute_dir_sizes(tree, id)
        }

        let mut size = 0;
        for child in tree.children(&entry).unwrap() {
            size += child.data().size();
        }

        match tree.get_mut(&entry).unwrap().data_mut() {
            Entry::Dir(e) => e.size = size,
            _ => panic!("Setting size of file entry?"),
        }
    }

    fn sum_dirs_under(&self, max_size: i64) -> i64 {
        let mut sum = 0;

        for entry in self
            .tree
            .traverse_level_order(self.tree.root_node_id().unwrap())
            .unwrap()
        {
            match entry.data() {
                Entry::Dir(e) if e.size <= max_size => sum += e.size,
                _ => (),
            }
        }

        sum
    }

    fn find_del_dir(&self, total_space: i64, needed_space: i64) -> i64 {
        let root_id = self.tree.root_node_id().unwrap();
        let root_size = self.tree.get(root_id).unwrap().data().dir_size().unwrap();

        let free_space = total_space - root_size;
        if free_space > needed_space {
            return 0;
        }

        let to_delete = needed_space - free_space;
        if root_size < to_delete {
            panic!("Insufficient space");
        }

        let mut del_size = root_size;
        for node in self.tree.traverse_level_order(root_id).unwrap() {
            if let Some(size) = node.data().dir_size() {
                if size >= to_delete {
                    del_size = del_size.min(size);
                }
            }
        }

        del_size
    }

    fn print(&self) {
        let root_id = self.tree.root_node_id().unwrap();
        FS::print_lvl(&self.tree, root_id, 0)
    }

    fn print_lvl(tree: &Tree<Entry>, id: &NodeId, level: usize) {
        let node = tree.get(id).unwrap().data();
        println!("{}{}", " ".repeat(level * 2), node);

        for child in tree.children_ids(id).unwrap() {
            FS::print_lvl(tree, child, level + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
    }

    #[test]
    fn examples_1() {
        let fs = FS::new(test_data());
        fs.print();
        assert_eq!(95_437, fs.sum_dirs_under(100_000))
    }

    #[test]
    fn examples_2() {
        let fs = FS::new(test_data());
        assert_eq!(24_933_642, fs.find_del_dir(70_000_000, 30_000_000))
    }
}
