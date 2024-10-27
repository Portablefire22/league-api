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
            Self::IRON => write!("IRON"),
            Self::BRONZE => write!("BRONZE"),
            Self::SILVER => write!("SILVER"),
            Self::GOLD => write!("GOLD"),
            Self::PLATINUM => write!("PLATINUM"),
            Self::EMERALD => write!("EMERALD"),
            Self::DIAMOND => write!("DIAMOND"),
            Self::MASTER => write!("MASTER"),
            Self::GRANDMASTER => write!("GRANDMASTER"),
            Self::CHALLENGER => write!("CHALLENGER"),
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
