use crate::backend::{round::action::Action, setup::game::Game, logging::log_entry::LogEntry};


pub fn encode_action(game: &Game, legal_action: &Action) -> Vec<u32> {

    let new_log = extend_log(game, legal_action);

    game.hot_encode_log(&new_log)
}

fn extend_log(game: &Game, legal_action: &Action) -> LogEntry {

    let mut game_clone = game.clone();

    game_clone.take_action(legal_action.clone(), None);

    game_clone.log[game_clone.log.len() - 1].clone()
}