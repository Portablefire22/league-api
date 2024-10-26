use crate::region::server::ServerRegion;
use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Summoner {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "profileIconId")]
    profile_icon_id: u32,
    #[serde(rename = "revisionDate")]
    revision_date: u64,
    id: String,
    puuid: String,
    #[serde(rename = "summonerLevel")]
    summoner_level: u64,
}

impl Summoner {
    pub async fn by_rso_puuid(
        region: ServerRegion,
        api_key: &String,
        rso_puuid: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/fulfillment/v1/summoners/by-puuid/{}?api_key={}",
            region.to_string(),
            rso_puuid,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{:?}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn by_account(
        region: ServerRegion,
        api_key: &String,
        encrypted_account_id: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-account/{}?api_key={}",
            region.to_string(),
            encrypted_account_id,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{:?}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn by_puuid(region: ServerRegion, api_key: &String, puuid: &String) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}",
            region.to_string(),
            puuid,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{:?}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub async fn by_summoner_id(
        region: ServerRegion,
        api_key: &String,
        summoner_id: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/{}?api_key={}",
            region.to_string(),
            summoner_id,
            api_key
        );
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{:?}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    }

    pub fn get_account_id(&self) -> &String {
        &self.account_id
    }
    pub fn get_profile_icon_id(&self) -> u32 {
        self.profile_icon_id
    }
    pub fn get_revision_date(&self) -> u64 {
        self.revision_date
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_puuid(&self) -> &String {
        &self.puuid
    }
    pub fn get_summoner_level(&self) -> u64 {
        self.summoner_level
    }
}
