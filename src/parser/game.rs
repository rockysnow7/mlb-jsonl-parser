use strum::EnumIter;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, EnumIter, Deserialize, Clone, Copy)]
pub enum PlayType {
    Groundout,
    #[serde(rename = "Bunt Groundout")]
    BuntGroundout,
    Strikeout,
    Lineout,
    #[serde(rename = "Bunt Lineout")]
    BuntLineout,
    Flyout,
    #[serde(rename = "Pop Out")]
    PopOut,
    #[serde(rename = "Bunt Pop Out")]
    BuntPopOut,
    Forceout,
    #[serde(rename = "Fielders Choice Out")]
    FieldersChoiceOut,
    #[serde(rename = "Double Play")]
    DoublePlay,
    #[serde(rename = "Triple Play")]
    TriplePlay,
    #[serde(rename = "Runner Double Play")]
    RunnerDoublePlay,
    #[serde(rename = "Runner Triple Play")]
    RunnerTriplePlay,
    #[serde(rename = "Grounded Into Double Play")]
    GroundedIntoDoublePlay,
    #[serde(rename = "Strikeout Double Play")]
    StrikeoutDoublePlay,
    Pickoff,
    #[serde(rename = "Pickoff Error")]
    PickoffError,
    #[serde(rename = "Caught Stealing")]
    CaughtStealing,
    #[serde(rename = "Pickoff Caught Stealing")]
    PickoffCaughtStealing,
    #[serde(rename = "Wild Pitch")]
    WildPitch,
    #[serde(rename = "Runner Out")]
    RunnerOut,
    #[serde(rename = "Field Out")]
    FieldOut,
    #[serde(rename = "Batter Out")]
    BatterOut,
    Balk,
    #[serde(rename = "Passed Ball")]
    PassedBall,
    Error,
    Single,
    Double,
    Triple,
    #[serde(rename = "Home Run")]
    HomeRun,
    Walk,
    #[serde(rename = "Intent Walk")]
    IntentWalk,
    #[serde(rename = "Hit By Pitch")]
    HitByPitch,
    #[serde(rename = "Fielders Choice")]
    FieldersChoice,
    #[serde(rename = "Catcher Interference")]
    CatcherInterference,
    #[serde(rename = "Stolen Base")]
    StolenBase,
    #[serde(rename = "Sac Fly")]
    SacFly,
    #[serde(rename = "Sac Fly Double Play")]
    SacFlyDoublePlay,
    #[serde(rename = "Sac Bunt")]
    SacBunt,
    #[serde(rename = "Sac Bunt Double Play")]
    SacBuntDoublePlay,
    #[serde(rename = "Field Error")]
    FieldError,
    #[serde(rename = "Game Advisory")]
    GameAdvisory,
    Ejection,
}

impl ToString for PlayType {
    fn to_string(&self) -> String {
        match self {
            PlayType::Groundout => "Groundout",
            PlayType::BuntGroundout => "Bunt Groundout",
            PlayType::Strikeout => "Strikeout",
            PlayType::Lineout => "Lineout",
            PlayType::BuntLineout => "Bunt Lineout",
            PlayType::Flyout => "Flyout",
            PlayType::PopOut => "Pop Out",
            PlayType::BuntPopOut => "Bunt Pop Out",
            PlayType::Forceout => "Forceout",
            PlayType::FieldersChoiceOut => "Fielders Choice Out",
            PlayType::DoublePlay => "Double Play",
            PlayType::TriplePlay => "Triple Play",
            PlayType::RunnerDoublePlay => "Runner Double Play",
            PlayType::RunnerTriplePlay => "Runner Triple Play",
            PlayType::GroundedIntoDoublePlay => "Grounded Into Double Play",
            PlayType::StrikeoutDoublePlay => "Strikeout Double Play",
            PlayType::Pickoff => "Pickoff",
            PlayType::PickoffError => "Pickoff Error",
            PlayType::CaughtStealing => "Caught Stealing",
            PlayType::PickoffCaughtStealing => "Pickoff Caught Stealing",
            PlayType::WildPitch => "Wild Pitch",
            PlayType::RunnerOut => "Runner Out",
            PlayType::FieldOut => "Field Out",
            PlayType::BatterOut => "Batter Out",
            PlayType::Balk => "Balk",
            PlayType::PassedBall => "Passed Ball",
            PlayType::Error => "Error",
            PlayType::Single => "Single",
            PlayType::Double => "Double",
            PlayType::Triple => "Triple",
            PlayType::HomeRun => "Home Run",
            PlayType::Walk => "Walk",
            PlayType::IntentWalk => "Intent Walk",
            PlayType::HitByPitch => "Hit By Pitch",
            PlayType::FieldersChoice => "Fielders Choice",
            PlayType::CatcherInterference => "Catcher Interference",
            PlayType::StolenBase => "Stolen Base",
            PlayType::SacFly => "Sac Fly",
            PlayType::SacFlyDoublePlay => "Sac Fly Double Play",
            PlayType::SacBunt => "Sac Bunt",
            PlayType::SacBuntDoublePlay => "Sac Bunt Double Play",
            PlayType::FieldError => "Field Error",
            PlayType::GameAdvisory => "Game Advisory",
            PlayType::Ejection => "Ejection",
        }.to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "weather")]
