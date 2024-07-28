use crate::{ai::simple_nn::evaluate::evaluate_actions, backend::{round::action::Action, setup::game::Game}};
use rand::{prelude::SliceRandom, thread_rng};

pub fn play(game: &Game, legal_actions: Vec<Action>) -> Option<Action> {
    
    let action_values = evaluate_actions(game, &legal_actions);

    let max_value = action_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let max_indices: Vec<usize> = action_values.iter()
                                     .enumerate()
                                     .filter_map(|(index, &value)| {
                                         if value == max_value {
                                             Some(index)
                                         } else {
                                             None
                                         }
                                     })
                                     .collect();

    // ADD CHECK FOR TRADES: DO NOT WANT TO MAKE THE SAME OFFER WITHIN THE SAME TURN (use phase_count)
    let best_actions: Vec<Action> = max_indices.iter()
                                .map(|i| legal_actions[*i].clone())
                                .collect();

    let mut rng = thread_rng();
    let selected_action = best_actions.choose(&mut rng).cloned();

    // println!("Dice throws: {:#?}", game.log[game.log.len() - 1].count_dice_draws);

    selected_action
}