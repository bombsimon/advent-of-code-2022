use crate::input;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let x = input::file_for_day(7);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Debug, Clone)]
struct Dir {
    file_sizes: Vec<i64>,
    subdirs: HashSet<String>,
}

impl Dir {
    fn new() -> Self {
        Self {
            file_sizes: Vec::new(),
            subdirs: HashSet::new(),
        }
    }

    fn size(&self, tree: &HashMap<String, Dir>) -> i64 {
        let mut size = self.file_sizes.iter().sum();
        for childen in &self.subdirs {
            let subdir = tree.get(childen).unwrap();
            size += subdir.size(tree);
        }

        size
    }
}

fn part_one(input: Vec<String>) -> i64 {
    let tree = get_tree(input);

    tree.values().fold(0, |acc, v| {
        let size = v.size(&tree);
        if size < 100000 {
            acc + size
        } else {
            acc
        }
    })
}

fn part_two(input: Vec<String>) -> i64 {
    let tree = get_tree(input);
    let used = tree.get("/").unwrap().size(&tree);
    let free = 70000000 - used;

    tree.values()
        .filter_map(|v| {
            let size = v.size(&tree);
            if free + size >= 30000000 {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn get_tree(input: Vec<String>) -> HashMap<String, Dir> {
    let mut cwd = Dir::new();
    let mut tree: HashMap<String, Dir> = HashMap::new();
    let mut path = vec!["/".to_string()];

    for line in input {
        let (lhs, rhs) = line.split_once(' ').unwrap();

        match lhs {
            "$" if rhs == "ls" => (),
            "$" if rhs.starts_with("cd") => {
                let (_, dir) = rhs.split_once(' ').unwrap();

                match dir {
                    "/" => (),
                    ".." => {
                        tree.insert(path.join("/"), cwd.clone());
                        let _ = path.pop();
                        cwd = tree.get(&path.join("/")).unwrap().to_owned();
                    }
                    _ => {
                        tree.insert(path.join("/"), cwd.clone());
                        path.push(dir.to_owned());
                        cwd = Dir::new();
                    }
                }
            }
            "dir" => {
                let mut p = path.clone();
                p.push(rhs.to_owned());
                let _ = cwd.subdirs.insert(p.join("/"));
            }
            filesize => cwd.file_sizes.push(filesize.parse::<i64>().unwrap()),
        }
    }

    tree.insert(path.join("/"), cwd.clone());

    tree
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 95437;
    static SOLUTION_TWO: i64 = 24933642;
    static TEST_INPUT: &str = r#"
$ cd /
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
7214296 k"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
