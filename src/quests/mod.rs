use quest_2024_01::assemble_quest_2024_1;
use quest_2024_02::assemble_quest_2024_2;
use quest_2024_03::assemble_quest_2024_3;
use quest_2024_04::assemble_quest_2024_4;
use quest_2024_05::assemble_quest_2024_5;
use quest_2024_06::assemble_quest_2024_6;

mod quest_2024_01;
mod quest_2024_02;
mod quest_2024_03;
mod quest_2024_04;
mod quest_2024_05;
mod quest_2024_06;

pub struct QuestFactory;
impl QuestFactory {
    pub fn quest(&self, number: usize) -> Option<Quest> {
        match number {
            1 => Some(assemble_quest_2024_1()),
            2 => Some(assemble_quest_2024_2()),
            3 => Some(assemble_quest_2024_3()),
            4 => Some(assemble_quest_2024_4()),
            5 => Some(assemble_quest_2024_5()),
            6 => Some(assemble_quest_2024_6()),
            _ => None,
        }
    }

    pub fn custom(&self) -> Option<Quest> {
        self.quest(6)
    }
}

#[derive(Debug)]
pub enum QuestError {
    IoError,
    NoInput,
}

pub type QuestResult<T> = Result<T, QuestError>;

pub struct Quest {
    pub title: String,
    pub input_loader: QuestInputLoader,
    pub solution: Box<dyn Solution>,
}

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn part_three(&self, input: &str) -> String;
}

pub struct QuestInputLoader {
    sources: Vec<String>,
}

impl QuestInputLoader {
    pub fn with_quest_date(year: usize, number: usize) -> Self {
        let path_list = (1..=3)
            .map(|part| format!("everybody_codes_e{}_q{:02}_p{}.txt", year, number, part))
            // .inspect(|p| println!("{p}"))
            .map(|s| format!("input/{}", s))
            .collect::<Vec<_>>();
        Self::with_sources(&path_list)
    }

    pub fn with_sources<T: AsRef<str>>(path_list: &[T]) -> Self {
        let sources = path_list
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>();
        Self { sources }
    }

    pub fn load(&self, part: usize) -> QuestResult<String> {
        let Some(file) = self.sources.get(part) else {
            return Err(QuestError::NoInput);
        };
        let input = std::fs::read_to_string(file).map_err(|_| QuestError::IoError)?;
        Ok(input)
    }
}
