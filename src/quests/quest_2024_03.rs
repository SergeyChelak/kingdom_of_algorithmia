use std::collections::HashSet;

use crate::{
    common::Position2,
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble_quest_2024_3() -> Quest {
    Quest {
        title: "Quest 3: Mining Maestro".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 3),
        solution: Box::new(Q2024_3),
    }
}

struct Q2024_3;

impl Solution for Q2024_3 {
    fn part_one(&self, input: &str) -> String {
        let mut area = make_area(input);
        let mut count = area.len();
        while !area.is_empty() {
            area = next_area(&area);
            count += area.len();
        }
        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        todo!()
    }

    fn part_three(&self, input: &str) -> String {
        todo!()
    }
}

type Position = Position2<isize>;

fn make_area(input: &str) -> HashSet<Position> {
    let mut area = HashSet::new();
    input
        .split('\n')
        // .map(|s| s.trim())
        // .filter(|s| !s.is_empty())
        .enumerate()
        .for_each(|(row, s)| {
            for (col, ch) in s.chars().enumerate() {
                if ch != '#' {
                    continue;
                }
                area.insert(Position::new(row as isize, col as isize));
            }
        });
    area
}

fn next_area(area: &HashSet<Position>) -> HashSet<Position> {
    let mut result = HashSet::new();
    for pos in area.iter() {
        let count = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(i, j)| Position::new(pos.row + i, pos.col + j))
            .filter(|pos| area.contains(pos))
            .count();
        if count == 4 {
            result.insert(*pos);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_3_part_1() {
        let input = "..........
..###.##..
...####...
..######..
..######..
...####...
..........";
        let quest = Q2024_3;
        assert_eq!(quest.part_one(&input), "35")
    }
}
