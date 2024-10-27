use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RankedQueue {
    #[serde(rename = "RANKED_SOLO_5X5")]
    RankedSolo5x5,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSR,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTT,
}

impl ToString for RankedQueue {
    fn to_string(&self) -> String {
        match &self {
            Self::RankedSolo5x5 => String::from("RANKED_SOLO_5X5"),
            Self::RankedFlexSR => String::from("RANKED_FLEX_SR"),
            Self::RankedFlexTT => String::from("RANKED_FLEX_TT"),
        }
    }
}

impl From<&'static str> for RankedQueue {
    fn from(value: &'static str) -> Self {
        match value {
            "RANKED_SOLO_5X5" => RankedQueue::RankedSolo5x5,
            "RANKED_FLEX_SR" => RankedQueue::RankedFlexSR,
            "RANKED_FLEX_TT" => RankedQueue::RankedFlexTT,
            _ => {
                error!("Unknown queue '{}', defaulting to RankedSolo5x5!", value);
                RankedQueue::RankedSolo5x5
            }
        }
    }
}
