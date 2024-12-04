use crate::quests::quest_2::Quest2;

mod quests;

fn main() {
    let mut quest = Quest2::new();
    if let Err(err) = quest.load() {
        println!("[Error] {err:?}");
        return;
    }
    println!("Part 1: {}", quest.part_one());
    println!("Part 2: {}", quest.part_two());
    println!("Part 2: {}", quest.part_three());
}
