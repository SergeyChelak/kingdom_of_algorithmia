use crate::common::strings::{reversed_str, TrimmedSplit};
use crate::quests::{Quest, QuestInputLoader, Solution};
use std::collections::HashSet;

pub fn assemble_quest_2024_2() -> Quest {
    Quest {
        title: "Quest 2: The Runes of Power".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 2),
        solution: Box::new(Q2024_2),
    }
}

struct Q2024_2;

impl Solution for Q2024_2 {
    fn part_one(&self, input: &str) -> String {
        let data = split_input(input);
        assert_eq!(data.len(), 2);
        words_count(&data[0], &data[1]).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let data = split_input(input);
        let words = split_words(&data[0]);
        assert!(data.len() > 1);
        data[1..]
            .iter()
            .map(|text| symbols_count(&words, text))
            .sum::<usize>()
            .to_string()
    }

    fn part_three(&self, input: &str) -> String {
        assert!(!input.is_empty());
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let data = split_input(input);
        for row in &data[1..] {
            let arr = row.chars().collect::<Vec<char>>();
            matrix.push(arr);
        }
        let words = split_words(&data[0])
            .iter()
            .map(|word| word.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

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
    matrix: &[Vec<char>],
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
fn split_input(input: &str) -> Vec<String> {
    input
        .trimmed_split()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn split_words(line: &str) -> Vec<&str> {
    let Some((_, words)) = line.split_once(":") else {
        panic!("Invalid format");
    };
    words.split(",").collect::<Vec<&str>>()
}

fn words_count(words: &str, text: &str) -> usize {
    split_words(words)
        .iter()
        .map(|word| text.match_indices(word).count())
        .sum::<usize>()
}

fn symbols_count(words: &[&str], text: &str) -> usize {
    let mut set = HashSet::<usize>::new();

    let reversed = reversed_str(text);
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
    use super::*;

    #[test]
    fn quest2_case_1() {
        let input = r"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        assert_eq!(Q2024_2.part_one(input), "4");
    }

    #[test]
    fn quest2_case_2() {
        let input = r"WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        assert_eq!(Q2024_2.part_two(input), "42");
    }

    #[test]
    fn quest2_case_3() {
        let input = r"WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";
        assert_eq!(Q2024_2.part_three(input), "10");
    }
}
