use std::collections::HashMap;

use crate::{
    common::strings::TrimmedSplit,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble() -> Quest {
    Quest {
        title: "Quest 2024 7: Not Fast but Furious".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 7),
        solution: Box::new(Q2024_7),
    }
}

struct Q2024_7;

impl Solution for Q2024_7 {
    fn part_one(&self, input: &str) -> String {
        let data = parse(input);
        perform(data, 10)
    }

    fn part_two(&self, _input: &str) -> String {
        todo!()
    }

    fn part_three(&self, _input: &str) -> String {
        todo!()
    }
}

fn perform(data: HashMap<String, Vec<Action>>, segments: usize) -> String {
    let mut scores = HashMap::<String, Vec<usize>>::new();
    for segment in 0..segments {
        for key in data.keys() {
            let params = data.get(key).expect("Unreachable");
            let action = params[segment % params.len()];
            let scores = scores.entry(key.clone()).or_default();
            let mut score = *(scores.last().unwrap_or(&10));
            score = match action {
                Action::Dec => score.saturating_sub(1),
                Action::Inc => score + 1,
                _ => score,
            };
            scores.push(score);
        }
    }

    let mut scores = scores
        .iter()
        .map(|(key, scores)| {
            let sum = scores.iter().sum::<usize>();
            (key, sum)
        })
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores
        .iter()
        .map(|(val, _)| *val)
        .cloned()
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Inc,
    Dec,
    Keep,
}

fn parse(input: &str) -> HashMap<String, Vec<Action>> {
    input
        .trimmed_split()
        .map(parse_line)
        .collect::<HashMap<_, _>>()
}

fn parse_line(input: &str) -> (String, Vec<Action>) {
    let (name, actions) = input.split_once(':').expect("Invalid input format");
    let actions = actions
        .split(',')
        .map(|action| match action {
            "+" => Action::Inc,
            "-" => Action::Dec,
            "=" => Action::Keep,
            _ => panic!("Unexpected action"),
        })
        .collect::<Vec<_>>();
    (name.to_string(), actions)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_07_part1() {
        let input = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
        assert_eq!("BDCA", Q2024_7.part_one(input))
    }
}
