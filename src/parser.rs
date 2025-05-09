mod game;
mod json_schema;

use game::{Context, Game, GameBuilder, Inning, Movement, Play, PlayType, Base};
use json_schema::{JsonType, KeyValueType, ToRegex};
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};
use serde::Deserialize;
use strum::IntoEnumIterator;

const UNICODE_WORD_CHAR: &str = r"[a-zA-ZÀ-ÖØ-öø-ÿ.'\- ]";

/// The JSON schema for the `weather` section of the `GameSection::Context` section.
fn context_section_weather_json() -> KeyValueType {
    JsonType::key_value(
        "weather",
        JsonType::Object(vec![
            JsonType::key_value(
                "condition",
                JsonType::string_with_regex(r"[a-zA-Z ]+"),
            ),
            JsonType::key_value(
                "temperature",
                JsonType::integer_max_digits(3),
            ),
            JsonType::key_value(
                "wind_speed",
                JsonType::integer_max_digits(3),
            ),
        ]),
    )
}

/// The positions that can be assigned to a player.
const POSITIONS: [&str; 18] = [
    "PITCHER",
    "CATCHER",
    "FIRST_BASE",
    "SECOND_BASE",
    "THIRD_BASE",
    "SHORTSTOP",
    "LEFT_FIELD",
    "CENTER_FIELD",
    "RIGHT_FIELD",
    "DESIGNATED_HITTER",
    "PINCH_HITTER",
    "PINCH_RUNNER",
    "TWO_WAY_PLAYER",
    "OUTFIELD",
    "INFIELD",
    "UTILITY",
    "RELIEF_PITCHER",
    "STARTING_PITCHER",
];

/// The JSON schema for a `player` object in a team section of the `GameSection::Context` section.
fn context_section_team_player_json() -> JsonType {
    let position_regex = format!("({})", POSITIONS.join("|"));

    JsonType::Object(vec![
        JsonType::key_value(
            "position",
            JsonType::string_with_regex(&position_regex),
        ),
        JsonType::key_value(
            "name",
            JsonType::string_with_regex(&format!("{UNICODE_WORD_CHAR}+")),
        ),
    ])
}

/// The JSON schema for a team object in the `GameSection::Context` section.
fn context_section_team_json() -> JsonType {
    JsonType::Object(vec![
        JsonType::key_value(
            "id",
            JsonType::integer_max_digits(3),
        ),
        JsonType::key_value(
            "players",
            JsonType::array(context_section_team_player_json()),
        ),
    ])
}

/// The JSON schema for a `Context` object.
fn context_section_json() -> JsonType {
    let game_pk = JsonType::key_value(
        "game_pk",
        JsonType::integer_max_digits(6),
    );
    let date = JsonType::key_value(
        "date",
        JsonType::string_with_regex(r"\d{4}-\d{2}-\d{2}"),
    );
    let venue_name = JsonType::key_value(
        "venue_name",
        JsonType::string_with_regex(&format!("{UNICODE_WORD_CHAR}+")),
    );

    let home_team = JsonType::key_value(
        "home_team",
        context_section_team_json(),
    );
    let away_team = JsonType::key_value(
        "away_team",
        context_section_team_json(),
    );

    JsonType::object(vec![
        game_pk,
        date,
        venue_name,
        context_section_weather_json(),
        home_team,
        away_team,
    ])
}

/// The type of line being parsed.
#[derive(Debug, PartialEq, Eq)]
enum LineType {
    /// A line that contains the game's context.
    Context,
    /// A line that contains the inning and type of a play.
    PlayIntroduction,
    /// A line that contains the rest of the information for the play introduced in the previous line.
    PlayInformation,
    /// A line that contains the movements for the play introduced in the previous lines.
    PlayMovements,
}

/// The contents of a play introduction.
#[derive(Debug, Deserialize)]
struct PlayIntroduction {
    inning: Inning,
    #[serde(rename = "type")]
    play_type: PlayType,
}

/// The contents of a play information object.
#[derive(Debug, Deserialize)]
struct PlayInformation {
    base: Option<Base>,
    batter: Option<String>,
    pitcher: Option<String>,
    catcher: Option<String>,
    fielders: Option<Vec<String>>,
    runner: Option<String>,
    scoring_runner: Option<String>,
}

