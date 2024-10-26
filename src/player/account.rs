use log::{debug, error};
use reqwest::header::WARNING;
use serde::{Deserialize, Serialize};

use crate::region::routing::RoutingRegion;

#[derive(Serialize, Deserialize, Debug)]
pub struct RiotAccount {
    puuid: String,
    game_name: Option<String>,
    tag_line: Option<String>,
}

impl RiotAccount {
    /// Requests Riot account information from api.
    ///
    /// Returns None on error or if the account does not exist.
    pub async fn by_riot_id(
        region: RoutingRegion,
        api_key: &String,
        game_name: &String,
        tag_line: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
            region.to_string(),
            game_name,
            tag_line,
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

    pub async fn by_puuid(region: RoutingRegion, api_key: &String, puuid: &String) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/riot/account/v1/accounts/by-puuid/{}?api_key={}",
            region.to_string(),
            puuid,
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

    pub fn get_puuid(&self) -> &String {
        &self.puuid
    }

    pub fn get_game_name(&self) -> &Option<String> {
        &self.game_name
    }

    pub fn get_tag_line(&self) -> &Option<String> {
        &self.tag_line
    }

    pub fn new(puuid: String, game_name: Option<String>, tag_line: Option<String>) -> Self {
        Self {
            puuid,
            game_name,
            tag_line,
        }
    }
}
