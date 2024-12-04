use crate::quests::quest_1::assemble_quest_1;

pub mod quest_1;
pub mod quest_2;

pub struct QuestFactory;
impl QuestFactory {
    pub fn quest(&self, number: usize) -> Option<Quest> {
        match number {
            1 => Some(assemble_quest_1()),
            _ => None,
        }
    }

    pub fn custom(&self) -> Option<Quest> {
        self.quest(1)
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
    pub fn with_sources(path_list: &[&str]) -> QuestInputLoader {
        let sources = path_list
            .iter()
            .map(|s| s.to_string())
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