/// The contents of a play movements object.
#[derive(Debug, Deserialize)]
struct PlayMovements {
    movements: Vec<Movement>,
}

/// A streaming parser for the format described in `FORMAT.md`.
#[pyclass]
pub struct Parser {
    /// Whether to print debug information.
    debug: bool,
    /// The type of line to be parsed next.
    line_type: LineType,
    /// The builder for the game.
    pub game_builder: GameBuilder,
}

impl Parser {
    /// The JSON schema for a play introduction.
    fn play_introduction_json(&self) -> JsonType {
        let (valid_inning_numbers, valid_inning_tops) = if let Some(play) = self.game_builder.plays.last() {
            let current_inning = play.get_inning();
            let next_inning = current_inning.next();

            let mut valid_inning_numbers = vec![current_inning.number as usize];
            let mut valid_inning_tops = vec![current_inning.top];

            if !valid_inning_numbers.contains(&(next_inning.number as usize)) {
                valid_inning_numbers.push(next_inning.number as usize);
            }
            if !valid_inning_tops.contains(&next_inning.top) {
                valid_inning_tops.push(next_inning.top);
            }

            (valid_inning_numbers, valid_inning_tops)
        } else {
            (vec![1], vec![true])
        };

        let inning = JsonType::key_value("inning", JsonType::object(vec![
            JsonType::key_value("number", JsonType::integer_with_options(valid_inning_numbers)),
            JsonType::key_value("top", JsonType::boolean_with_options(valid_inning_tops)),
        ]));
        let play_type = JsonType::key_value(
            "type",
            JsonType::string_with_regex(&PlayType::iter().map(|play_type| play_type.to_string()).collect::<Vec<_>>().join("|")),
        );

        JsonType::object(vec![
            inning,
            play_type,
        ])
    }
    
    /// Generates the JSON schema for a given `Movement` object.
    fn movement_json(&self, movement: &Movement) -> JsonType {
        JsonType::object(vec![
            JsonType::key_value("runner", JsonType::string_with_regex(&movement.runner.replace(".", "\\."))),
            JsonType::key_value("start_base", JsonType::string_with_regex(&movement.start_base.to_string())),
            JsonType::key_value("end_base", JsonType::string_with_regex(&movement.end_base.to_string())),
            JsonType::key_value("is_out", JsonType::boolean_with_options(vec![movement.is_out])),
        ])
    }

