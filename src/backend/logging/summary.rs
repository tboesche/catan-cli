use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::backend::setup::game::Game;

use super::player_summary::PlayerSummary;


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Summary {
    pub game_title: Option<String>,
    pub n_rounds: usize,
    pub winner_id: u32,
    pub duration: u128,
    pub player_summaries: Vec<PlayerSummary>,
}

impl Summary {
    pub fn new(game: &Game) -> Self {
        
        let n_rounds = game.log.len();

        let winner_id = game.round.board.scores.iter()
                                    .enumerate()
                                    .max_by_key(|&(_, value)| value)
                                    .map(|(index, _)| index)
                                    .unwrap() as u32;

        let duration = game.log.iter().fold(0, |mut acc, entry| {
            let dur = match entry.duration_ms {
                Some(d) => d,
                None => 0,
            };

            acc += dur;

            acc
        });

        let player_summaries = (0..game.parameters.n_players)
                                                            .map(|i_player| PlayerSummary::new(game, i_player as usize))
                                                            .collect();


        Summary {
            game_title: game.parameters.clone().title,
            n_rounds,
            winner_id,
            duration,
            player_summaries,
        }
    }
}
