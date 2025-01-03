use std::collections::{HashMap, HashSet};

use crate::{
    common::strings::TrimmedSplit,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble() -> Quest {
    Quest {
        title: "Quest 2024 6: The Tree of Titans".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 6),
        solution: Box::new(Q2024_6),
    }
}

struct Q2024_6;

impl Solution for Q2024_6 {
    fn part_one(&self, input: &str) -> String {
        let tree = parse(input);
        find_unique_path(&tree).expect("Path not found").join("")
    }

    fn part_two(&self, input: &str) -> String {
        let tree = parse(input);
        find_unique_path(&tree)
            .expect("Path not found")
            .iter()
            .filter_map(|s| s.chars().next())
            .collect::<String>()
    }

    fn part_three(&self, input: &str) -> String {
        self.part_two(input)
    }
}

type Node = String;
type Tree = HashMap<Node, HashSet<Node>>;

fn parse(input: &str) -> Tree {
    let mut tree = Tree::new();
    for line in input.trimmed_split() {
        let (node, connections) = line.split_once(':').expect("Invalid input format");
        let set = tree.entry(node.to_string()).or_default();
        for connection in connections.split(',') {
            set.insert(connection.to_string());
        }
    }
    tree
}

fn find_unique_path(tree: &Tree) -> Option<Vec<String>> {
    fn dfs<'l>(
        tree: &'l Tree,
        node: &'l str,
        path: &mut Vec<&'l str>,
        output: &mut HashMap<usize, Vec<Vec<Node>>>,
    ) {
        if node == "@" {
            let mut value = path.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            value.push("@".to_string());
            let entry = output.entry(path.len()).or_default();
            entry.push(value);
            return;
        };
        if path.contains(&node) {
            return;
        }
        let Some(connections) = tree.get(node) else {
            return;
        };
        path.push(node);
        for child in connections {
            dfs(tree, child, path, output);
        }
        path.pop();
    }

    let mut output = HashMap::new();

    dfs(tree, "RR", &mut Vec::new(), &mut output);

    output
        .iter()
        .find(|(_, val)| val.len() == 1)
        .and_then(|(_, set)| set.iter().next().cloned())
}
