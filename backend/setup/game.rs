
use std::{io::{self, Error}, thread::sleep, time::{Duration, Instant, SystemTime}};

use crate::{backend::{logging::log_entry, round::{action::Action, legal, phase::Phase, round_struct::Round}}, frontend::{actions::get_pretty_action, board_parameters::UIBoardParameters}};

use super::{super::logging::{log_entry::LogEntry, summary::Summary}, board::Board, game_parameters::GameParameters, player::PlayerType};

#[derive(Debug, Clone)]
pub struct Game {
    pub parameters: GameParameters,
    pub round: Round,
    pub log: Vec<LogEntry>,
    pub result: Option<Summary>,
}

impl Default for Game {
    fn default() -> Self {

        let start = SystemTime::now();

        let parameters = GameParameters::default();

        Self::initialize_from_parameters(parameters, start).expect("Initialization of default game failed:")
    }
}

impl Game {
    pub fn new(parameters: GameParameters) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn from_template(template_name: String) -> Result<Self, &'static str> {

        let start = SystemTime::now();

        // initialize game parameters
        let parameters = GameParameters::default();
        let parameters = parameters.default_from_template(None, template_name);

        Self::initialize_from_parameters(parameters, start)
    }

    pub fn from_template_settled(template_name: String) -> Result<Self, &'static str> {

        let start = SystemTime::now();

        // initialize game parameters
        let parameters = GameParameters::default();
        let parameters = parameters.default_settled(None, template_name);

        // println!("{:#?}", parameters);

        let mut game = Self::initialize_from_parameters(parameters, start)?;

        game.round.phase = Phase::FirstCardPhase;

        Ok(game)
    }

    fn initialize_round(parameters: &GameParameters) -> Result<Round, &'static str> {

        let board = Board::new(parameters)?;

        let active_player = parameters.init_player;

        let phase = &parameters.init_phase;

        Ok(Round {
            board,
            active_player,
            throwing_player: active_player,
            cards_played: 0,
            phase: phase.clone(),
            phase_count: 0,
            card_count: 0,
            robber_count: 0,
            action: None,
            outcome: None,
        })
    }

    fn initialize_from_parameters(parameters: GameParameters, start: SystemTime) -> Result<Self, &'static str> {
        // initialize round
        let round = Self::initialize_round(&parameters)?;

        // initialize result
        let result: Option<Summary> = None;

        let log = log_entry::initialize_log(&round, start);

        Ok(Self {
            parameters,
            round,
            log,
            result,
        })
    }

    pub fn run(&mut self) -> Result<(), &'static str> {

        // println!("tile 6: {:?}", self.round.board.tiles[6].nodes);

        let v_players = self.parameters.v_players.clone();

        loop {
            let legal_actions = &self.get_legal_actions();
            // println!("len. actions: {:?}", legal_actions.len());

            let player = &v_players[self.round.active_player as usize];
            let selected_action: Option<Action>;
            let building_name: Option<String>;
            match &player.player_type  {
                PlayerType::Human => {

                    let ui_parameters = UIBoardParameters::default();

                    self.draw_board(ui_parameters.clone(), "board".to_string());

                    println!("Available actions:");

                    for (i_action, action) in legal_actions.iter().enumerate() {
                        println!("{:?}: {}", i_action, get_pretty_action(action, self, &ui_parameters));
                    }

                    println!("Please choose an action from among the above options!");

                    match &self.round.board.dice_outcome {
                        Some(d) => println!("Last Dice: {:?}", d),
                        None => (),
                    }
                    
                    println!("Your budget: {:?}", self.round.board.budgets[self.round.active_player as usize]);

                    let mut input = String::new();

                    if legal_actions.len() == 3 {  // do not ask for input if only one action is available
                        selected_action = Some(legal_actions[0].clone());

                        println!("You selected: {}", get_pretty_action(&legal_actions[0], self, &ui_parameters));
                    } else {

                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let input: usize = match input.trim().parse() {
                            Ok(num) => {
                                if num < legal_actions.len() {
                                    num
                                } else {
                                    println!("Please pick one of the above actions by their index.");
                                    continue
                                }
                            },
                            Err(_) => {
                                println!("Please type an integer from the above list of actions");
                                continue
                            },
                        };

                        if input == legal_actions.len() - 1 {
                            return Ok(())
                        }

                        selected_action = Some(legal_actions[input].clone());

                        println!("You selected: {}", get_pretty_action(&legal_actions[input], self, &ui_parameters));

                    }

                    building_name = None;

                    

                },
                _ => {
                    let player_function = match player.player_function {
                        Some(pf) => {
                            pf
                        },
                        None => {
                            return Err("AI player not implemented.");
                        },
                    };

                    selected_action = player_function(&self, legal_actions.clone());
                    match &selected_action {
                        Some(action) => {
                            let mut human_present = false;
                            for player in &self.parameters.v_players {
                                match player.player_type {
                                    PlayerType::Human => {
                                        human_present = true;
                                        break
                                    },
                                    _ => ()
                                }
                            }

                            if human_present {
                                let ui_parameters = UIBoardParameters::default();
                                println!("Player {:?} selected {}",self.round.active_player, get_pretty_action(action, self, &ui_parameters));
                                sleep(Duration::from_secs(1));
                            }
                        },
                        None => {
                            println!("Player {:?} selected no action.", self.round.active_player);
                        },
                    }
                    // println!("Player {:?} selected {:?}",self.round.active_player, selected_action);

                    building_name = None;
                },
            }

            if let Some(action) = selected_action {
                // println!("Taking action.");
                self.take_action(action.clone(), building_name);
                // println!("Action taken.")
                // self.draw_board(ui_parameters.clone());
            } else {
                // println!("No action selected");
                return Err("Player failed to select a legal action.");
            }

            if self.round.board.scores[self.round.active_player as usize] >= 10 {
                // self.draw_board(ui_parameters.clone());
                // println!("Player {} won!", self.round.active_player);
                // println!("Public scores: {:?}", self.round.board.public_scores);
                // println!("Scores: {:?}", self.round.board.scores);
                // println!("# Rounds: {:?}", self.log.len());
                // println!("Summary: {:#?}", Summary::new(self));

                let mut human_present = false;
                for player in &self.parameters.v_players {
                    match player.player_type {
                        PlayerType::Human => {
                            human_present = true;
                            break
                        },
                        _ => ()
                    }
                }

                if human_present {
                    println!("Player {} won!", self.round.active_player);
                    println!("Scores: {:?}", self.round.board.scores);
                }

                self.result = Some(Summary::new(&self));

                break
            }

            // println!("Public scores: {:?}", self.round.board.public_scores);
            // println!("Scores: {:?}", self.round.board.scores);
        }

        Ok(())
    }
}


