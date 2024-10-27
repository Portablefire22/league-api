use crate::{
    queue::ranked::RankedQueue,
    ranked::{division::RankedDivision, tier::RankedTier},
    region::server::ServerRegion,
};
use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueEntry {
    #[serde(rename = "leagueId")]
    league_id: String,
    #[serde(rename = "summonerId")]
    summoner_id: String,
    #[serde(rename = "queueType")]
    queue_type: RankedQueue,
    tier: RankedTier,
    rank: RankedDivision,
    #[serde(rename = "leaguePoints")]
    league_points: u32,
    wins: u32,
    losses: u32,
    #[serde(rename = "hotStreak")]
    hot_streak: bool,
    veteran: bool,
    #[serde(rename = "freshBlood")]
    fresh_blood: bool,
    inactive: bool,
    #[serde(rename = "miniSeries")]
    mini_series: MiniSeries,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MiniSeries {
    losses: u32,
    progress: String,
    target: u32,
    wins: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueList {
    #[serde(rename = "leagueId")]
    league_id: String,
    entries: Vec<LeagueItem>,
    tier: RankedTier,
    name: String,
    queue: RankedQueue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueItem {
    fresh_blood: bool,
    wins: u32,
    #[serde(rename = "miniSeries")]
    mini_series: MiniSeries,
    inactive: bool,
    veteran: bool,
    #[serde(rename = "hotStreak")]
    hot_streak: bool,
    rank: String,
    #[serde(rename = "leaguePoints")]
    league_points: u32,
    losses: u32,
    #[serde(rename = "summonerId")]
    summoner_id: String,
}

impl LeagueList {
    pub async fn challenger_by_queue(
        region: ServerRegion,
        api_key: &String,
        queue: RankedQueue,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/league/v4/challengerleagues/by-queue/{}?api_key={}",
            region.to_string(),
            queue.to_string(),
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn grandmaster_by_queue(
        region: ServerRegion,
        api_key: &String,
        queue: RankedQueue,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/league/v4/grandmasterleagues/by-queue/{}?api_key={}",
            region.to_string(),
            queue.to_string(),
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn master_by_queue(
        region: ServerRegion,
        api_key: &String,
        queue: RankedQueue,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/league/v4/masterleagues/by-queue/{}?api_key={}",
            region.to_string(),
            queue.to_string(),
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn leagues(
        region: ServerRegion,
        api_key: &String,
        league_id: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/league/v4/leagues/{}?api_key={}",
            region.to_string(),
            league_id,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }
}

impl LeagueEntry {
    pub async fn by_summoner(
        region: ServerRegion,
        api_key: &String,
        summoner_id: &String,
    ) -> Option<Vec<Self>> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}",
            region.to_string(),
            summoner_id,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn entries(
        region: ServerRegion,
        api_key: &String,
        division: RankedDivision,
        tier: RankedTier,
        queue: RankedQueue,
        page: Option<u32>,
    ) -> Option<Vec<Self>> {
        let request_url: String;
        if page.is_some() {
            request_url = format!(
                "https://{}.api.riotgames.com/lol/league/v4/entries/{}/{}/{}?api_key={}",
                region.to_string(),
                queue.to_string(),
                tier.to_string(),
                division.to_string(),
                api_key
            );
        } else {
            request_url = format!(
                "https://{}.api.riotgames.com/lol/league/v4/entries/{}/{}/{}?page={}&api_key={}",
                region.to_string(),
                queue.to_string(),
                tier.to_string(),
                division.to_string(),
                page.unwrap(),
                api_key
            );
        }

        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }
}
