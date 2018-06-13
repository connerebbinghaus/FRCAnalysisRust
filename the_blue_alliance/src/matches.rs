use team::Alliances;
use ::{TBA, Result};

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompLevel {
    #[serde(rename = "qm")]
    QualificationMatch,
    #[serde(rename = "ef")]
    EighthFinal,
    #[serde(rename = "qf")]
    QuarterFinal,
    #[serde(rename = "sf")]
    SemiFinal,
    #[serde(rename = "f")]
    Final
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2015Alliance {
    pub auto_points: i32,
    pub teleop_points: i32,
    pub container_points: i32,
    pub tote_points: i32,
    pub litter_points: i32,
    pub foul_points: i32,
    pub adjust_points: i32,
    pub total_points: i32,
    pub foul_count: i32,
    pub tote_count_far: i32,
    pub tote_count_near: i32,
    pub tote_set: i32,
    pub tote_stack: i32,
    pub container_count_level1: i32,
    pub container_count_level2: i32,
    pub container_count_level3: i32,
    pub container_count_level4: i32,
    pub container_count_level5: i32,
    pub container_count_level6: i32,
    pub container_set: bool,
    pub litter_count_container: i32,
    pub litter_count_landfill: i32,
    pub litter_count_unprocessed: i32,
    pub robot_set: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Coopertition {
    None, Unknown, Stack
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2015 {
    pub blue: ScoreBreakdown2015Alliance,
    pub red: ScoreBreakdown2015Alliance,
    pub coopertition: Coopertition,
    pub coopertition_points: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Auto2016 {
    Crossed, Reached, None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2016Alliance {
    #[serde(rename = "autoPoints")]
    pub auto_points: i32,
    #[serde(rename = "teleopPoints")]
    pub teleop_points: i32,
    #[serde(rename = "breachPoints")]
    pub breach_points: i32,
    #[serde(rename = "foulPoints")]
    pub foul_points: i32,
    #[serde(rename = "capturePoints")]
    pub capture_points: i32,
    #[serde(rename = "adjustPoints")]
    pub adjust_points: i32,
    #[serde(rename = "totalPoints")]
    pub total_points: i32,
    #[serde(rename = "robot1Auto")]
    pub robot1_auto: Auto2016,
    #[serde(rename = "robot2Auto")]
    pub robot2_auto: Auto2016,
    #[serde(rename = "robot3Auto")]
    pub robot3_auto: Auto2016,
    #[serde(rename = "autoReachPoints")]
    pub auto_reach_points: i32,
    #[serde(rename = "autoCrossingPoints")]
    pub auto_crossing_points: i32,
    #[serde(rename = "autoBouldersLow")]
    pub auto_boulders_low: i32,
    #[serde(rename = "autoBouldersHigh")]
    pub auto_boulders_high: i32,
    #[serde(rename = "autoBoulderPoints")]
    pub auto_boulder_points: i32,
    #[serde(rename = "teleopCrossingPoints")]
    pub teleop_crossing_points: i32,
    #[serde(rename = "teleopBouldersLow")]
    pub teleop_boulders_low: i32,
    #[serde(rename = "teleopBouldersHigh")]
    pub teleop_bouldershigh: i32,
    #[serde(rename = "teleopBoulderPoints")]
    pub teleop_boulder_points: i32,
    #[serde(rename = "teleopDefensesBreached")]
    pub teleop_defenses_breached: i32,
    #[serde(rename = "teleopChallengePoints")]
    pub teleop_challenge_points: i32,
    #[serde(rename = "teleopScalePoints")]
    pub teleop_scale_points: i32,
    #[serde(rename = "teleopTowerCaptured")]
    pub teleop_tower_captured: i32,
    #[serde(rename = "towerFaceA")]
    pub tower_face_a: String,
    #[serde(rename = "towerFaceB")]
    pub tower_face_b: String,
    #[serde(rename = "towerFaceC")]
    pub tower_face_c: String,
    #[serde(rename = "towerEndStrength")]
    pub tower_end_strength: i32,
    #[serde(rename = "techFoulCount")]
    pub tech_foul_count: i32,
    #[serde(rename = "foulCount")]
    pub foul_count: i32,
    pub position2: String,
    pub position3: String,
    pub position4: String,
    pub position5: String,
    pub position1crossings: i32,
    pub position2crossings: i32,
    pub position3crossings: i32,
    pub position4crossings: i32,
    pub position5crossings: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2016 {
    pub blue: ScoreBreakdown2016Alliance,
    pub red: ScoreBreakdown2016Alliance
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Auto2017 {
    Unknown, Mobility, None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2017Alliance {
    #[serde(rename = "autoPoints")]
    pub auto_points: i32,
    #[serde(rename = "teleopPoints")]
    pub teleop_points: i32,
    #[serde(rename = "foulPoints")]
    pub foul_points: i32,
    #[serde(rename = "adjustPoints")]
    pub adjust_points: i32,
    #[serde(rename = "totalPoints")]
    pub total_points: i32,
    #[serde(rename = "robot1Auto")]
    pub robot1_auto: Auto2017,
    #[serde(rename = "robot2Auto")]
    pub robot2_auto: Auto2017,
    #[serde(rename = "robot3Auto")]
    pub robot3_auto: Auto2017,
    #[serde(rename = "rotor1Auto")]
    pub rotor1_auto: bool,
    #[serde(rename = "rotor2Auto")]
    pub rotor2_auto: bool,
    #[serde(rename = "autoFuelLow")]
    pub auto_fuel_low: i32,
    #[serde(rename = "autoFuelHigh")]
    pub auto_fuel_high: i32,
    #[serde(rename = "autoMobilityPoints")]
    pub auto_mobility_points: i32,
    #[serde(rename = "autoRotorPoints")]
    pub auto_rotor_points: i32,
    #[serde(rename = "autoFuelPoints")]
    pub auto_fuel_points: i32,
    #[serde(rename = "teleopFuelPoints")]
    pub teleop_fuel_points: i32,
    #[serde(rename = "teleopFuelLow")]
    pub teleop_fuel_low: i32,
    #[serde(rename = "teleopFuelHigh")]
    pub teleop_fuel_high: i32,
    #[serde(rename = "teleopRotorPoints")]
    pub teleop_rotor_points: i32,
    #[serde(rename = "kPaRankingPointAchieved")]
    pub kpa_ranking_point_achieved: bool,
    #[serde(rename = "teleopTakeoffPoints")]
    pub teleop_takeoff_points: i32,
    #[serde(rename = "kPaBonusPoints")]
    pub kpa_bonus_points: i32,
    #[serde(rename = "rotorBonusPoints")]
    pub rotor_bonus_points: i32,
    #[serde(rename = "rotor1Engaged")]
    pub rotor1_engaged: bool,
    #[serde(rename = "rotor2Engaged")]
    pub rotor2_engaged: bool,
    #[serde(rename = "rotor3Engaged")]
    pub rotor3_engaged: bool,
    #[serde(rename = "rotor4Engaged")]
    pub rotor4_engaged: bool,
    #[serde(rename = "rotorRankingPointAchieved")]
    pub rotor_ranking_point_achieved: bool,
    #[serde(rename = "techFoulCount")]
    pub tech_foul_count: i32,
    #[serde(rename = "foulCount")]
    pub foul_count: i32,
    #[serde(rename = "touchpadNear")]
    pub touchpad_near: i32,
    #[serde(rename = "touchpadMiddle")]
    pub touchpad_middle: i32,
    #[serde(rename = "touchpadFar")]
    pub touchpad_far: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2017 {
    pub blue: ScoreBreakdown2017Alliance,
    pub red: ScoreBreakdown2017Alliance
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2018Alliance {
    #[serde(rename = "adjustPoints")]
    pub adjust_points: i32,
    #[serde(rename = "autoOwnershipPoints")]
    pub auto_ownership_points: i32,
    #[serde(rename = "autoPoints")]
    pub auto_points: i32,
    #[serde(rename = "autoQuestRankingPoint")]
    pub auto_quest_ranking_point: bool,
    #[serde(rename = "autoRobot1")]
    pub auto_robot1: String, // TODO: Enum, need to look at FMS API Docs.
    #[serde(rename = "autoRobot2")]
    pub auto_robot2: String, // TODO
    #[serde(rename = "autoRobot3")]
    pub auto_robot3: String, // TODO
    #[serde(rename = "autoRunPoints")]
    pub auto_run_points: i32,
    #[serde(rename = "autoScaleOwnershipSec")]
    pub auto_scale_ownership_sec: i32,
    #[serde(rename = "autoSwitchAtZero")]
    pub auto_switch_at_zero: bool,
    #[serde(rename = "autoSwitchOwnershipSec")]
    pub auto_switch_ownership_sec: i32,
    #[serde(rename = "endgamePoints")]
    pub endgame_points: i32,
    #[serde(rename = "endgameRobot1")]
    pub endgame_robot1: String, // TODO
    #[serde(rename = "endgameRobot2")]
    pub endgame_robot2: String, // TODO
    #[serde(rename = "endgameRobot3")]
    pub endgame_robot3: String, // TODO
    #[serde(rename = "faceTheBossRankingPoint")]
    pub face_the_boss_ranking_point: bool,
    #[serde(rename = "foulCount")]
    pub foul_count: i32,
    #[serde(rename = "foulPoints")]
    pub foul_points: i32,
    pub rp: i32,
    #[serde(rename = "techFoulCount")]
    pub tech_foul_count: i32,
    #[serde(rename = "teleopOwnershipPoints")]
    pub teleop_ownership_points: i32,
    #[serde(rename = "teleopPoints")]
    pub teleop_points: i32,
    #[serde(rename = "teleopScaleBoostSec")]
    pub teleop_scale_boost_sec: i32,
    #[serde(rename = "teleopScaleForceSec")]
    pub teleop_scale_force_sec: i32,
    #[serde(rename = "teleopScaleOwnershipSec")]
    pub teleop_scale_ownership_sec: i32,
    #[serde(rename = "teleopSwitchBoostSec")]
    pub teleop_switch_boost_sec: i32,
    #[serde(rename = "teleopSwitchForceSec")]
    pub teleop_switch_force_sec: i32,
    #[serde(rename = "teleopSwitchOwnershipSec")]
    pub teleop_switch_ownership_sec: i32,
    #[serde(rename = "totalPoints")]
    pub total_points: i32,
    #[serde(rename = "vaultBoostPlayed")]
    pub vault_boost_played: i32,
    #[serde(rename = "vaultBoostTotal")]
    pub vault_boost_total: i32,
    #[serde(rename = "vaultForcePlayed")]
    pub vault_force_played: i32,
    #[serde(rename = "vaultForceTotal")]
    pub vault_force_total: i32,
    #[serde(rename = "vaultLevitatePlayed")]
    pub vault_levitate_played: i32,
    #[serde(rename = "vaultLevitateTotal")]
    pub vault_levitate_total: i32,
    #[serde(rename = "vaultPoints")]
    pub vault_points: i32,
    #[serde(rename = "tba_gameData")]
    pub tba_game_data: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown2018 {
    pub blue: ScoreBreakdown2018Alliance,
    pub red: ScoreBreakdown2018Alliance
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ScoreBreakdown {
    Year2015(ScoreBreakdown2015),
    Year2016(ScoreBreakdown2016),
    Year2017(ScoreBreakdown2017),
    Year2018(ScoreBreakdown2018),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoType {
    #[serde(rename = "youtube")]
    YouTube,
    #[serde(rename = "tba")]
    TBA
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub key: String,
    #[serde(rename = "type")]
    pub video_type: VideoType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Winner {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "")]
    None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub key: String,
    pub comp_level: CompLevel,
    pub set_number: i32,
    pub match_number: i32,
    pub alliances: Option<Alliances>,
    pub winning_alliance: Option<Winner>,
    pub event_key: String,
    pub time: Option<u64>,
    pub actual_time: Option<u64>,
    pub predicted_time: Option<u64>,
    pub post_result_time: Option<u64>,
    pub score_breakdown: Option<ScoreBreakdown>,
    pub videos: Option<Vec<Video>>
}

impl Match {
    pub fn from_key(mut tba: TBA, key: String) -> Result<Match> {
        tba.get("/match/".to_owned() + &key)
    }

    pub fn from_event(mut tba: TBA, key: String) -> Result<Vec<Match>> {
        tba.get("/event/".to_owned() + &key + "/matches")
    }

    pub fn team_keys(&self) -> Option<Vec<&String>> {
        if let Some(ref alliances) = self.alliances {
            let mut ret = Vec::new();
            ret.extend(&alliances.blue.team_keys[..]);
            ret.extend(&alliances.red.team_keys[..]);
            return Some(ret);
        }
        return None;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchSimple {
    pub key: String,
    pub comp_level: CompLevel,
    pub set_number: i32,
    pub match_number: i32,
    pub alliances: Option<Alliances>,
    pub winning_alliance: Option<Winner>,
    pub event_key: String,
    pub time: Option<u64>,
    pub predicted_time: Option<u64>,
    pub actual_time: Option<u64>
}

impl MatchSimple {
    pub fn from_key(mut tba: TBA, key: String) -> Result<Match> {
        tba.get("/match/".to_owned() + &key + "/simple")
    }

    pub fn from_event(mut tba: TBA, key: String) -> Result<Vec<Match>> {
        tba.get("/event/".to_owned() + &key + "/matches/simple")
    }
}