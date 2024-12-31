use std::collections::{HashMap, HashSet};

use crate::{
    common::strings::TrimmedSplit,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble_quest_2024_6() -> Quest {
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
        find_unique_path(&tree).expect("Path not found")
    }

    fn part_two(&self, _input: &str) -> String {
        todo!()
    }

    fn part_three(&self, _input: &str) -> String {
        todo!()
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

fn find_unique_path(tree: &Tree) -> Option<String> {
    fn dfs<'l>(
        tree: &'l Tree,
        node: &'l str,
        path: &mut Vec<&'l str>,
        output: &mut HashMap<usize, Vec<String>>,
    ) {
        if node == "@" {
            let value = path.join("") + "@";
            let entry = output.entry(path.len()).or_default();
            entry.push(value);
            return;
        };
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
        .filter(|(_, val)| val.len() == 1)
        .next()
        .and_then(|(_, set)| set.iter().next().cloned())
}

// fn find_root_node(tree: &Tree) -> Option<Node> {
//     let all_keys = tree.keys().collect::<HashSet<_>>();
//     let all_connections = tree.values().flatten().collect::<HashSet<_>>();
//     all_keys
//         .difference(&all_connections)
//         .next()
//         .map(|x| x.to_string())
// }
