pub mod quest_1;
pub mod quest_2;

pub use quest_1::*;

#[derive(Debug)]
pub enum QuestError {
    IoError,
}

impl From<std::io::Error> for QuestError {
    fn from(_e: std::io::Error) -> Self {
        QuestError::IoError
    }
}

type QuestResult<T> = Result<T, QuestError>;
