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
    map_id: u32,
    participants: Vec<Participant>,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "queueId")]
    queue_id: String,
    teams: Vec<Team>,
    #[serde(rename = "tournamentCode")]
    tournament_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    all_in_pings: u32,
    assist_me_pings: u32,
    assists: u32,
    baron_kills: u32,
    bounty_level: u32,
    champ_experience: u32,
    champ_level: u32,
    champion_id: u32,
    champion_name: String,
    command_pings: u32,
    /// Only used for kayn
    champion_transform: Option<u32>,
    consumables_purchased: u32,
    challenges: Challenges,
    damage_dealt_to_buildings: u32,
    damage_dealt_to_objectives: u32,
    damage_dealt_to_turrets: u32,
    damage_self_mitigated: u32,
    deaths: u32,
    detector_wards_placed: u32,
    double_kills: u32,
    dragon_kills: u32,
    eligible_for_progession: bool,
    enemy_missing_pings: u32,
    enemy_vision_pings: u32,
    first_blood_assist: bool,
    first_blood_kill: bool,
    first_tower_assist: bool,
    game_ended_in_early_surrender: bool,
    game_ended_in_surrender: bool,
    hold_pings: u32,
    get_back_pings: u32,
    gold_earned: u32,
    gold_spent: u32,
    /// This is a guess, typically best to use team_position
    individual_position: String,
    inhibitor_kills: u32,
    inhibitor_takedowns: u32,
    inhibitors_lost: u32,
    item_0: u32,
    item_1: u32,
    item_2: u32,
    item_3: u32,
    item_4: u32,
    item_5: u32,
    item_6: u32,
    items_purchased: u32,
    killing_sprees: u32,
    kills: u32,
    lane: String,
    largest_critical_strike: u32,
    largest_killing_spree: u32,
    largest_multi_kill: u32,
    longest_time_spent_living: u32,
    magic_damage_dealt: u32,
    magic_damage_dealt_to_champions: u32,
    magic_damage_taken: u32,
    missions: Missions,
    /// Jungle camps & pet kills
    neutral_minions_killed: u32,
    need_vision_pings: u32,
    nexus_kills: u32,
    nexus_takedowns: u32,
    nexus_lost: u32,
    objectives_stolen: u32,
    objectives_stolen_assists: u32,
    on_my_way_pings: u32,
    participant_id: u32,
    player_score_0: u32,
    player_score_1: u32,
    player_score_2: u32,
    player_score_3: u32,
    player_score_4: u32,
    player_score_5: u32,
    player_score_6: u32,
    player_score_7: u32,
    player_score_8: u32,
    player_score_9: u32,
    player_score_10: u32,
    player_score_11: u32,
    penta_kills: u32,
    perks: Perks,
    physical_damage_dealt: u32,
    physical_damage_dealt_to_champions: u32,
    physical_damage_taken: u32,
    placement: u32,
    player_augment_1: u32,
    player_augment_2: u32,
    player_augment_3: u32,
    player_augment_4: u32,
    player_subteam_id: u32,
    push_pings: u32,
    profile_icon: u32,
    puuid: String,
    quadra_kills: u32,
    riot_id_game_name: String,
    riot_id_tagline: String,
    role: String,
    sight_wards_bought_in_game: u32,
    spell1_casts: u32,
    spell2_casts: u32,
    spell3_casts: u32,
    spell4_casts: u32,
    subteam_placement: u32,
    summoner1_casts: u32,
    summoner1_id: u32,
    summoner2_casts: u32,
    summoner2_id: u32,
    summoner_id: String,
    summoner_level: u32,
    summoner_name: String,
    team_early_surrendered: bool,
    team_id: u32,
    team_position: String,
    time_ccing_others: u32,
    time_played: u32,
    total_ally_jungle_minions_killed: u32,
    total_damage_dealt: u32,
    total_damage_dealt_to_champions: u32,
    total_damage_shielded_on_teammated: u32,
    total_damage_taken: u32,
    total_enemy_jungle_minions_killed: u32,
    /// Only direct heals, not regeneration
    total_heal: u32,
    total_heals_on_teammates: u32,
    /// Does not include jungle or pets
    total_minions_killed: u32,
    total_time_cc_dealt: u32,
    total_time_spent_dead: u32,
    total_units_healed: u32,
    triple_kills: u32,
    true_damage_dealt: u32,
    true_damage_dealt_to_champion: u32,
    true_damage_taken: u32,
    turret_kills: u32,
    turret_takedowns: u32,
    turrets_lost: u32,
    /// Hexakill?
    unreal_kills: u32,
    vision_score: u32,
    vision_cleared_pings: u32,
    vision_wards_bought_in_game: u32,
    wards_killed: u32,
    wards_placed: u32,
    win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    #[serde(rename = "12AssistStreakCount")]
    twelve_assist_streak_count: u32,
    baron_buff_gold_advantage_over_threshold: u32,
    control_ward_time_coverage_in_river_or_enemy_half: f32,
    earliest_baron: f32,
    earliest_dragon_takedown: f32,
    earliest_elder_dragon: f32,
    early_laning_phase_gold_exp_advantage: u32,
    faster_support_quest_completion: u32,
    fastest_legendary: u32,
    had_afk_teammate: u32,
    highest_champion_damage: u32,
    highest_crowd_control_score: u32,
    highest_ward_kills: u32,
    jungler_kills_early_jungle: u32,
    kills_on_laners_early_jungle_as_jungler: u32,
    laning_phase_gold_exp_advantage: u32,
    legendary_count: u32,
    max_cs_advantage_on_lane_opponent: f32,
    max_level_lead_lane_opponent: u32,
    most_wards_destroyed_one_sweeper: u32,
    mythic_item_used: u32,
    played_champ_select_position: u32,
    solo_turrets_lategame: u32,
    takedowns_first_25_minutes: u32,
    teleport_takedowns: u32,
    third_inhibitor_destroyed_time: u32,
    three_wards_one_sweeper_count: u32,
    vision_score_advantage_lane_opponent: f32,
    infernal_scale_pickup: u32,
    fist_bump_participation: u32,
    void_monster_kill: u32,
    ability_uses: u32,
    aces_before_15_minutes: u32,
    allied_jungle_monster_kills: f32,
    baron_takedowns: u32,
    blast_cone_opposite_opponent_count: u32,
    bounty_gold: u32,
    buffs_stolen: u32,
    complete_support_quest_in_time: u32,
    control_wards_placed: u32,
    damage_per_minute: f32,
    damage_taken_on_team_percentage: f32,
    danced_with_rift_herald: u32,
    deaths_by_enemy_champs: u32,
    dodge_skill_shots_small_window: u32,
    double_aces: u32,
    dragon_takedowns: u32,
    legendary_item_used: Vec<u32>,
    effective_heal_and_shielding: f32,
    elder_dragon_kills_with_opposing_soul: u32,
    elder_dragon_multi_kills: u32,
    enemy_champion_immobilizations: u32,
    enemy_jungle_monster_kills: u32,
    epic_monster_kills_near_enemy_jungler: u32,
    epic_monster_kills_within_30_seconds_of_spawn: u32,
    epic_monster_steals: u32,
    epic_monster_stolen_without_smite: u32,
    first_turret_killed: u32,
    first_turret_killed_time: f32,
    flawless_aces: u32,
    full_team_takedown: u32,
    game_length: f32,
    get_takedowns_in_all_lanes_early_jungle_as_laner: u32,
    gold_per_minute: f32,
    had_open_nexus: u32,
    immobilize_and_kill_with_ally: u32,
    initial_buff_count: u32,
    initial_crab_count: u32,
    jungle_cs_before_10_minutes: f32,
    jungler_takedowns_near_damaged_epic_monster: u32,
    kda: f32,
    kill_after_hidden_with_ally: u32,
    killed_champ_took_full_team_damage_survived: u32,
    killing_sprees: u32,
    kill_participation: f32,
    kills_near_enemy_turret: u32,
    kills_on_other_lanes_early_jungle_as_laner: u32,
    kills_on_recently_healed_by_aram_pack: u32,
    kills_under_own_turret: u32,
    kills_with_help_from_epic_monster: u32,
    knock_enemy_into_team_and_kill: u32,
    k_turrets_destroyed_before_plates_fall: u32,
    land_skill_shots_early_game: u32,
    lane_minions_first_10_minutes: u32,
    lost_an_inhibitor: u32,
    max_kill_deficit: u32,
    mejais_full_stack_in_time: u32,
    more_enemy_jungle_than_opponent: f32,
    multi_kill_one_spell: u32,
    multikills: u32,
    multikills_after_aggressive_flash: u32,
    multi_turret_rift_heral_count: u32,
    outer_turret_executes_before_10_minutes: u32,
    outnumbered_kills: u32,
    outnumbered_nexus_kill: u32,
    perfect_dragon_souls_taken: u32,
    perfect_game: u32,
    pick_kill_with_ally: u32,
    poro_explosions: u32,
    quick_cleanse: u32,
    quick_first_turret: u32,
    quick_solo_kills: u32,
    rift_herald_takedowns: u32,
    save_ally_from_death: u32,
    scuttle_crab_kills: u32,
    shortest_time_to_ace_from_first_takedown: f32,
    skillshots_dodged: u32,
    skillshots_hit: u32,
    snowballs_hit: u32,
    solo_baron_kills: u32,
    swarm_defeat_aatrox: u32,
    swarm_defeat_briar: u32,
    swarm_defeat_mini_bosses: u32,
    swarm_evolve_weapon: u32,
    swarm_have_3_passives: u32,
    swarm_kill_enemy: u32,
    swarm_pickup_gold: f32,
    swarm_reach_level_50: u32,
    swarm_survive_15_min: u32,
    swarm_win_with_5_evolved_weapons: u32,
    solo_kills: u32,
    stealth_wards_placed: u32,
    survived_single_digit_hp_count: u32,
    survived_three_immobilizes_in_fight: u32,
    takedown_on_first_turret: u32,
    takedowns: u32,
    takedowns_after_gaining_level_advantage: u32,
    takedowns_before_jungle_minion_spawn: u32,
    takedowns_first_x_minutes: u32,
    takedowns_in_alcove: u32,
    takedowns_in_enemy_fountain: u32,
    team_baron_kills: u32,
    team_damage_percentage: f32,
    team_elder_dragon_kills: u32,
    team_rift_herald_kills: u32,
    took_large_damage_survived: u32,
    turrets_plates_taken: u32,
    turrets_taken_with_rift_herald: u32,
    turret_takedowns: u32,
    twenty_minions_in_3_seconds_count: u32,
    two_wards_one_sweeper_count: u32,
    unseen_recalls: u32,
    vision_score_per_minute: f32,
    wards_guarded: u32,
    ward_takedowns: u32,
    ward_takedowns_before_20m: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Missions {
    player_score_0: u32,
    player_score_1: u32,
    player_score_2: u32,
    player_score_3: u32,
    player_score_4: u32,
    player_score_5: u32,
    player_score_6: u32,
    player_score_7: u32,
    player_score_8: u32,
    player_score_9: u32,
    player_score_10: u32,
    player_score_11: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    stat_perks: PerkStats,
    styles: Vec<PerkStyle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerkStats {
    defense: u32,
    flex: u32,
    offense: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerkStyle {
    description: String,
    selections: Vec<PerkStyleSelection>,
    style: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerkStyleSelection {
    perk: u32,
    var1: u32,
    var2: u32,
    var3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    bans: Vec<Ban>,
    objectives: Objectives,
    team_id: u32,
    win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    champion_id: u32,
    pick_turn: u32,
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
pub struct Objective {
    first: bool,
    kills: u32,
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
        queue: Option<u32>,
        queue_type: Option<QueueType>,
        start: Option<u32>,
        count: Option<u32>,
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
    participant_id: u32,
    puuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FramesTimeline {
    events: Vec<EventsTimeline>,
    participant_frames: ParticipantFrames,
    timestamp: u32,
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
    current_gold: u32,
    damage_stats: DamageStats,
    gold_per_second: u32,
    jungle_minions_killed: u32,
    level: u32,
    minions_killed: u32,
    participant_id: u32,
    position: Position,
    time_enemy_spent_controlled: u32,
    total_gold: u32,
    xp: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    ability_haste: u32,
    ability_power: u32,
    armor: u32,
    armor_pen: u32,
    armor_pen_percent: u32,
    attack_damage: u32,
    attack_speed: u32,
    bonus_armor_pen_percent: u32,
    bonus_magic_pen_percent: u32,
    cc_reduction: u32,
    cooldown_reduction: u32,
    health: u32,
    health_max: u32,
    health_regen: u32,
    lifesteal: u32,
    magic_pen: u32,
    magic_pen_percent: u32,
    magic_resist: u32,
    movement_speed: u32,
    omnivamp: u32,
    physical_vammp: u32,
    power: u32,
    power_max: u32,
    power_regen: u32,
    spell_vamp: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    magic_damage_done: u32,
    magic_damage_done_to_champions: u32,
    magic_damage_taken: u32,
    physical_damage_done: u32,
    physical_damage_done_to_champions: u32,
    physical_damage_taken: u32,
    total_damage_done: u32,
    total_damage_done_to_champions: u32,
    total_damage_taken: u32,
    true_damage_done: u32,
    true_damage_done_to_champions: u32,
    true_damage_taken: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    x: u32,
    y: u32,
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
