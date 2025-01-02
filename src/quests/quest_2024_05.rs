use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::{
    common::{strings::TrimmedSplit, Vec2},
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble() -> Quest {
    Quest {
        title: "Quest 2024 5: Pseudo-Random Clap Dance".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 5),
        solution: Box::new(Q2024_5),
    }
}

struct Q2024_5;

impl Solution for Q2024_5 {
    fn part_one(&self, input: &str) -> String {
        let mut data = parse(input);
        make_movements(&mut data, 10)
    }

    fn part_two(&self, input: &str) -> String {
        let mut data = parse(input);
        let mut map = HashMap::<String, usize>::new();
        for step in 0.. {
            process_movement(&mut data, step);
            let output = calculate_output(&data);
            let entry = map.entry(output).or_default();
            *entry += 1;
            if *entry == 2024 {
                let output = calculate_output(&data);
                let val = output.parse::<usize>().expect("Failed parse output value");
                return (val * (1 + step)).to_string();
            }
        }
        "Not found".to_string()
    }

    fn part_three(&self, input: &str) -> String {
        let mut data = parse(input);
        let mut set = HashSet::<String>::new();
        let mut result = String::new();
        let mut last_add = 0;
        for step in 0.. {
            process_movement(&mut data, step);
            let output = calculate_output(&data);
            if !set.insert(output.clone()) {
                if step - last_add > set.len() {
                    break;
                }
                continue;
            }
            last_add = step;
            if matches!(result.cmp(&output), Ordering::Less) {
                result = output;
            }
        }
        result
    }
}

type Int = usize;

fn make_movements(data: &mut Vec2<Int>, times: usize) -> String {
    for step in 0..times {
        process_movement(data, step);
    }
    calculate_output(data)
}

fn process_movement(data: &mut Vec2<Int>, step: usize) {
    let col = step % data.len();
    let val = data[col].remove(0);
    let next_col = (col + 1) % data.len();
    let next_len = data[next_col].len();
    let mut pos = (val - 1) % (2 * next_len);
    if pos > next_len {
        pos = 2 * next_len - pos;
    }
    data[next_col].insert(pos, val);
}

fn calculate_output(data: &[Vec<Int>]) -> String {
    data.iter()
        .map(|x| x.first().unwrap())
        .map(|x| x.to_string())
        .collect::<String>()
}

fn parse(input: &str) -> Vec2<Int> {
    let input = input
        .trimmed_split()
        .map(|s| {
            s.split(' ')
                .map(|x| x.parse::<Int>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    transpose(&input)
}

fn transpose(source: &[Vec<Int>]) -> Vec2<Int> {
    if source.is_empty() {
        return Vec::new();
    }
    let rows = source.len();
    let cols = source[0].len();
    let mut result = vec![vec![0; rows]; cols];
    for (r, row) in source.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            result[c][r] = *val;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_05_part1() {
        let input = "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";
        let mut data = parse(input);
        assert_eq!(make_movements(&mut data, 10), "2323");
    }

    #[test]
    fn quest2024_05_part3() {
        let input = "2 3 4 5
6 7 8 9";
        assert_eq!(Q2024_5.part_three(input), "6584");
    }
}
