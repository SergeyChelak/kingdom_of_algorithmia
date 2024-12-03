use crate::quests::Quest1;

mod quests;

fn main() {
    let mut quest = Quest1::new();
    if let Err(err) = quest.load() {
        println!("[Error] {err:?}");
        return;
    }
    println!("Part 1: {}", quest.part_one());
    println!("Part 2: {}", quest.part_two());
    println!("Part 2: {}", quest.part_three());
}