    /// The JSON schema for an array of valid movement objects.
    fn valid_movements_json(&self) -> JsonType {
        let runner_positions = self.game_builder.runner_positions.clone();
        let players_bases = runner_positions.iter().filter_map(|(base, player)| {
            if let Some(player) = player {
                Some((player, base))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        let mut valid_movements: Vec<Movement> = Vec::new();
        // add the runners' movements
        for (player, base) in players_bases {
            let valid_to_bases = base.valid_to_bases();
            for to_base in valid_to_bases {
                valid_movements.push(Movement {
                    runner: player.clone(),
                    start_base: *base,
                    end_base: to_base,
                    is_out: false,
                });
                valid_movements.push(Movement {
                    runner: player.clone(),
                    start_base: *base,
                    end_base: to_base,
                    is_out: true,
                });
            }

            valid_movements.push(Movement {
                runner: player.clone(),
                start_base: *base,
                end_base: *base,
                is_out: true,
            });
        }

        // add the batter's movements
        if let Some(batter) = &self.game_builder.play_builder.batter {
            let valid_to_bases = Base::Home.valid_to_bases();
            for to_base in valid_to_bases {
                valid_movements.push(Movement {
                    runner: batter.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: false,
                });
                valid_movements.push(Movement {
                    runner: batter.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: true,
                });
            }
        }

        // add pinch runners' movements
        let runners_are_home_team = !self.game_builder.play_builder.inning.unwrap().top;
        let pinch_runners = if runners_are_home_team {
            self.game_builder.home_team_pinch_runner_names()
        } else {
            self.game_builder.away_team_pinch_runner_names()
        }.unwrap();

        let pinch_movements = valid_movements.iter().map(|movement| pinch_runners.iter().map(|runner| Movement {
            runner: runner.clone(),
            start_base: movement.start_base,
            end_base: movement.end_base,
            is_out: movement.is_out,
        })).flatten().collect::<Vec<_>>();
        valid_movements.extend(pinch_movements);

        // add the runner's and scoring runner's movements
        if let Some(runner) = &self.game_builder.play_builder.runner {
            let valid_to_bases = Base::Home.valid_to_bases();
            for to_base in valid_to_bases {
                valid_movements.push(Movement {
                    runner: runner.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: false,
                });
                valid_movements.push(Movement {
                    runner: runner.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: true,
                });
            }
        }

        if let Some(scoring_runner) = &self.game_builder.play_builder.scoring_runner {
            let valid_to_bases = Base::Home.valid_to_bases();
            for to_base in valid_to_bases {
                valid_movements.push(Movement {
                    runner: scoring_runner.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: false,
                });
                valid_movements.push(Movement {
                    runner: scoring_runner.clone(),
                    start_base: Base::Home,
                    end_base: to_base,
                    is_out: true,
                });
            }
        }

        let valid_movements_json = valid_movements.iter().map(|movement| self.movement_json(movement)).collect::<Vec<_>>();
        let valid_movements_json = JsonType::array(JsonType::Union(valid_movements_json));

        valid_movements_json
    }

    /// The JSON schema for a movements line object. It is technically too broad, but it will do for now.
    fn movements_json(&self) -> JsonType {
        JsonType::object(vec![
            JsonType::key_value(
                "movements",
                self.valid_movements_json(),
            ),
        ])
    }

    /// Generates the JSON schema for a play information object with the given fields.
    fn single_play_information_json(
        &self,
        needs_base: bool,
        needs_batter: bool,
        needs_pitcher: bool,
        needs_catcher: bool,
        needs_fielders: bool,
        needs_runner: bool,
        needs_scoring_runner: bool,
        needs_movements: bool,
    ) -> JsonType {
        let home_team_batting = !self.game_builder.play_builder.inning.unwrap().top; // if the inning is top, then the away team is batting
        let home_team_player_names_regex = if let Some(names) = self.game_builder.home_team_player_names() {
            names.iter().map(|name| format!("({name})").replace(".", "\\.")).collect::<Vec<_>>().join("|")
        } else {
            format!("{UNICODE_WORD_CHAR}+")
        };
        let away_team_player_names_regex = if let Some(names) = self.game_builder.away_team_player_names() {
            names.iter().map(|name| format!("({name})").replace(".", "\\.")).collect::<Vec<_>>().join("|")
        } else {
            format!("{UNICODE_WORD_CHAR}+")
        };

        let mut json_object = Vec::new();

        if needs_base {
            json_object.push(JsonType::key_value("base", JsonType::string_with_regex(r"home|1|2|3")));
        }
        if needs_batter {
            json_object.push(JsonType::key_value("batter", JsonType::string_with_regex(if home_team_batting {
                &home_team_player_names_regex
            } else {
                &away_team_player_names_regex
            })));
        }
        if needs_pitcher {
            json_object.push(JsonType::key_value("pitcher", JsonType::string_with_regex(if home_team_batting {
                &away_team_player_names_regex
            } else {
                &home_team_player_names_regex
            })));
        }
        if needs_catcher {
            json_object.push(JsonType::key_value("catcher", JsonType::string_with_regex(if home_team_batting {
                &away_team_player_names_regex
            } else {
                &home_team_player_names_regex
            })));
        }
        if needs_fielders {
            json_object.push(JsonType::key_value("fielders", JsonType::array(JsonType::string_with_regex(if home_team_batting {
                &away_team_player_names_regex
            } else {
                &home_team_player_names_regex
            }))));
        }
        if needs_runner {
            json_object.push(JsonType::key_value("runner", JsonType::string_with_regex(if home_team_batting {
                &home_team_player_names_regex
            } else {
                &away_team_player_names_regex
            })));
        }
        if needs_scoring_runner {
            json_object.push(JsonType::key_value("scoring_runner", JsonType::string_with_regex(if home_team_batting {
                &home_team_player_names_regex
            } else {
                &away_team_player_names_regex
            })));
        }

        JsonType::object(json_object)
    }

    /// Generates the JSON schema for a play information object for the given `PlayType`.
    fn play_information_json_for_play_type(&self, play_type: &PlayType) -> JsonType {
        match play_type {
            PlayType::Groundout =>              self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::BuntGroundout =>          self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::Strikeout =>              self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::Lineout =>                self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::BuntLineout =>            self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::Flyout =>                 self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::PopOut =>                 self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::BuntPopOut =>             self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::Forceout =>               self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::FieldersChoiceOut =>      self.single_play_information_json(  false,  true,   true,   false,  true,   false,  true,   true),
            PlayType::DoublePlay =>             self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::TriplePlay =>             self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::RunnerDoublePlay =>       self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::RunnerTriplePlay =>       self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::GroundedIntoDoublePlay => self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::StrikeoutDoublePlay =>    self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::Pickoff =>                self.single_play_information_json(  true,   false,  false,  false,  true,   true,   false,  true),
            PlayType::PickoffError =>           self.single_play_information_json(  true,   false,  false,  false,  true,   true,   false,  true),
            PlayType::CaughtStealing =>         self.single_play_information_json(  true,   false,  false,  false,  true,   true,   false,  true),
            PlayType::PickoffCaughtStealing =>  self.single_play_information_json(  true,   false,  false,  false,  true,   true,   false,  true),
            PlayType::WildPitch =>              self.single_play_information_json(  false,  false,  true,   false,  false,  true,   false,  true),
            PlayType::RunnerOut =>              self.single_play_information_json(  false,  false,  false,  false,  true,   true,   false,  true),
            PlayType::FieldOut =>               self.single_play_information_json(  false,  false,  false,  false,  true,   true,   false,  true),
            PlayType::BatterOut =>              self.single_play_information_json(  false,  true,   false,  true,   false,  false,  false,  true),
            PlayType::Balk =>                   self.single_play_information_json(  false,  false,  true,   false,  false,  false,  false,  true),
            PlayType::PassedBall =>             self.single_play_information_json(  false,  false,  true,   true,   false,  false,  false,  true),
            PlayType::Error =>                  self.single_play_information_json(  false,  false,  true,   true,   false,  false,  false,  true),
            PlayType::Single =>                 self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::Double =>                 self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::Triple =>                 self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::HomeRun =>                self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::Walk =>                   self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::IntentWalk =>             self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::HitByPitch =>             self.single_play_information_json(  false,  true,   true,   false,  false,  false,  false,  true),
            PlayType::FieldersChoice =>         self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::CatcherInterference =>    self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::StolenBase =>             self.single_play_information_json(  true,   false,  false,  false,  false,  true,   false,  true),
            PlayType::SacFly =>                 self.single_play_information_json(  false,  true,   true,   false,  true,   false,  true,   true),
            PlayType::SacFlyDoublePlay =>       self.single_play_information_json(  false,  true,   true,   false,  true,   false,  true,   true),
            PlayType::SacBunt =>                self.single_play_information_json(  false,  true,   true,   false,  true,   true,   false,  true),
            PlayType::SacBuntDoublePlay =>      self.single_play_information_json(  false,  true,   true,   false,  true,   true,   false,  true),
            PlayType::FieldError =>             self.single_play_information_json(  false,  true,   true,   false,  true,   false,  false,  true),
            PlayType::GameAdvisory =>           self.single_play_information_json(  false,  false,  false,  false,  false,  false,  false,  false),
            PlayType::Ejection =>               self.single_play_information_json(  false,  false,  false,  false,  false,  false,  false,  true),
        }
    }

    /// Parses the given line as a `Context` object.
    fn parse_context(&mut self, line: &str) {
        let context: Context = serde_json::from_str(line).unwrap();
        self.game_builder.add_context(context);
    }

    /// Adds the given play to the `GameBuilder`.
    fn add_play(&mut self, play: Play) {
        if let Some(last_play) = self.game_builder.plays.last() {
            let last_inning = last_play.get_inning();
            let play_inning = play.get_inning();

            // if a new half inning is starting, clear the runner positions
            if last_inning.top != play_inning.top {
                self.game_builder.clear_runner_positions();
            }
        }
        self.game_builder.add_play(play);
    }

    /// Parses the given line as a `PlayIntroduction` object.
    fn parse_play_introduction(&mut self, line: &str) {
        let play_introduction: PlayIntroduction = serde_json::from_str(line).unwrap();
        self.game_builder.play_builder.inning = Some(play_introduction.inning);
        self.game_builder.play_builder.play_type = Some(play_introduction.play_type);

        if self.game_builder.play_builder.play_type.unwrap() == PlayType::GameAdvisory {
            let play = self.game_builder.play_builder.build();
            self.add_play(play);
        }
    }

    /// Parses the given line as a `PlayInformation` object.
    fn parse_play_information(&mut self, line: &str) {
        if self.debug {
            println!("play_information: {line:?}");
        }
        let play_information: PlayInformation = serde_json::from_str(line).unwrap();

        if let Some(base) = play_information.base {
            self.game_builder.play_builder.set_base(base);
        }
        if let Some(batter) = play_information.batter {
            self.game_builder.play_builder.set_batter(batter);
        }
        if let Some(pitcher) = play_information.pitcher {
            self.game_builder.play_builder.set_pitcher(pitcher);
        }
        if let Some(catcher) = play_information.catcher {
            self.game_builder.play_builder.set_catcher(catcher);
        }
        if let Some(fielders) = play_information.fielders {
            self.game_builder.play_builder.set_fielders(fielders);
        }
        if let Some(runner) = play_information.runner {
            self.game_builder.play_builder.set_runner(runner);
        }
        if let Some(scoring_runner) = play_information.scoring_runner {
            self.game_builder.play_builder.set_scoring_runner(scoring_runner);
        }

        if self.game_builder.play_builder.play_type.unwrap() == PlayType::GameAdvisory {
            let play = self.game_builder.play_builder.build();
            self.add_play(play);
        }
    }

    /// Parses the given line as a `PlayMovements` object.
    fn parse_play_movements(&mut self, line: &str) {
        let play_movements: PlayMovements = serde_json::from_str(line).unwrap();
        self.game_builder.play_builder.set_movements(play_movements.movements);

        let play = self.game_builder.play_builder.build();
        self.add_play(play);
    }
}

#[pymethods]
impl Parser {
    /// Creates a new parser. If `debug` is true, debug information will be printed during parsing.
    #[new]
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            line_type: LineType::Context,
            game_builder: GameBuilder::new(),
        }
    }

    /// Generates the regex for the next line to be parsed.
    fn generate_regex(&self) -> String {
        match &self.line_type {
            LineType::Context => context_section_json().to_regex(),
            LineType::PlayIntroduction => self.play_introduction_json().to_regex(),
            LineType::PlayInformation => self.play_information_json_for_play_type(&self.game_builder.play_builder.play_type.unwrap()).to_regex(),
            LineType::PlayMovements => self.movements_json().to_regex(),
        }
    }

    /// Parses a line, updates the parser's state, and returns the next line's regex.
    pub fn parse_line(&mut self, line: &str) -> PyResult<String> {
        match &self.line_type {
            LineType::Context => {
                self.parse_context(line);
                self.line_type = LineType::PlayIntroduction;
            }
            LineType::PlayIntroduction => {
                self.parse_play_introduction(line);
                if self.debug {
                    println!("play_type: {:?}", self.game_builder.play_builder.play_type);
                }
                self.line_type = match self.game_builder.play_builder.play_type.unwrap() {
                    PlayType::GameAdvisory => LineType::PlayIntroduction,
                    PlayType::Ejection => LineType::PlayMovements,
                    _ => LineType::PlayInformation,
                };
            }
            LineType::PlayInformation => {
                self.parse_play_information(line);
                self.line_type = LineType::PlayMovements;
            }
            LineType::PlayMovements => {
                self.parse_play_movements(line);
                self.line_type = LineType::PlayIntroduction;
            }
        }

        Ok(self.generate_regex())
    }

    /// Builds and returns the game.
    pub fn finish(&self) -> PyResult<Game> {
        match self.game_builder.build() {
            Ok(game) => Ok(game),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use game::{Context, Weather, Team, Player, Play, Base};

    #[test]
    fn parse_context() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);
        assert_eq!(parser.game_builder.context.unwrap(), Context {
            game_pk: 123456,
            date: "2024-04-24".to_string(),
            venue_name: "Test Stadium".to_string(),
            weather: Weather { condition: "Sunny".to_string(), temperature: 70, wind_speed: 10 },
            home_team: Team { id: 1, players: vec![Player { position: "PITCHER".to_string(), name: "John Doe".to_string() }] },
            away_team: Team { id: 2, players: vec![Player { position: "CATCHER".to_string(), name: "Jane Doe".to_string() }] },
        });
    }

    #[test]
    fn parse_play_introduction() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);

