use crate::{
    common::{strings::TrimmedSplit, Vec2},
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble_quest_2024_5() -> Quest {
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
        todo!()
    }

    fn part_three(&self, input: &str) -> String {
        todo!()
    }
}

type Int = u32;

fn make_movements(data: &mut Vec2<Int>, times: usize) -> String {
    for step in 0..times {
        let col = step % data.len();
        let val = data[col].remove(0);
        let mut is_forward = true;
        let mut idx = 0;
        let next_col = (col + 1) % data.len();
        for _ in 0..val {
            if idx == data[next_col].len() {
                is_forward = false;
                continue;
            }

            if is_forward {
                idx += 1;
            } else {
                idx -= 1;
            }
        }
        let pos = if is_forward { idx - 1 } else { idx };
        data[next_col].insert(pos, val);
        println!("{:?}", data);
    }
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
        // assert_eq!(make_movements(&mut data, 1), "3345");
        // assert_eq!(make_movements(&mut data, 2), "3245");
        // assert_eq!(make_movements(&mut data, 3), "3255");
        // assert_eq!(make_movements(&mut data, 4), "3252");
        assert_eq!(make_movements(&mut data, 10), "2323");
    }
}
