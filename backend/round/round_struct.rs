use serde::Deserialize;

use crate::backend::setup::game_parameters::GameParameters;

use super::{action::Action, outcome::Outcome, phase::Phase, super::setup::board::Board};

#[derive(Debug, Clone, Deserialize)]
pub struct Round {      
    pub board: Board, 
    pub active_player: u32,
    pub throwing_player: u32,
    pub cards_played: u32,
    pub phase: Phase,
    pub phase_count: u32,
    pub card_count: u32,
    pub robber_count: u32,
    pub action: Option<Action>,
    pub outcome: Option<Outcome>,
}


