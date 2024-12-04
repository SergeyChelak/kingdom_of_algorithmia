use crate::quests::{Quest, QuestFactory};
use std::time::Instant;

mod common;
mod quests;

fn main() {
    println!("Kingdom of Algorithmia");
    let factory = QuestFactory;
    let Some(quest) = factory.custom() else {
        println!("[Warn] No Quest");
        return;
    };
    execute(&quest);
}

fn execute(quest: &Quest) {
    println!();
    let mut description = quest.title.as_str();
    if description.is_empty() {
        description = "## UNTITLED QUEST ##";
    }
    println!("{}", description);
    let solution = &quest.solution;
    for part in 0..3 {
        let Ok(input) = quest.input_loader.load(part) else {
            println!("[Error] failed to load input for part {part}");
            continue;
        };
        let now = Instant::now();
        let result = match part {
            0 => solution.part_one(&input),
            1 => solution.part_two(&input),
            2 => solution.part_three(&input),
            _ => format!("[Warn] part {} not found", part),
        };
        let duration = now.elapsed().as_millis();
        let title = format!("{} ms for part {}", duration, part + 1);
        println!("{:>30}: {}", title, result);
    }
}
