use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RankedDivision {
    I,
    II,
    III,
    IV,
}

impl ToString for RankedDivision {
    fn to_string(&self) -> String {
        match &self {
            RankedDivision::I => String::from("I"),
            RankedDivision::II => String::from("II"),
            RankedDivision::III => String::from("III"),
            RankedDivision::IV => String::from("IV"),
        }
    }
}

impl From<&'static str> for RankedDivision {
    fn from(value: &'static str) -> Self {
        match value {
            "I" => RankedDivision::I,
            "II" => RankedDivision::II,
            "III" => RankedDivision::III,
            "IV" => RankedDivision::IV,
            _ => {
                error!("Invalid divison '{}', defaulting to I", value);
                RankedDivision::I
            }
        }
    }
}
