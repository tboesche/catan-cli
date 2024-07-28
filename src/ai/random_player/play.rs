use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::backend::round::action::Action;
use crate::backend::setup::game::Game;


pub fn play(game: &Game, legal_actions: Vec<Action>) -> Option<Action> {
    // Select a random action from the available actions
    let mut rng = thread_rng();
    let selected_action = legal_actions.choose(&mut rng).cloned();

    // println!("Dice throws: {:#?}", game.log[game.log.len() - 1].count_dice_draws);

    selected_action
}