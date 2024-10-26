pub enum RoutingRegion {
    EUROPE,
    AMERICAS,
    ASIA,
    ESPORTS,
}

impl ToString for RoutingRegion {
    fn to_string(&self) -> String {
        match self {
            Self::EUROPE => String::from("europe"),
            Self::AMERICAS => String::from("americas"),
            Self::ASIA => String::from("asia"),
            Self::ESPORTS => String::from("esports"),
        }
    }
}
