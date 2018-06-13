use serde;
use serde_derive;
use serde::ser::Serialize;
use serde::de::Deserialize;
use team::Alliances;
use futures::Future;
use ::TBA;
#[derive(Serialize, Deserialize)]
enum CompLevel {
    qm, ef, qf, sf, f
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2015Alliance {
    auto_points: i32,
    teleop_points: i32,
    container_points: i32,
    tote_points: i32,
    litter_points: i32,
    foul_points: i32,
    adjust_points: i32,
    total_points: i32,
    foul_count: i32,
    tote_count_far: i32,
    tote_count_near: i32,
    tote_set: i32,
    tote_stack: i32,
    container_count_level1: i32,
    container_count_level2: i32,
    container_count_level3: i32,
    container_count_level4: i32,
    container_count_level5: i32,
    container_count_level6: i32,
    container_set: bool,
    litter_count_container: i32,
    litter_count_landfill: i32,
    litter_count_unprocessed: i32,
    robot_set: bool
}

#[derive(Serialize, Deserialize)]
pub enum Coopertition {
    None, Unknown, Stack
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2015 {
    blue: ScoreBreakdown2015Alliance,
    red: ScoreBreakdown2015Alliance,
    coopertition: Coopertition,
    coopertition_points: i32
}

#[derive(Serialize, Deserialize)]
pub enum Auto2016 {
    Crossed, Reached, None
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2016Alliance {
    autoPoints: i32,
    teleopPoints: i32,
    breachPoints: i32,
    foulPoints: i32,
    capturePoints: i32,
    adjustPoints: i32,
    totalPoints: i32,
    robot1Auto: Auto2016,
    robot2Auto: Auto2016,
    robot3Auto: Auto2016,
    autoReachPoints: i32,
    autoCrossingPoints: i32,
    autoBouldersLow: i32,
    autoBouldersHigh: i32,
    autoBoulderPoints: i32,
    teleopCrossingPoints: i32,
    teleopBouldersLow: i32,
    teleopBouldersHigh: i32,
    teleopBoulderPoints: i32,
    teleopDefensesBreached: i32,
    teleopChallengePoints: i32,
    teleopScalePoints: i32,
    teleopTowerCaptured: i32,
    towerFaceA: String,
    towerFaceB: String,
    towerFaceC: String,
    towerEndStrength: i32,
    techFoulCount: i32,
    foulCount: i32,
    position2: String,
    position3: String,
    position4: String,
    position5: String,
    position1crossings: i32,
    position2crossings: i32,
    position3crossings: i32,
    position4crossings: i32,
    position5crossings: i32
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2016 {
    blue: ScoreBreakdown2016Alliance,
    red: ScoreBreakdown2016Alliance
}

#[derive(Serialize, Deserialize)]
enum Auto2017 {
    Unknown, Mobility, None
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2017Alliance {
    autoPoints: i32,
    teleopPoints: i32,
    foulPoints: i32,
    adjustPoints: i32,
    totalPoints: i32,
    robot1Auto: Auto2017,
    robot2Auto: Auto2017,
    robot3Auto: Auto2017,
    rotor1Auto: bool,
    rotor2Auto: bool,
    autoFuelLow: i32,
    autoFuelHigh: i32,
    autoMobilityPoints: i32,
    autoRotorPoints: i32,
    autoFuelPoints: i32,
    teleopFuelPoints: i32,
    teleopFuelLow: i32,
    teleopFuelHigh: i32,
    teleopRotorPoints: i32,
    kPaRankingPointAchieved: bool,
    teleopTakeoffPoints: i32,
    kPaBonusPoints: i32,
    rotorBonusPoints: i32,
    rotor1Engaged: bool,
    rotor2Engaged: bool,
    rotor3Engaged: bool,
    rotor4Engaged: bool,
    rotorRankingPointAchieved: bool,
    techFoulCount: i32,
    foulCount: i32,
    touchpadNear: i32,
    touchpadMiddle: i32,
    touchpadFar: i32
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2017 {
    blue: ScoreBreakdown2017Alliance,
    red: ScoreBreakdown2017Alliance
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2018Alliance {
    adjustPoints: i32,
    autoOwnershipPoints: i32,
    autoPoints: i32,
    autoQuestRankingPoint: bool,
    autoRobot1: String, // TODO: Enum, need to look at FMS API Docs.
    autoRobot2: String, // TODO
    autoRobot3: String, // TODO
    autoRunPoints: i32,
    autoScaleOwnershipSec: i32,
    autoSwitchAtZero: bool,
    autoSwitchOwnershipSec: i32,
    endgamePoints: i32,
    endgameRobot1: String, // TODO
    endgameRobot2: String, // TODO
    endgameRobot3: String, // TODO
    faceTheBossRankingPoint: bool,
    foulCount: i32,
    foulPoints: i32,
    rp: i32,
    techFoulCount: i32,
    teleopOwnershipPoints: i32,
    teleopPoints: i32,
    teleopScaleBoostSec: i32,
    teleopScaleForceSec: i32,
    teleopScaleOwnershipSec: i32,
    teleopSwitchBoostSec: i32,
    teleopSwitchForceSec: i32,
    teleopSwitchOwnershipSec: i32,
    totalPoints: i32,
    vaultBoostPlayed: i32,
    vaultBoostTotal: i32,
    vaultForcePlayed: i32,
    vaultForceTotal: i32,
    vaultLevitatePlayed: i32,
    vaultLevitateTotal: i32,
    vaultPoints: i32,
    tba_gameData: String
}

#[derive(Serialize, Deserialize)]
pub struct ScoreBreakdown2018 {
    blue: ScoreBreakdown2018Alliance,
    red: ScoreBreakdown2018Alliance
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ScoreBreakdown {
    Year2015(ScoreBreakdown2015),
    Year2016(ScoreBreakdown2016),
    Year2017(ScoreBreakdown2017),
    Year2018(ScoreBreakdown2018),
}

#[derive(Serialize, Deserialize)]
enum VideoType {
    youtube, tba
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    key: String,
    #[serde(rename = "type")]
    video_type: VideoType
}

#[derive(Serialize, Deserialize)]
pub enum Winner {
    red, blue,
    #[serde(rename = "")]
    none
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    key: String,
    comp_level: CompLevel,
    set_number: i32,
    match_number: i32,
    alliances: Option<Alliances>,
    winning_alliance: Option<Winner>,
    event_key: String,
    time: Option<u64>,
    actual_time: Option<u64>,
    predicted_time: Option<u64>,
    post_result_time: Option<u64>,
    score_breakdown: Option<ScoreBreakdown>,
    videos: Option<Vec<Video>>
}

impl Match {
    pub fn from_key(tba: TBA, key: String) -> impl Future<Item = Match, Error = ()> {
        tba.get("/match/" + key)
    }

    pub fn from_event(tba: TBA, key: String) -> impl Future<Item = Vec<Match>, Error = ()> {
        tba.get("/event/" + key + "/matches")
    }
}

#[derive(Serialize, Deserialize)]
pub struct MatchSimple {
    key: String,
    comp_level: CompLevel,
    set_number: i32,
    match_number: i32,
    alliances: Option<Alliances>,
    winning_alliance: Option<Winner>,
    event_key: String,
    time: Option<u64>,
    predicted_time: Option<u64>,
    actual_time: Option<u64>
}

impl MatchSimple {
    pub fn from_key(tba: TBA, key: String) -> impl Future<Item = Match, Error = ()> {
        tba.get("/match/" + key + "/simple")
    }

    pub fn from_event(tba: TBA, key: String) -> impl Future<Item = Vec<Match>, Error = ()> {
        tba.get("/event/" + key + "/matches/simple")
    }
}