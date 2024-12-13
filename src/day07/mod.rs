use std::{fs, usize};

#[derive(Debug)]
struct Operation {
    expected: u64,
    numbers: Vec<u64>,
}

impl Operation {
    fn can_be_true(&self) -> bool {
        let max_level = self.numbers.len();
        let data = self
            .numbers
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, number)| {
                for _ in 0..(2_usize.pow(i.try_into().expect("could not convert usize to u64"))) {
                    acc.push(number);
                }
                acc
            });
        let binary_tree = BinaryTree::new(data);
        let root = binary_tree.get_root();
        let mut can_be_true = false;
        dfs(
            &binary_tree,
            &root,
            **root.value,
            1,
            &max_level,
            &self.expected,
            &mut can_be_true,
        );
        can_be_true
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    data: Vec<T>,
}

struct Node<T> {
    value: T,
    index: usize,
}

impl<T> BinaryTree<T> {
    fn new(data: Vec<T>) -> BinaryTree<T> {
        BinaryTree { data }
    }

    fn get_root(&self) -> Node<&T> {
        Node {
            value: &self.data[0],
            index: 0,
        }
    }

    fn get_left(&self, node: usize) -> Option<Node<&T>> {
        let index = 2 * node + 1;
        if index >= self.data.len() {
            return Option::None;
        }
        Option::Some(Node {
            value: &self.data[index],
            index,
        })
    }

    fn get_right(&self, node: usize) -> Option<Node<&T>> {
        let index = 2 * node + 2;
        if index >= self.data.len() {
            return Option::None;
        }
        Option::Some(Node {
            value: &self.data[index],
            index,
        })
    }
}

fn parse_input(input: String) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let expected = split
                .next()
                .expect("could not get expected number")
                .parse::<u64>()
                .expect("coult not parse expected number");
            let numbers = split
                .next()
                .expect("coult not get numbers string")
                .split(" ")
                .map(|number_string| {
                    number_string
                        .parse::<u64>()
                        .expect("coult not parse number string")
                })
                .collect::<Vec<_>>();
            Operation { expected, numbers }
        })
        .collect::<Vec<_>>()
}

fn dfs(
    binary_tree: &BinaryTree<&u64>,
    node: &Node<&&u64>,
    res: u64,
    level: usize,
    max_level: &usize,
    expected: &u64,
    can_be_true: &mut bool,
) {
    if level == *max_level && *expected == res {
        *can_be_true = true;
    }
    if let Some(left_node) = binary_tree.get_left(node.index) {
        dfs(
            binary_tree,
            &left_node,
            res * **left_node.value,
            level + 1,
            max_level,
            expected,
            can_be_true,
        );
    };
    if let Some(right_node) = binary_tree.get_right(node.index) {
        dfs(
            binary_tree,
            &right_node,
            res + **right_node.value,
            level + 1,
            max_level,
            expected,
            can_be_true,
        );
    };
}

pub fn main() {
    let input = fs::read_to_string("src/day07/input.txt").expect("could not read input file");
    let operations = parse_input(input);
    let res = operations
        .iter()
        .filter(|operation| operation.can_be_true())
        .map(|operation| operation.expected)
        .sum::<u64>();
    println!("{res}");
}
