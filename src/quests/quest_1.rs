use crate::quests::{Quest, QuestInputLoader, Solution};
use std::collections::HashMap;

pub fn assemble_quest_1() -> Quest {
    Quest {
        title: "Quest 1: The Battle for the Farmlands".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 1),
        solution: Box::new(Q1),
    }
}

struct Q1;

impl Solution for Q1 {
    fn part_one(&self, input: &str) -> String {
        calculate_potion_amount(input, 1)
    }

    fn part_two(&self, input: &str) -> String {
        calculate_potion_amount(input, 2)
    }

    fn part_three(&self, input: &str) -> String {
        calculate_potion_amount(input, 3)
    }
}

fn calculate_potion_amount(input: &str, enemies: usize) -> String {
    let map = HashMap::from([('A', 0), ('B', 1), ('C', 3), ('D', 5)]);
    input
        .chars()
        .collect::<Vec<char>>()
        .chunks(enemies)
        .map(|chunk| {
            let arr = (0..enemies)
                .filter_map(|i| chunk.get(i).and_then(|ch| map.get(ch)))
                .copied()
                .collect::<Vec<i32>>();
            arr.iter().sum::<i32>()
                + match arr.len() {
                    2 => 2,
                    3 => 6,
                    _ => 0,
                }
        })
        .sum::<i32>()
        .to_string()
}
