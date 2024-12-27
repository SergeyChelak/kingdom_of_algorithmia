use crate::{
    common::strings::split_into_trimmed_strings,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble_quest_2024_4() -> Quest {
    Quest {
        title: "Quest 2024 4: Royal Smith's Puzzle".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 4),
        solution: Box::new(Q2024_4),
    }
}

type Int = u32;

struct Q2024_4;

impl Solution for Q2024_4 {
    fn part_one(&self, input: &str) -> String {
        let values = split_into_trimmed_strings(input)
            .iter()
            .map(|x| x.parse::<Int>().expect("Invalid input"))
            .collect::<Vec<_>>();
        get_min_hammer_strikes(&values).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        todo!()
    }

    fn part_three(&self, input: &str) -> String {
        todo!()
    }
}

fn get_min_hammer_strikes(arr: &[Int]) -> Int {
    let Some(min) = arr.iter().min() else {
        return 0;
    };
    arr.iter().map(|x| x - min).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_4_part1() {
        let result = get_min_hammer_strikes(&[3, 4, 7, 8]);
        assert_eq!(result, 10);
    }
}
