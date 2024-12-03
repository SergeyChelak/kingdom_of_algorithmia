use crate::quests::{QuestError, QuestResult};
use std::collections::HashMap;

// Quest 1: The Battle for the Farmlands
#[derive(Default)]
pub struct Quest1 {
    inputs: Vec<String>,
}

impl Quest1 {
    pub fn new() -> Quest1 {
        Self::default()
    }

    pub fn load(&mut self) -> QuestResult<()> {
        self.inputs.clear();
        let files = ["input/quest_1a", "input/quest_1b", "input/quest_1c"];
        for file in files {
            let input = std::fs::read_to_string(file).map_err(QuestError::from)?;
            self.inputs.push(input);
        }
        Ok(())
    }
    pub fn part_one(&self) -> String {
        calculate_poison_amount(&self.inputs[0], 1)
    }

    pub fn part_two(&self) -> String {
        calculate_poison_amount(&self.inputs[1], 2)
    }

    pub fn part_three(&self) -> String {
        calculate_poison_amount(&self.inputs[2], 3)
    }
}

pub fn calculate_poison_amount(input: &str, enemies: usize) -> String {
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
