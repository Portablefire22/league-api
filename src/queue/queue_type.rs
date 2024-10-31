use core::fmt;

pub enum QueueType {
    RANKED,
    NORMAL,
    TOURNEY,
    TUTORIAL,
}

impl fmt::Display for QueueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::RANKED => write!(f, "RANKED"),
            Self::NORMAL => write!(f, "NORMAL"),
            Self::TOURNEY => write!(f, "TOURNEY"),
            Self::TUTORIAL => write!(f, "TUTORIAL"),
        }
    }
}
