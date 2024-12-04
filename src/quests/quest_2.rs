use crate::quests::{QuestError, QuestResult};
use std::collections::HashSet;

#[derive(Default)]
pub struct Quest2 {
    inputs: Vec<String>,
}

impl Quest2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self) -> QuestResult<()> {
        self.inputs.clear();
        let files = ["input/quest_2a", "input/quest_2b", "input/quest_2c"];
        for file in files {
            let input = std::fs::read_to_string(file).map_err(QuestError::from)?;
            self.inputs.push(input);
        }
        Ok(())
    }

    pub fn part_one(&mut self) -> String {
        let data = split_input(&self.inputs[0]);
        assert_eq!(data.len(), 2);
        // println!("{:?}", data);
        words_count(&data[0], &data[1]).to_string()
    }

    pub fn part_two(&mut self) -> String {
        let data = split_input(&self.inputs[1]);
        assert!(data.len() > 1);
        data[1..]
            .iter()
            .map(|text| symbols_count(&data[0], text))
            .sum::<usize>()
            .to_string()
    }

    pub fn part_three(&mut self) -> String {
        assert!(!&self.inputs[2].is_empty());
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let data = split_input(&self.inputs[2]);
        for row in &data[1..] {
            let arr = row.chars().collect::<Vec<char>>();
            matrix.push(arr);
        }
        //
        println!("{:?}", matrix);
        //
        let Some((_, words)) = data[0].split_once(":") else {
            panic!("Invalid format");
        };
        let words = words
            .split(",")
            .inspect(|word| print!("{} ", word))
            .map(|word| word.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        println!();
        //
        let directions = [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ];
        let mut scales = HashSet::<(usize, usize)>::new();

        for word in words {
            for (row, arr) in matrix.iter().enumerate() {
                for (col, _) in arr.iter().enumerate() {
                    directions
                        .iter()
                        .filter_map(|dir| traverse(&matrix, row, col, *dir, &word))
                        .for_each(|set| {
                            scales.extend(set);
                        });
                }
            }
        }

        scales.len().to_string()
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn traverse(
    matrix: &Vec<Vec<char>>,
    mut row: usize,
    mut col: usize,
    direction: Direction,
    target: &[char],
) -> Option<HashSet<(usize, usize)>> {
    let mut output = HashSet::<(usize, usize)>::new();
    let mut can_move = true;
    for ch in target {
        if !can_move || *ch != matrix[row][col] {
            return None;
        }
        output.insert((row, col));

        let rows = matrix.len();
        let cols = matrix[row].len();
        col += cols;
        match direction {
            Direction::Up if row > 0 => row -= 1,
            Direction::Down if row < rows - 1 => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
            _ => can_move = false,
        }
        col %= cols;
    }
    Some(output)
}

fn reversed(s: &str) -> String {
    s.chars().rev().collect()
}

fn split_input(input: &str) -> Vec<String> {
    input
        // .split("\n")
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn words_count(words: &str, text: &str) -> usize {
    let Some((_, words)) = words.split_once(":") else {
        panic!("Invalid format");
    };
    words
        .split(",")
        .map(|word| text.match_indices(word).count())
        .sum::<usize>()
}

fn symbols_count(words: &str, text: &str) -> usize {
    let Some((_, words)) = words.split_once(":") else {
        panic!("Invalid format");
    };
    let words = words.split(",").collect::<Vec<&str>>();
    let mut set = HashSet::<usize>::new();

    let reversed = reversed(text);
    for word in words {
        let indices = text.match_indices(word);
        for (i, val) in indices {
            (i..i + val.len()).for_each(|x| {
                set.insert(x);
            })
        }

        let indices = reversed.match_indices(word);
        for (i, val) in indices {
            (i..i + val.len())
                .map(|x| reversed.len() - x - 1)
                .for_each(|x| {
                    set.insert(x);
                })
        }
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use crate::quests::quest_2::Quest2;

    #[test]
    fn quest2_case_1() {
        let mut q = Quest2 {
            inputs: vec![r"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
                .to_string()],
        };
        assert_eq!(q.part_one(), "4");
    }

    #[test]
    fn quest2_case_2() {
        let mut q = Quest2 {
            inputs: vec![
                "".to_string(),
                r"WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ"
                    .to_string(),
            ],
        };
        assert_eq!(q.part_two(), "42");
    }

    #[test]
    fn quest2_case_3() {
        let mut q = Quest2 {
            inputs: vec![
                "".to_string(),
                "".to_string(),
                r"WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
                    .to_string(),
            ],
        };
        assert_eq!(q.part_three(), "10");
    }
}
