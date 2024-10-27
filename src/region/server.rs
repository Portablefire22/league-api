use log::error;

pub enum ServerRegion {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    ME1,
    NA1,
    OC1,
    PH2,
    RU,
    SG2,
    TH2,
    TR1,
    TW2,
    VN2,
}

impl ToString for ServerRegion {
    fn to_string(&self) -> String {
        match &self {
            Self::BR1 => String::from("br1"),
            Self::EUN1 => String::from("eun1"),
            Self::EUW1 => String::from("euw1"),
            Self::JP1 => String::from("jp1"),
            Self::KR => String::from("kr"),
            Self::LA1 => String::from("la1"),
            Self::LA2 => String::from("la2"),
            Self::ME1 => String::from("me1"),
            Self::NA1 => String::from("na1"),
            Self::OC1 => String::from("oc1"),
            Self::PH2 => String::from("ph2"),
            Self::RU => String::from("ru"),
            Self::SG2 => String::from("sg2"),
            Self::TH2 => String::from("th2"),
            Self::TR1 => String::from("tr1"),
            Self::TW2 => String::from("tw2"),
            Self::VN2 => String::from("vn2"),
        }
    }
}

impl From<String> for ServerRegion {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "br" => Self::BR1,
            "eun1" => Self::EUN1,
            "euw1" => Self::EUW1,
            "jp1" => Self::JP1,
            "kr" => Self::KR,
            "la1" => Self::LA1,
            "la2" => Self::LA2,
            "me1" => Self::ME1,
            "na1" => Self::NA1,
            "oc1" => Self::OC1,
            "ph2" => Self::PH2,
            "ru" => Self::RU,
            "sg2" => Self::SG2,
            "th2" => Self::TH2,
            "tr1" => Self::TR1,
            "tw2" => Self::TW2,
            "vn2" => Self::VN2,
            _ => {
                error!("Invalid region '{}' defaulting to EUW1", value);
                Self::EUW1
            }
        }
    }
}
