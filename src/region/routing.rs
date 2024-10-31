use serde::{Deserialize, Serialize};

use super::server::ServerRegion;

#[derive(Serialize, Deserialize)]
pub enum RoutingRegion {
    EUROPE,
    AMERICAS,
    ASIA,
    SEA,
    ESPORTS,
}

impl ToString for RoutingRegion {
    fn to_string(&self) -> String {
        match self {
            Self::EUROPE => String::from("europe"),
            Self::AMERICAS => String::from("americas"),
            Self::ASIA => String::from("asia"),
            Self::SEA => String::from("sea"),
            Self::ESPORTS => String::from("esports"),
        }
    }
}

impl RoutingRegion {
    pub fn from_server(server: &ServerRegion) -> Self {
        match server {
            ServerRegion::NA1 | ServerRegion::BR1 | ServerRegion::LA1 | ServerRegion::LA2 => {
                Self::AMERICAS
            }
            ServerRegion::KR | ServerRegion::JP1 => Self::ASIA,
            ServerRegion::EUN1
            | ServerRegion::EUW1
            | ServerRegion::ME1
            | ServerRegion::TR1
            | ServerRegion::RU => Self::EUROPE,
            ServerRegion::OC1
            | ServerRegion::PH2
            | ServerRegion::SG2
            | ServerRegion::TH2
            | ServerRegion::TW2
            | ServerRegion::VN2 => Self::SEA,
        }
    }
}
