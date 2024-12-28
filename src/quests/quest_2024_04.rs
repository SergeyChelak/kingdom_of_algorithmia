use crate::{
    common::strings::TrimmedSplit,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble_quest_2024_4() -> Quest {
    Quest {
        title: "Quest 2024 4: Royal Smith's Puzzle".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 4),
        solution: Box::new(Q2024_4),
    }
}

type Int = u64;

struct Q2024_4;

impl Solution for Q2024_4 {
    fn part_one(&self, input: &str) -> String {
        let values = parse(input);
        get_min_strikes(&values).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.part_one(input)
    }

    fn part_three(&self, input: &str) -> String {
        let values = parse(input);
        get_min_bidirectional_strikes(&values).to_string()
    }
}

fn get_min_strikes(arr: &[Int]) -> Int {
    let Some(min) = arr.iter().min() else {
        return 0;
    };
    arr.iter().map(|x| x - min).sum()
}

fn get_min_bidirectional_strikes(arr: &[Int]) -> Int {
    arr.iter()
        .map(|nail| arr.iter().map(|x| x.abs_diff(*nail)).sum())
        .min()
        .unwrap_or_default()
}

fn parse(input: &str) -> Vec<Int> {
    input
        .trimmed_split()
        .map(|x| x.parse::<Int>().expect("Invalid input"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_4_part1() {
        let result = get_min_strikes(&[3, 4, 7, 8]);
        assert_eq!(result, 10);
    }

    #[test]
    fn quest2024_4_part3() {
        let result = get_min_bidirectional_strikes(&[2, 4, 5, 6, 8]);
        assert_eq!(result, 8);
    }
}