        let _ = parser.parse_line("{ \"inning\": { \"number\": 1, \"top\": true }, \"type\": \"Groundout\" }\n");
        assert_eq!(parser.line_type, LineType::PlayInformation);
        assert_eq!(parser.game_builder.play_builder.inning.unwrap(), Inning { number: 1, top: true });
        assert_eq!(parser.game_builder.play_builder.play_type.unwrap(), PlayType::Groundout);
    }

    #[test]
    fn parse_play_information() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);

        let _ = parser.parse_line("{ \"inning\": { \"number\": 1, \"top\": true }, \"type\": \"Groundout\" }\n");
        assert_eq!(parser.line_type, LineType::PlayInformation);
        assert_eq!(parser.game_builder.play_builder.inning.unwrap(), Inning { number: 1, top: true });
        assert_eq!(parser.game_builder.play_builder.play_type.unwrap(), PlayType::Groundout);

        let _ = parser.parse_line("{ \"batter\": \"Jane Doe\", \"pitcher\": \"John Doe\", \"fielders\": [\"John Doe\"] }\n");
        assert_eq!(parser.line_type, LineType::PlayMovements);
    }

    #[test]
    fn parse_game_advisory_does_not_expect_play_information_nor_movements() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);

        let _ = parser.parse_line("{ \"inning\": { \"number\": 1, \"top\": true }, \"type\": \"Game Advisory\" }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);
    }

    #[test]
    fn parse_ejection_does_not_expect_play_information_but_does_expect_movements() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);

        let _ = parser.parse_line("{ \"inning\": { \"number\": 1, \"top\": true }, \"type\": \"Ejection\" }\n");
        assert_eq!(parser.line_type, LineType::PlayMovements);
    }

    #[test]
    fn parse_play_movements() {
        let mut parser = Parser::new(true);

        let _ = parser.parse_line("{ \"game_pk\": 123456, \"date\": \"2024-04-24\", \"venue_name\": \"Test Stadium\", \"weather\": { \"condition\": \"Sunny\", \"temperature\": 70, \"wind_speed\": 10 }, \"home_team\": { \"id\": 1, \"players\": [{ \"position\": \"PITCHER\", \"name\": \"John Doe\" }] }, \"away_team\": { \"id\": 2, \"players\": [{ \"position\": \"CATCHER\", \"name\": \"Jane Doe\" }] } }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);

        let _ = parser.parse_line("{ \"inning\": { \"number\": 1, \"top\": true }, \"type\": \"Walk\" }\n");
        assert_eq!(parser.line_type, LineType::PlayInformation);

        let _ = parser.parse_line("{ \"batter\": \"Jane Doe\", \"pitcher\": \"John Doe\" }\n");
        assert_eq!(parser.line_type, LineType::PlayMovements);

        let _ = parser.parse_line("{ \"movements\": [{ \"runner\": \"Jane Doe\", \"start_base\": \"home\", \"end_base\": \"1\", \"is_out\": false }] }\n");
        assert_eq!(parser.line_type, LineType::PlayIntroduction);
        assert_eq!(parser.game_builder.plays.len(), 1);
        assert_eq!(parser.game_builder.plays[0], Play::Walk {
            inning: Inning { number: 1, top: true },
            batter: "Jane Doe".to_string(),
            pitcher: "John Doe".to_string(),
            movements: vec![Movement {
                runner: "Jane Doe".to_string(),
                start_base: Base::Home,
                end_base: Base::First,
                is_out: false,
            }],
        });
        assert_eq!(parser.game_builder.runner_positions, HashMap::from([
            (Base::First, Some("Jane Doe".to_string())),
            (Base::Second, None),
            (Base::Third, None),
        ]));
    }

    #[test]
    fn parse_entire_game() {
        let mut parser = Parser::new(true);

        let game = include_str!("../test_data/748236.jsonl");
        for line in game.lines() {
            let _ = parser.parse_line(line);
        }

        assert_eq!(parser.game_builder.plays.len(), 78);
    }
}
