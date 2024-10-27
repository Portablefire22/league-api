use core::fmt;

use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RankedTier {
    IRON,
    BRONZE,
    SILVER,
    GOLD,
    PLATINUM,
    EMERALD,
    DIAMOND,
    MASTER,
    GRANDMASTER,
    CHALLENGER,
}

impl fmt::Display for RankedTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::IRON => write!(f, "IRON"),
            Self::BRONZE => write!(f, "BRONZE"),
            Self::SILVER => write!(f, "SILVER"),
            Self::GOLD => write!(f, "GOLD"),
            Self::PLATINUM => write!(f, "PLATINUM"),
            Self::EMERALD => write!(f, "EMERALD"),
            Self::DIAMOND => write!(f, "DIAMOND"),
            Self::MASTER => write!(f, "MASTER"),
            Self::GRANDMASTER => write!(f, "GRANDMASTER"),
            Self::CHALLENGER => write!(f, "CHALLENGER"),
        }
    }
}

impl From<&'static str> for RankedTier {
    fn from(value: &'static str) -> Self {
        match value {
            "IRON" => Self::IRON,
            "BRONZE" => Self::BRONZE,
            "SILVER" => Self::SILVER,
            "GOLD" => Self::GOLD,
            "PLATINUM" => Self::PLATINUM,
            "EMERALD" => Self::EMERALD,
            "DIAMOND" => Self::DIAMOND,
            "MASTER" => Self::MASTER,
            "GRANDMASTER" => Self::GRANDMASTER,
            "CHALLENGER" => Self::CHALLENGER,
            _ => {
                error!("Unknown tier '{}', defaulting to Emerald", value);
                Self::EMERALD
            }
        }
    }
}