pub struct Weather {
    pub condition: String,
    pub temperature: u32,
    pub wind_speed: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "player")]
pub struct Player {
    pub position: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u32,
    pub players: Vec<Player>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Context {
    pub game_pk: u32,
    pub date: String,
    pub venue_name: String,
    pub weather: Weather,
    pub home_team: Team,
    pub away_team: Team,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Inning {
    pub number: u32,
    pub top: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Movement {
    pub runner: String,
    pub start_base: String,
    pub end_base: String,
    pub is_out: bool,
}

#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Play {
    Groundout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Bunt Groundout")]
    BuntGroundout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    Strikeout {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    Lineout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Bunt Lineout")]
    BuntLineout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    Flyout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Pop Out")]
    PopOut {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Bunt Pop Out")]
    BuntPopOut {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    Forceout {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Fielders Choice Out")]
    FieldersChoiceOut {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Double Play")]
    DoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Triple Play")]
    TriplePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Runner Double Play")]
    RunnerDoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Runner Triple Play")]
    RunnerTriplePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Grounded Into Double Play")]
    GroundedIntoDoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Strikeout Double Play")]
    StrikeoutDoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    Pickoff {
        inning: Inning,
        base: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Pickoff Error")]
    PickoffError {
        inning: Inning,
        base: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Caught Stealing")]
    CaughtStealing {
        inning: Inning,
        base: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Pickoff Caught Stealing")]
    PickoffCaughtStealing {
        inning: Inning,
        base: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Wild Pitch")]
    WildPitch {
        inning: Inning,
        pitcher: String,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Runner Out")]
    RunnerOut {
        inning: Inning,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Field Out")]
    FieldOut {
        inning: Inning,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Batter Out")]
    BatterOut {
        inning: Inning,
        batter: String,
        catcher: String,
        movements: Vec<Movement>,
    },
    Balk {
        inning: Inning,
        pitcher: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Passed Ball")]
    PassedBall {
        inning: Inning,
        pitcher: String,
        catcher: String,
        movements: Vec<Movement>,
    },
    Error {
        inning: Inning,
        pitcher: String,
        catcher: String,
        movements: Vec<Movement>,
    },
    Single {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    Double {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    Triple {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Home Run")]
    HomeRun {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    Walk {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Intent Walk")]
    IntentWalk {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Hit By Pitch")]
    HitByPitch {
        inning: Inning,
        batter: String,
        pitcher: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Fielders Choice")]
    FieldersChoice {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Catcher Interference")]
    CatcherInterference {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Stolen Base")]
    StolenBase {
        inning: Inning,
        base: String,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Sac Fly")]
    SacFly {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Sac Fly Double Play")]
    SacFlyDoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Sac Bunt")]
    SacBunt {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Sac Bunt Double Play")]
    SacBuntDoublePlay {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        runner: String,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Field Error")]
    FieldError {
        inning: Inning,
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        movements: Vec<Movement>,
    },
    #[serde(rename = "Game Advisory")]
    GameAdvisory {
        inning: Inning,
    },
    Ejection {
        inning: Inning,
        movements: Vec<Movement>,
    },
}

pub struct PlayBuilder {
    pub inning: Option<Inning>,
    pub play_type: Option<PlayType>,
    pub base: Option<String>,
    pub batter: Option<String>,
    pub pitcher: Option<String>,
    pub catcher: Option<String>,
    pub fielders: Option<Vec<String>>,
    pub runner: Option<String>,
    pub scoring_runner: Option<String>,
    pub movements: Option<Vec<Movement>>,
}

impl PlayBuilder {
    pub fn new() -> Self {
        Self {
            inning: None,
            play_type: None,
            base: None,
            batter: None,
            pitcher: None,
            catcher: None,
            fielders: None,
            runner: None,
            scoring_runner: None,
            movements: None,
        }
    }

    pub fn set_inning(&mut self, inning: Inning) {
        self.inning = Some(inning);
    }

    pub fn set_play_type(&mut self, play_type: PlayType) {
        self.play_type = Some(play_type);
    }

    pub fn set_base(&mut self, base: String) {
        self.base = Some(base);
    }

    pub fn set_batter(&mut self, batter: String) {
        self.batter = Some(batter);
    }

    pub fn set_pitcher(&mut self, pitcher: String) {
        self.pitcher = Some(pitcher);
    }

    pub fn set_catcher(&mut self, catcher: String) {
        self.catcher = Some(catcher);
    }

    pub fn set_fielders(&mut self, fielders: Vec<String>) {
        self.fielders = Some(fielders);
    }

    pub fn set_runner(&mut self, runner: String) {
        self.runner = Some(runner);
    }

    pub fn set_scoring_runner(&mut self, scoring_runner: String) {
        self.scoring_runner = Some(scoring_runner);
    }

    pub fn set_movements(&mut self, movements: Vec<Movement>) {
        self.movements = Some(movements);
    }

    pub fn build(&self) -> Play {
        match &self.play_type {
            Some(play_type) => match play_type {
                PlayType::Groundout => Play::Groundout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Groundout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Groundout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Groundout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a Groundout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Groundout").clone(),
                },
                PlayType::BuntGroundout => Play::BuntGroundout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a BuntGroundout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a BuntGroundout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a BuntGroundout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a BuntGroundout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a BuntGroundout").clone(),
                },
                PlayType::Strikeout => Play::Strikeout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Strikeout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Strikeout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Strikeout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Strikeout").clone(),
                },
                PlayType::Lineout => Play::Lineout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Lineout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Lineout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Lineout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a Lineout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Lineout").clone(),
                },
                PlayType::BuntLineout => Play::BuntLineout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a BuntLineout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a BuntLineout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a BuntLineout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a BuntLineout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a BuntLineout").clone(),
                },
                PlayType::Flyout => Play::Flyout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Flyout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Flyout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Flyout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a Flyout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Flyout").clone(),
                },
                PlayType::PopOut => Play::PopOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a PopOut"),
                    batter: self.batter.as_ref().expect("batter should be set for building a PopOut").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a PopOut").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a PopOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a PopOut").clone(),
                },
                PlayType::BuntPopOut => Play::BuntPopOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a BuntPopOut"),
                    batter: self.batter.as_ref().expect("batter should be set for building a BuntPopOut").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a BuntPopOut").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a BuntPopOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a BuntPopOut").clone(),
                },
                PlayType::Forceout => Play::Forceout {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Forceout"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Forceout").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Forceout").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a Forceout").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Forceout").clone(),
                },
                PlayType::FieldersChoiceOut => Play::FieldersChoiceOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a FieldersChoiceOut"),
                    batter: self.batter.as_ref().expect("batter should be set for building a FieldersChoiceOut").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a FieldersChoiceOut").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a FieldersChoiceOut").clone(),
                    scoring_runner: self.scoring_runner.as_ref().expect("scoring_runner should be set for building a FieldersChoiceOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a FieldersChoiceOut").clone(),
                },
                PlayType::DoublePlay => Play::DoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a DoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a DoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a DoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a DoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a DoublePlay").clone(),
                },
                PlayType::TriplePlay => Play::TriplePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a TriplePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a TriplePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a TriplePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a TriplePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a TriplePlay").clone(),
                },
                PlayType::RunnerDoublePlay => Play::RunnerDoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a RunnerDoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a RunnerDoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a RunnerDoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a RunnerDoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a RunnerDoublePlay").clone(),
                },
                PlayType::RunnerTriplePlay => Play::RunnerTriplePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a RunnerTriplePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a RunnerTriplePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a RunnerTriplePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a RunnerTriplePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a RunnerTriplePlay").clone(),
                },
                PlayType::GroundedIntoDoublePlay => Play::GroundedIntoDoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a GroundedIntoDoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a GroundedIntoDoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a GroundedIntoDoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a GroundedIntoDoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a GroundedIntoDoublePlay").clone(),
                },
                PlayType::StrikeoutDoublePlay => Play::StrikeoutDoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a StrikeoutDoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a StrikeoutDoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a StrikeoutDoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a StrikeoutDoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a StrikeoutDoublePlay").clone(),
                },
                PlayType::Pickoff => Play::Pickoff {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Pickoff"),
                    base: self.base.as_ref().expect("base should be set for building a Pickoff").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a Pickoff").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a Pickoff").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Pickoff").clone(),
                },
                PlayType::PickoffError => Play::PickoffError {
                    inning: *self.inning.as_ref().expect("inning should be set for building a PickoffError"),
                    base: self.base.as_ref().expect("base should be set for building a PickoffError").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a PickoffError").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a PickoffError").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a PickoffError").clone(),
                },
                PlayType::CaughtStealing => Play::CaughtStealing {
                    inning: *self.inning.as_ref().expect("inning should be set for building a CaughtStealing"),
                    base: self.base.as_ref().expect("base should be set for building a CaughtStealing").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a CaughtStealing").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a CaughtStealing").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a CaughtStealing").clone(),
                },
                PlayType::PickoffCaughtStealing => Play::PickoffCaughtStealing {
                    inning: *self.inning.as_ref().expect("inning should be set for building a PickoffCaughtStealing"),
                    base: self.base.as_ref().expect("base should be set for building a PickoffCaughtStealing").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a PickoffCaughtStealing").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a PickoffCaughtStealing").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a PickoffCaughtStealing").clone(),
                },
                PlayType::WildPitch => Play::WildPitch {
                    inning: *self.inning.as_ref().expect("inning should be set for building a WildPitch"),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a WildPitch").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a WildPitch").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a WildPitch").clone(),
                },
                PlayType::RunnerOut => Play::RunnerOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a RunnerOut"),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a RunnerOut").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a RunnerOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a RunnerOut").clone(),
                },
                PlayType::FieldOut => Play::FieldOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a FieldOut"),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a FieldOut").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a FieldOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a FieldOut").clone(),
                },
                PlayType::BatterOut => Play::BatterOut {
                    inning: *self.inning.as_ref().expect("inning should be set for building a BatterOut"),
                    batter: self.batter.as_ref().expect("batter should be set for building a BatterOut").clone(),
                    catcher: self.catcher.as_ref().expect("catcher should be set for building a BatterOut").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a BatterOut").clone(),
                },
                PlayType::Balk => Play::Balk {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Balk"),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Balk").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Balk").clone(),
                },
                PlayType::PassedBall => Play::PassedBall {
                    inning: *self.inning.as_ref().expect("inning should be set for building a PassedBall"),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a PassedBall").clone(),
                    catcher: self.catcher.as_ref().expect("catcher should be set for building a PassedBall").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a PassedBall").clone(),
                },
                PlayType::Error => Play::Error {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Error"),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Error").clone(),
                    catcher: self.catcher.as_ref().expect("catcher should be set for building a Error").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Error").clone(),
                },
                PlayType::Single => Play::Single {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Single"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Single").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Single").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Single").clone(),
                },
                PlayType::Double => Play::Double {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Double"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Double").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Double").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Double").clone(),
                },
                PlayType::Triple => Play::Triple {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Triple"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Triple").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Triple").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Triple").clone(),
                },
                PlayType::HomeRun => Play::HomeRun {
                    inning: *self.inning.as_ref().expect("inning should be set for building a HomeRun"),
                    batter: self.batter.as_ref().expect("batter should be set for building a HomeRun").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a HomeRun").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a HomeRun").clone(),
                },
                PlayType::Walk => Play::Walk {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Walk"),
                    batter: self.batter.as_ref().expect("batter should be set for building a Walk").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a Walk").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a Walk").clone(),
                },
                PlayType::IntentWalk => Play::IntentWalk {
                    inning: *self.inning.as_ref().expect("inning should be set for building a IntentWalk"),
                    batter: self.batter.as_ref().expect("batter should be set for building a IntentWalk").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a IntentWalk").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a IntentWalk").clone(),
                },
                PlayType::HitByPitch => Play::HitByPitch {
                    inning: *self.inning.as_ref().expect("inning should be set for building a HitByPitch"),
                    batter: self.batter.as_ref().expect("batter should be set for building a HitByPitch").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a HitByPitch").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a HitByPitch").clone(),
                },
                PlayType::FieldersChoice => Play::FieldersChoice {
                    inning: *self.inning.as_ref().expect("inning should be set for building a FieldersChoice"),
                    batter: self.batter.as_ref().expect("batter should be set for building a FieldersChoice").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a FieldersChoice").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a FieldersChoice").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a FieldersChoice").clone(),
                },
                PlayType::CatcherInterference => Play::CatcherInterference {
                    inning: *self.inning.as_ref().expect("inning should be set for building a CatcherInterference"),
                    batter: self.batter.as_ref().expect("batter should be set for building a CatcherInterference").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a CatcherInterference").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a CatcherInterference").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a CatcherInterference").clone(),
                },
                PlayType::StolenBase => Play::StolenBase {
                    inning: *self.inning.as_ref().expect("inning should be set for building a StolenBase"),
                    base: self.base.as_ref().expect("base should be set for building a StolenBase").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a StolenBase").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a StolenBase").clone(),
                },
                PlayType::SacFly => Play::SacFly {
                    inning: *self.inning.as_ref().expect("inning should be set for building a SacFly"),
                    batter: self.batter.as_ref().expect("batter should be set for building a SacFly").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a SacFly").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a SacFly").clone(),
                    scoring_runner: self.scoring_runner.as_ref().expect("scoring_runner should be set for building a SacFly").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a SacFly").clone(),
                },
                PlayType::SacFlyDoublePlay => Play::SacFlyDoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a SacFlyDoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a SacFlyDoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a SacFlyDoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a SacFlyDoublePlay").clone(),
                    scoring_runner: self.scoring_runner.as_ref().expect("scoring_runner should be set for building a SacFlyDoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a SacFlyDoublePlay").clone(),
                },
                PlayType::SacBunt => Play::SacBunt {
                    inning: *self.inning.as_ref().expect("inning should be set for building a SacBunt"),
                    batter: self.batter.as_ref().expect("batter should be set for building a SacBunt").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a SacBunt").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a SacBunt").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a SacBunt").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a SacBunt").clone(),
                },
                PlayType::SacBuntDoublePlay => Play::SacBuntDoublePlay {
                    inning: *self.inning.as_ref().expect("inning should be set for building a SacBuntDoublePlay"),
                    batter: self.batter.as_ref().expect("batter should be set for building a SacBuntDoublePlay").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a SacBuntDoublePlay").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a SacBuntDoublePlay").clone(),
                    runner: self.runner.as_ref().expect("runner should be set for building a SacBuntDoublePlay").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a SacBuntDoublePlay").clone(),
                },
                PlayType::FieldError => Play::FieldError {
                    inning: *self.inning.as_ref().expect("inning should be set for building a FieldError"),
                    batter: self.batter.as_ref().expect("batter should be set for building a FieldError").clone(),
                    pitcher: self.pitcher.as_ref().expect("pitcher should be set for building a FieldError").clone(),
                    fielders: self.fielders.as_ref().expect("fielders should be set for building a FieldError").clone(),
                    movements: self.movements.as_ref().expect("movements should be set for building a FieldError").clone(),
                },
                PlayType::GameAdvisory => Play::GameAdvisory {
                    inning: *self.inning.as_ref().expect("inning should be set for building a GameAdvisory"),
                },
                PlayType::Ejection => Play::Ejection {
                    inning: *self.inning.as_ref().expect("inning should be set for building a Ejection"),
                    movements: self.movements.as_ref().expect("movements should be set for building a Ejection").clone(),
                },
            }
            None => panic!("Play type not set"),
        }
    }
}

pub struct GameBuilder {
    pub context: Option<Context>,
    pub plays: Vec<Play>,
    pub play_builder: PlayBuilder,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            context: None,
            plays: Vec::new(),
            play_builder: PlayBuilder::new(),
        }
    }

    pub fn add_context(&mut self, context: Context) {
        self.context = Some(context);
    }

    pub fn clear_play_builder(&mut self) {
        self.play_builder = PlayBuilder::new();
    }

    pub fn add_play(&mut self, play: Play) {
        self.plays.push(play);
    }

    pub fn home_team_player_names(&self) -> Option<Vec<String>> {
        if let Some(context) = &self.context {
            Some(context.home_team.players.iter().map(|p| p.name.clone()).collect::<Vec<_>>())
        } else {
            None
        }
    }

    pub fn away_team_player_names(&self) -> Option<Vec<String>> {
        if let Some(context) = &self.context {
            Some(context.away_team.players.iter().map(|p| p.name.clone()).collect::<Vec<_>>())
        } else {
            None
        }
    }
}
