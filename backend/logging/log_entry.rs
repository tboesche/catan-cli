use std::time::SystemTime;

use serde::Deserialize;

use crate::backend::{round::outcome::Outcome::{DrawCardOutcome, RobberOutcome}, setup::game::Game};

use super::super::round::round_struct::Round;

#[derive(Debug, Deserialize, Clone)]
pub struct LogEntry {
    pub log_id: u32,
    pub round: Option<Round>,
    prev_dice_outcome: Option<u32>,

    start_time: SystemTime,
    end_time: Option<SystemTime>,
    pub duration_ms: Option<u128>,

    pub count_dice_draws: u32,
    count_robber_draws: u32,
    count_card_draws: u32,
}

pub fn initialize_log(round: &Round, start: SystemTime) -> Vec<LogEntry>  {

    let log_id = 0_u32;

    let count_dice_draws = 0_u32;
    let count_robber_draws = 0_u32;
    let count_card_draws = 0_u32;

    let start_time = start;
    let end_time = Some(SystemTime::now());

    let duration = end_time.expect("Unwrapping end_time failed.").duration_since(start)
        .expect("SystemTime::duration_since failed");
    let duration_ms = Some(duration.as_millis());

    let init_log = LogEntry {
        log_id,
        round: Some(round.clone()),
        prev_dice_outcome: None,
        start_time,
        end_time,
        duration_ms,
        count_dice_draws,
        count_robber_draws,
        count_card_draws
    };

    vec![init_log]
}

impl LogEntry {
    pub fn new(game: &Game) -> Self {

        let prev_entry = &game.log[game.log.len() - 1];

        let log_id = prev_entry.log_id + 1;

        let prev_dice_outcome = prev_entry.prev_dice_outcome;

        let start_time = SystemTime::now();

        let count_dice_draws = prev_entry.count_dice_draws;
        let count_robber_draws = prev_entry.count_robber_draws;
        let count_card_draws = prev_entry.count_card_draws;

        LogEntry {
            log_id,
            round: None,
            prev_dice_outcome,
            start_time,
            end_time: None,
            duration_ms: None,
            count_dice_draws,
            count_robber_draws,
            count_card_draws
        }
    }

    pub fn close(&mut self, game: &Game) {
        
        self.end_time = Some(SystemTime::now());

        let duration = self.end_time.expect("Unwrapping end_time failed.").duration_since(self.start_time)
        .expect("SystemTime::duration_since failed");
        self.duration_ms = Some(duration.as_millis());

        self.round = Some(game.round.clone());

        match &self.round {
            Some(round_unwrapped) => {

                let dice_outcome = round_unwrapped.board.dice_outcome;

                // println!("Dice: {:#?}", dice_outcome);
                // println!("prev Dice: {:#?}", self.prev_dice_outcome);

                if dice_outcome != self.prev_dice_outcome {
                    self.count_dice_draws += 1;
                    // println!("{:?}", self.count_dice_draws)
                }

                self.prev_dice_outcome = dice_outcome;

                match &round_unwrapped.outcome {
                    Some(outcome_unwrapped) => {
                        match outcome_unwrapped {
                            RobberOutcome(_) => {
                                self.count_robber_draws += 1;
                            },
                            DrawCardOutcome(_) => {
                                self.count_card_draws += 1;
                            },
                            _ => (), // no need to change anything if no random outcome in round
                        }
                    },
                    None => {
                        () // no need to change anything if no random outcome in round
                    },
                }
            },
            None => {
                println!("Log entry without Round found.")
            },
        }
    }
}