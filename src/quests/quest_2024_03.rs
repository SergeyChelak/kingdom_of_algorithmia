use crate::quests::{Quest, QuestInputLoader, Solution};

pub fn assemble_quest_2024_3() -> Quest {
    Quest {
        title: "Quest 2024 3: ???".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 3),
        solution: Box::new(Q2024_3),
    }
}

struct Q2024_3;

impl Solution for Q2024_3 {
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
