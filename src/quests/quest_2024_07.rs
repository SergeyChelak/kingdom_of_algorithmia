use crate::quests::{Quest, QuestInputLoader, Solution};

pub fn assemble() -> Quest {
    Quest {
        title: "Quest 2024 7: ???".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 7),
        solution: Box::new(Q2024_7),
    }
}

struct Q2024_7;

impl Solution for Q2024_7 {
    fn part_one(&self, input: &str) -> String {
        todo!()
    }

    fn part_two(&self, input: &str) -> String {
        todo!()
    }

    fn part_three(&self, input: &str) -> String {
        todo!()
    }
}
