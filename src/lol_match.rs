use crate::{queue::queue_type::QueueType, region::routing::RoutingRegion};
use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    metadata: Metadata,
    info: Info,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    data_version: String,
    match_id: String,
    participants: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "endOfGameResult")]
    end_of_game_result: String,
    #[serde(rename = "gameCreation")]
    game_creation: u64,
    #[serde(rename = "gameDuration")]
    game_duration: u64,
    #[serde(rename = "gameEndTimestamp")]
    game_end_timestamp: Option<u64>,
    #[serde(rename = "gameId")]
    game_id: u64,
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "mapId")]
    map_id: i64,
    participants: Vec<Participant>,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "queueId")]
    queue_id: i64,
    teams: Vec<Team>,
    #[serde(rename = "tournamentCode")]
    tournament_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    all_in_pings: i64,
    assist_me_pings: i64,
    assists: i64,
    baron_kills: i64,
    bounty_level: i64,
    champ_experience: i64,
    champ_level: i64,
    champion_id: i64,
    champion_name: String,
    command_pings: i64,
    /// Only used for kayn
    champion_transform: Option<i64>,
    consumables_purchased: i64,
    challenges: Challenges,
    damage_dealt_to_buildings: i64,
    damage_dealt_to_objectives: i64,
    damage_dealt_to_turrets: i64,
    damage_self_mitigated: i64,
    deaths: i64,
    detector_wards_placed: i64,
    double_kills: i64,
    dragon_kills: i64,
    eligible_for_progression: bool,
    enemy_missing_pings: i64,
    enemy_vision_pings: i64,
    first_blood_assist: bool,
    first_blood_kill: bool,
    first_tower_assist: bool,
    game_ended_in_early_surrender: bool,
    game_ended_in_surrender: bool,
    hold_pings: i64,
    get_back_pings: i64,
    gold_earned: i64,
    gold_spent: i64,
    /// This is a guess, typically best to use team_position
    individual_position: String,
    inhibitor_kills: i64,
    inhibitor_takedowns: i64,
    inhibitors_lost: i64,
    item_0: i64,
    item_1: i64,
    item_2: i64,
    item_3: i64,
    item_4: i64,
    item_5: i64,
    item_6: i64,
    items_purchased: i64,
    killing_sprees: i64,
    kills: i64,
    lane: String,
    largest_critical_strike: i64,
    largest_killing_spree: i64,
    largest_multi_kill: i64,
    longest_time_spent_living: i64,
    magic_damage_dealt: i64,
    magic_damage_dealt_to_champions: i64,
    magic_damage_taken: i64,
    missions: Missions,
    /// Jungle camps & pet kills
    neutral_minions_killed: i64,
    need_vision_pings: i64,
    nexus_kills: i64,
    nexus_takedowns: i64,
    nexus_lost: i64,
    objectives_stolen: i64,
    objectives_stolen_assists: i64,
    on_my_way_pings: i64,
    participant_id: i64,
    #[serde(rename = "playerScore0")]
    player_score0: Option<i64>,
    #[serde(rename = "playerScore1")]
    player_score1: Option<i64>,
    #[serde(rename = "playerScore2")]
    player_score2: Option<i64>,
    #[serde(rename = "playerScore3")]
    player_score3: Option<i64>,
    #[serde(rename = "playerScore4")]
    player_score4: Option<i64>,
    #[serde(rename = "playerScore5")]
    player_score5: Option<i64>,
    #[serde(rename = "playerScore6")]
    player_score6: Option<i64>,
    #[serde(rename = "playerScore7")]
    player_score7: Option<i64>,
    #[serde(rename = "playerScore8")]
    player_score8: Option<i64>,
    #[serde(rename = "playerScore9")]
    player_score9: Option<i64>,
    #[serde(rename = "playerScore10")]
    player_score10: Option<i64>,
    #[serde(rename = "playerScore11")]
    player_score11: Option<i64>,
    penta_kills: i64,
    perks: Perks,
    physical_damage_dealt: i64,
    physical_damage_dealt_to_champions: i64,
    physical_damage_taken: i64,
    placement: i64,
    player_augment_1: i64,
    player_augment_2: i64,
    player_augment_3: i64,
    player_augment_4: i64,
    player_subteam_id: i64,
    push_pings: i64,
    profile_icon: i64,
    puuid: String,
    quadra_kills: i64,
    riot_id_game_name: String,
    riot_id_tagline: String,
    role: String,
    sight_wards_bought_in_game: i64,
    spell1_casts: i64,
    spell2_casts: i64,
    spell3_casts: i64,
    spell4_casts: i64,
    subteam_placement: i64,
    summoner1_casts: i64,
    summoner1_id: i64,
    summoner2_casts: i64,
    summoner2_id: i64,
    summoner_id: String,
    summoner_level: i64,
    summoner_name: String,
    team_early_surrendered: bool,
    team_id: i64,
    team_position: String,
    #[serde(rename = "timeCCingOthers")]
    time_ccing_others: i64,
    time_played: i64,
    total_ally_jungle_minions_killed: i64,
    total_damage_dealt: i64,
    total_damage_dealt_to_champions: i64,
    total_damage_shielded_on_teammates: i64,
    total_damage_taken: i64,
    total_enemy_jungle_minions_killed: i64,
    /// Only direct heals, not regeneration
    total_heal: i64,
    total_heals_on_teammates: i64,
    /// Does not include jungle or pets
    total_minions_killed: i64,
    #[serde(rename = "totalTimeCCDealt")]
    total_time_cc_dealt: i64,
    total_time_spent_dead: i64,
    total_units_healed: i64,
    triple_kills: i64,
    true_damage_dealt: i64,
    true_damage_dealt_to_champions: i64,
    true_damage_taken: i64,
    turret_kills: i64,
    turret_takedowns: i64,
    turrets_lost: i64,
    /// Hexakill?
    unreal_kills: i64,
    vision_score: i64,
    vision_cleared_pings: i64,
    vision_wards_bought_in_game: i64,
    wards_killed: i64,
    wards_placed: i64,
    win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    #[serde(rename = "12AssistStreakCount")]
    pub n12assist_streak_count: Option<i64>,
    #[serde(rename = "HealFromMapSources")]
    pub heal_from_map_sources: f64,
    #[serde(rename = "InfernalScalePickup")]
    pub infernal_scale_pickup: Option<i64>,
    #[serde(rename = "SWARM_DefeatAatrox")]
    pub swarm_defeat_aatrox: Option<i64>,
    #[serde(rename = "SWARM_DefeatBriar")]
    pub swarm_defeat_briar: Option<i64>,
    #[serde(rename = "SWARM_DefeatMiniBosses")]
    pub swarm_defeat_mini_bosses: Option<i64>,
    #[serde(rename = "SWARM_EvolveWeapon")]
    pub swarm_evolve_weapon: Option<i64>,
    #[serde(rename = "SWARM_Have3Passives")]
    pub swarm_have3passives: Option<i64>,
    #[serde(rename = "SWARM_KillEnemy")]
    pub swarm_kill_enemy: Option<i64>,
    #[serde(rename = "SWARM_PickupGold")]
    pub swarm_pickup_gold: Option<i64>,
    #[serde(rename = "SWARM_ReachLevel50")]
    pub swarm_reach_level50: Option<i64>,
    #[serde(rename = "SWARM_Survive15Min")]
    pub swarm_survive15min: Option<i64>,
    #[serde(rename = "SWARM_WinWith5EvolvedWeapons")]
    pub swarm_win_with5evolved_weapons: Option<i64>,
    pub ability_uses: Option<i64>,
    #[serde(rename = "acesBefore15Minutes")]
    pub aces_before15minutes: Option<i64>,
    pub allied_jungle_monster_kills: Option<i64>,
    pub baron_takedowns: Option<i64>,
    pub blast_cone_opposite_opponent_count: Option<i64>,
    pub bounty_gold: Option<f64>,
    pub buffs_stolen: Option<i64>,
    pub complete_support_quest_in_time: Option<i64>,
    pub control_ward_time_coverage_in_river_or_enemy_half: Option<f64>,
    pub control_wards_placed: Option<i64>,
    pub damage_per_minute: f64,
    pub damage_taken_on_team_percentage: f64,
    pub danced_with_rift_herald: Option<i64>,
    pub deaths_by_enemy_champs: Option<i64>,
    pub dodge_skill_shots_small_window: Option<i64>,
    pub double_aces: Option<i64>,
    pub dragon_takedowns: Option<i64>,
    pub early_laning_phase_gold_exp_advantage: Option<i64>,
    pub effective_heal_and_shielding: f64,
    pub elder_dragon_kills_with_opposing_soul: Option<i64>,
    pub elder_dragon_multikills: Option<i64>,
    pub enemy_champion_immobilizations: Option<i64>,
    pub enemy_jungle_monster_kills: Option<i64>,
    pub epic_monster_kills_near_enemy_jungler: Option<i64>,
    #[serde(rename = "epicMonsterKillsWithin30SecondsOfSpawn")]
    pub epic_monster_kills_within30seconds_of_spawn: Option<i64>,
    pub epic_monster_steals: Option<i64>,
    pub epic_monster_stolen_without_smite: Option<i64>,
    pub first_turret_killed: Option<i64>,
    pub fist_bump_participation: Option<i64>,
    pub flawless_aces: Option<i64>,
    pub full_team_takedown: Option<i64>,
    pub game_length: f64,
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<i64>,
    pub gold_per_minute: f64,
    pub had_open_nexus: Option<i64>,
    pub immobilize_and_kill_with_ally: Option<i64>,
    pub initial_buff_count: Option<i64>,
    pub initial_crab_count: Option<i64>,
    #[serde(rename = "jungleCsBefore10Minutes")]
    pub jungle_cs_before10minutes: f64,
    pub jungler_takedowns_near_damaged_epic_monster: Option<i64>,
    pub k_turrets_destroyed_before_plates_fall: Option<i64>,
    pub kda: f64,
    pub kill_after_hidden_with_ally: Option<i64>,
    pub kill_participation: f64,
    pub killed_champ_took_full_team_damage_survived: Option<i64>,
    pub killing_sprees: Option<i64>,
    pub kills_near_enemy_turret: Option<i64>,
    pub kills_on_other_lanes_early_jungle_as_laner: Option<i64>,
    pub kills_on_recently_healed_by_aram_pack: Option<i64>,
    pub kills_under_own_turret: Option<i64>,
    pub kills_with_help_from_epic_monster: Option<i64>,
    pub knock_enemy_into_team_and_kill: Option<i64>,
    pub land_skill_shots_early_game: Option<i64>,
    #[serde(rename = "laneMinionsFirst10Minutes")]
    pub lane_minions_first10minutes: Option<i64>,
    pub laning_phase_gold_exp_advantage: Option<i64>,
    pub legendary_count: Option<i64>,
    pub legendary_item_used: Vec<i64>,
    pub lost_an_inhibitor: Option<i64>,
    pub max_cs_advantage_on_lane_opponent: Option<f64>,
    pub max_kill_deficit: Option<i64>,
    pub max_level_lead_lane_opponent: Option<i64>,
    pub mejais_full_stack_in_time: Option<i64>,
    pub more_enemy_jungle_than_opponent: f64,
    pub multi_kill_one_spell: Option<i64>,
    pub multi_turret_rift_herald_count: Option<i64>,
    pub multikills: Option<i64>,
    pub multikills_after_aggressive_flash: Option<i64>,
    #[serde(rename = "outerTurretExecutesBefore10Minutes")]
    pub outer_turret_executes_before10minutes: Option<i64>,
    pub outnumbered_kills: Option<i64>,
    pub outnumbered_nexus_kill: Option<i64>,
    pub perfect_dragon_souls_taken: Option<i64>,
    pub perfect_game: Option<i64>,
    pub pick_kill_with_ally: Option<i64>,
    pub played_champ_select_position: Option<i64>,
    pub poro_explosions: Option<i64>,
    pub quick_cleanse: Option<i64>,
    pub quick_first_turret: Option<i64>,
    pub quick_solo_kills: Option<i64>,
    pub rift_herald_takedowns: Option<i64>,
    pub save_ally_from_death: Option<i64>,
    pub scuttle_crab_kills: Option<i64>,
    pub skillshots_dodged: Option<i64>,
    pub skillshots_hit: Option<i64>,
    pub snowballs_hit: Option<i64>,
    pub solo_baron_kills: Option<i64>,
    pub solo_kills: Option<i64>,
    pub stealth_wards_placed: Option<i64>,
    pub survived_single_digit_hp_count: Option<i64>,
    pub survived_three_immobilizes_in_fight: Option<i64>,
    pub takedown_on_first_turret: Option<i64>,
    pub takedowns: Option<i64>,
    pub takedowns_after_gaining_level_advantage: Option<i64>,
    pub takedowns_before_jungle_minion_spawn: Option<i64>,
    #[serde(rename = "takedownsFirstXMinutes")]
    pub takedowns_first_xminutes: Option<i64>,
    pub takedowns_in_alcove: Option<i64>,
    pub takedowns_in_enemy_fountain: Option<i64>,
    pub team_baron_kills: Option<i64>,
    pub team_damage_percentage: f64,
    pub team_elder_dragon_kills: Option<i64>,
    pub team_rift_herald_kills: Option<i64>,
    pub took_large_damage_survived: Option<i64>,
    pub turret_plates_taken: Option<i64>,
    pub turret_takedowns: Option<i64>,
    pub turrets_taken_with_rift_herald: Option<i64>,
    #[serde(rename = "twentyMinionsIn3SecondsCount")]
    pub twenty_minions_in3seconds_count: Option<i64>,
    pub two_wards_one_sweeper_count: Option<i64>,
    pub unseen_recalls: Option<i64>,
    pub vision_score_advantage_lane_opponent: f64,
    pub vision_score_per_minute: f64,
    pub void_monster_kill: Option<i64>,
    pub ward_takedowns: Option<i64>,
    #[serde(rename = "wardTakedownsBefore20M")]
    pub ward_takedowns_before20m: Option<i64>,
    pub wards_guarded: Option<i64>,
    pub earliest_dragon_takedown: Option<f64>,
    pub highest_crowd_control_score: Option<i64>,
    pub jungler_kills_early_jungle: Option<i64>,
    pub kills_on_laners_early_jungle_as_jungler: Option<i64>,
    pub teleport_takedowns: Option<i64>,
    pub baron_buff_gold_advantage_over_threshold: Option<i64>,
    pub earliest_baron: Option<f64>,
    pub earliest_elder_dragon: Option<f64>,
    pub first_turret_killed_time: Option<f64>,
    pub highest_champion_damage: Option<i64>,
    pub solo_turrets_lategame: Option<i64>,
    pub faster_support_quest_completion: Option<i64>,
    pub highest_ward_kills: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Missions {
    player_score0: i64,
    player_score1: i64,
    player_score2: i64,
    player_score3: i64,
    player_score4: i64,
    player_score5: i64,
    player_score6: i64,
    player_score7: i64,
    player_score8: i64,
    player_score9: i64,
    player_score10: i64,
    player_score11: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    stat_perks: PerkStats,
    styles: Vec<PerkStyle>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStats {
    defense: i64,
    flex: i64,
    offense: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyle {
    description: String,
    selections: Vec<PerkStyleSelection>,
    style: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleSelection {
    perk: i64,
    var1: i64,
    var2: i64,
    var3: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    bans: Vec<Ban>,
    objectives: Objectives,
    team_id: i64,
    win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    champion_id: i64,
    pick_turn: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    baron: Objective,
    champion: Objective,
    dragon: Objective,
    horde: Objective,
    inhibitor: Objective,
    rift_herald: Objective,
    tower: Objective,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    first: bool,
    kills: i64,
}

impl Match {
    pub async fn by_match_id(
        region: &RoutingRegion,
        api_key: &String,
        match_id: &String,
    ) -> Option<Self> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",
            region.to_string(),
            match_id,
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
                error!("{e:?}, {:?}", resp);
                None
            }
        }
    }

    pub async fn by_puuid(
        region: &RoutingRegion,
        api_key: &String,
        puuid: &String,
        start_time: Option<u64>,
        end_time: Option<u64>,
        queue: Option<i64>,
        queue_type: Option<QueueType>,
        start: Option<i64>,
        count: Option<i64>,
    ) -> Option<Vec<String>> {
        let mut request_url = format!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids",
            region.to_string(),
            puuid,
        );
        if start.is_some() {
            request_url = format!("{}?start={}", request_url, start.unwrap());
        } else {
            request_url = format!("{}?start=20", request_url);
        }
        if start_time.is_some() {
            request_url = format!("{}startTime={}", request_url, start_time.unwrap());
        }
        if end_time.is_some() {
            request_url = format!("{}&endTime={}", request_url, end_time.unwrap());
        }
        if queue.is_some() {
            request_url = format!("{}&queue={}", request_url, queue.unwrap());
        }
        if queue_type.is_some() {
            request_url = format!("{}&type={}", request_url, queue_type.unwrap());
        }
        if count.is_some() {
            request_url = format!("{}&count={}", request_url, count.unwrap());
        }
        request_url = format!("{}&api_key={}", request_url, api_key);
        let resp = reqwest::get(request_url).await.unwrap();
        let resp = resp.text().await.expect("Could not parse");
        match serde_json::from_str(&resp) {
            Ok(t) => {
                debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
                Some(t)
            }
            Err(e) => {
                error!("{e:?}, {:?}", resp);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTimeline {
    data_version: String,
    match_id: String,
    /// List of participant PUUIDs
    participants: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InfoTimeline {
    end_of_game_result: String,
    frame_interval: u64,
    game_id: u64,
    participants: Vec<ParticipantTimeline>,
    frames: Vec<FramesTimeline>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimeline {
    participant_id: i64,
    puuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FramesTimeline {
    events: Vec<EventsTimeline>,
    participant_frames: ParticipantFrames,
    timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventsTimeline {
    timestamp: u64,
    real_timestamp: u64,
    #[serde(rename = "type")]
    event_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrames {
    #[serde(rename = "one")]
    one: ParticipantFrame,
    #[serde(rename = "two")]
    two: ParticipantFrame,
    #[serde(rename = "three")]
    three: ParticipantFrame,
    #[serde(rename = "four")]
    four: ParticipantFrame,
    #[serde(rename = "five")]
    five: ParticipantFrame,
    #[serde(rename = "six")]
    six: ParticipantFrame,
    #[serde(rename = "seven")]
    seven: ParticipantFrame,
    #[serde(rename = "eight")]
    eight: ParticipantFrame,
    #[serde(rename = "nine")]
    nine: ParticipantFrame,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrame {
    champion_stats: ChampionStats,
    current_gold: i64,
    damage_stats: DamageStats,
    gold_per_second: i64,
    jungle_minions_killed: i64,
    level: i64,
    minions_killed: i64,
    participant_id: i64,
    position: Position,
    time_enemy_spent_controlled: i64,
    total_gold: i64,
    xp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    ability_haste: i64,
    ability_power: i64,
    armor: i64,
    armor_pen: i64,
    armor_pen_percent: i64,
    attack_damage: i64,
    attack_speed: i64,
    bonus_armor_pen_percent: i64,
    bonus_magic_pen_percent: i64,
    cc_reduction: i64,
    cooldown_reduction: i64,
    health: i64,
    health_max: i64,
    health_regen: i64,
    lifesteal: i64,
    magic_pen: i64,
    magic_pen_percent: i64,
    magic_resist: i64,
    movement_speed: i64,
    omnivamp: i64,
    physical_vammp: i64,
    power: i64,
    power_max: i64,
    power_regen: i64,
    spell_vamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    magic_damage_done: i64,
    magic_damage_done_to_champions: i64,
    magic_damage_taken: i64,
    physical_damage_done: i64,
    physical_damage_done_to_champions: i64,
    physical_damage_taken: i64,
    total_damage_done: i64,
    total_damage_done_to_champions: i64,
    total_damage_taken: i64,
    true_damage_done: i64,
    true_damage_done_to_champions: i64,
    true_damage_taken: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    x: i64,
    y: i64,
}

impl Timeline {
    pub async fn by_match_id(
        region: &RoutingRegion,
        api_key: &String,
        match_id: &String,
    ) -> Option<Timeline> {
        let request_url = format!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}",
            region.to_string(),
            match_id,
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
                error!("{e:?}, {:?}", resp);
                None
            }
        }
    }
}
