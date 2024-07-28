use std::sync::Arc;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::backend::round::action::Action;
use crate::backend::round::cards::CardType::{KnightCard, MonopolyCard, PlentyCard, RoadsCard, VPCard};
use crate::backend::setup::game::Game;

use crate::backend::setup::game_parameters::GameParameters;
use crate::backend::setup::node_status::NodeStatus::{Adjacent, Citied, Free, Settled};

use crate::backend::setup::player::PlayerType::Classic;

pub fn play(game: &Game, legal_actions: Vec<Action>) -> Option<Action> {

    let action_values:Vec<f64> = evaluate_actions(&game, &legal_actions);

    let max_value = action_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

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


fn evaluate_actions(game: &Game, input_actions: &Vec<Action>) -> Vec<f64> {

    
    let mut output = vec![0.0; input_actions.len()];

    for (i_action, action) in input_actions.iter().enumerate() {
        output[i_action] = match &action {
            Action::SetUpMove(settlement_node, end_node) => {

                let v_resources = evaluate_resources(game);

                let settle_value = evaluate_settlement(*settlement_node as usize, &v_resources, game);

                let road_value = evaluate_road(*settlement_node as usize, *end_node as usize, &v_resources, game);

                // println!("Evaluated setup move");

                settle_value + road_value
            },

            Action::Robber(i_robber, i_tile, i_victim) => {
                evaluate_robber(i_robber, i_tile, i_victim, game)
            },

            Action::DiscardCards(v_discard) => {
                let v_resources = evaluate_resources(game);
                
                let player_id = game.round.active_player as usize;
                let budget = &game.round.board.budgets[player_id];
   
                let new_budget: Vec<u32> = budget.iter()
                                        .zip(v_discard.iter())
                                        .map(|(&b,d)| b.checked_sub(*d).unwrap_or(0_u32))
                                        .collect();

                new_budget.iter()
                        .zip(v_resources.iter())
                        .map(|(b, v)| v * *b as f64)
                        .sum::<f64>()
            },

            Action::NoDiscard => {
                0.0
            }

            Action::NoCardPlay => {
                0.0
            },

            Action::CardPlay(card_type) => {
                match card_type {
                    VPCard => {         // no point playing victory cards. Better to only reveal at end of game.
                        f64::MIN
                    },
                    KnightCard(i_robber, i_tile, i_victim) => {
                        evaluate_robber(i_robber, i_tile, i_victim, game)
                    },
                    RoadsCard(first_start, first_end, second_start, second_end) => {
                        let v_resources = &evaluate_resources(game);
                        
                        let first_value = evaluate_road(*first_start as usize, *first_end as usize, v_resources, game);
                        let second_value = evaluate_road(*second_start as usize, *second_end as usize, v_resources, game);

                        first_value + second_value
                    },
                    PlentyCard(first_r, second_r) => {
                        let v_resources = evaluate_resources(game);

                        v_resources[*first_r as usize] + v_resources[*second_r as usize]
                    },
                    MonopolyCard(i_resource) => {

                        let player_id = game.round.active_player as usize;
                        let v_resources = evaluate_resources(game);

                        let public_budgets = &game.round.board.public_budgets;

                        let public_budget: u32 = public_budgets.iter()
                                                .map(|pb| pb[*i_resource as usize])
                                                .sum();

                        let private_budget = game.round.board.budgets[player_id][*i_resource as usize];

                        let total = public_budget.checked_sub(private_budget).unwrap_or(0);

                        v_resources[*i_resource as usize] * total as f64
                    },
                }
            },

            Action::NoTrade => {
                0.0
            },

            Action::BankTrade(r_supplied, r_demanded) => {
                evaluate_trade(*r_supplied as usize, 4.0, *r_demanded as usize, 1.0, game)
            },

            Action::HarborTrade(i_harbor, r_supplied, r_demanded) => {
                if *i_harbor < game.parameters.n_resources {
                    evaluate_trade(*r_supplied as usize, 2.0, *r_demanded as usize, 1.0, game)
                } else {
                    evaluate_trade(*r_supplied as usize, 3.0, *r_demanded as usize, 1.0, game)
                }
            },

            Action::TradeQuote(quote) => {
                evaluate_trade(quote.resource_supplied as usize, quote.quantity_supplied as f64, quote.resource_demanded as usize, quote.quantity_demanded as f64, game)
            },

            Action::TradeResponse(_, accept) => {

                let v_resources = evaluate_resources(game);
                let active_player = game.round.active_player as usize;

                let value_budget = game.round.board.budgets[active_player].iter()
                    .zip(v_resources.iter())
                    .map(|(b, v)| v * *b as f64)
                    .sum::<f64>();

                if *accept {
                    let quote = &game.round.action.clone().unwrap();

                    match &quote {
                        Action::TradeQuote(q) => {
                            let value_supplied = q.quantity_supplied as f64 * v_resources[q.resource_supplied as usize];
                            let value_demanded = q.quantity_demanded as f64 * v_resources[q.resource_demanded as usize];

                            value_budget - value_supplied + value_demanded
                        },
                        _ => value_budget,
                    }

                } else {
                    value_budget 
                }
                
            },

            Action::NoBuying => {
                -10.0
            },

            Action::BuildRoad(start_node, end_node) => {
                let v_resources = &evaluate_resources(game);
                evaluate_road(*start_node as usize, *end_node as usize, v_resources, game)
            },

            Action::BuildSettlement(settlement_node) => {
                let v_resources = &evaluate_resources(game);
                evaluate_settlement(*settlement_node as usize, v_resources, game)
            },

            Action::BuildCity(city_node) => {
                let v_resources: &Vec<f64> = &evaluate_resources(game);
                evaluate_city(*city_node as usize, v_resources, game)
            },

            Action::BuyDevCard => {
                // NOTE: so far, the ex ante value of a dev card only depends on the probability of drawing a victory point. Other cards would require forward-looking beliefs.

                let active_player = game.round.active_player as usize;

                let n_init_vp_cards = game.parameters.init_undrawn_dev_cards[0];

                let n_max_vp_cards = n_init_vp_cards - game.round.board.drawn_dev_cards[active_player][0];

                let n_init_total_dev_cards: u32 = game.parameters.init_undrawn_dev_cards.iter().sum();
                let n_total_drawn = game.round.board.drawn_dev_cards.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>();
                let n_total_undrawn = n_init_total_dev_cards - n_total_drawn;

                n_max_vp_cards as f64 / n_total_undrawn as f64
            },

            Action::FinishRound => {
                0.0
            },

            Action::Save => {
                f64::MIN
            },

            Action::Quit => {
                f64::MIN
            },
        }
    }

    
    output
}

fn evaluate_resources(game: &Game) -> Vec<f64> {

    let mut v_resources: Vec<f64> = vec![];

    for i_resource in 0..game.parameters.n_resources {
        let player_id = game.round.active_player as usize;

        let default_weights = vec![vec![0.0; 13]; 5];

        let weights = match &game.parameters.v_players[player_id].player_type {
            Classic(w) => w,
            _ => &default_weights,
        };

        let r_weights = &weights[i_resource as usize];

        let covariates = vec![
            1.0,
            //game.log[&game.log.len() - 1].log_id as f64,
            game.round.board.scores[player_id] as f64,
            game.round.board.total_drawn_resources[player_id][0] as f64,
            game.round.board.total_drawn_resources[player_id][1] as f64,
            game.round.board.total_drawn_resources[player_id][2] as f64,
            game.round.board.total_drawn_resources[player_id][3] as f64,
            game.round.board.total_drawn_resources[player_id][4] as f64,
            // 0.0,
            // 0.0,
            // 0.0,
            // 0.0,
            // 0.0, 
            game.round.board.budgets[player_id][0] as f64,
            game.round.board.budgets[player_id][1] as f64,
            game.round.board.budgets[player_id][2] as f64,
            game.round.board.budgets[player_id][3] as f64,
            game.round.board.budgets[player_id][4] as f64,
            // 0.0,
            // 0.0,
            // 0.0,
            // 0.0,
            // 0.0,
        ];

        v_resources.push(r_weights.iter().zip(covariates.iter()).map(|(w, x)| w * x).sum());
    }

    v_resources
}

fn evaluate_tile(i_tile: usize, v_resources: &Vec<f64>, game: &Game) -> f64 {

    let tile = &game.round.board.tiles[i_tile];

    let rng_prob: f64; 
    
    match tile.rng {
        Some(s) => rng_prob = get_rng_probability(s as usize, &game.parameters),
        None => return 0.0,
    };

    match tile.resource {
        Some(r) => rng_prob * v_resources[r as usize],
        None => 0.0,
    }
}


fn evaluate_node(i_node: usize, v_resources: &Vec<f64>, game: &Game) -> f64 {

    let mut value = game.parameters.node_tiles_adjacency[i_node].iter()
                                            .map(|i_tile| {
                                                evaluate_tile(*i_tile, v_resources, game)                                         
                                            })
                                            .sum::<f64>();

    let node = &game.round.board.nodes[i_node];

    match &node.harbor {
        Some(_) => value += 0.01,
        None => (),
    };

    match &node.node_status {
        Free => (),
        Adjacent => value -= 0.05,
        Settled(_) => value -= 0.1,
        Citied(_) => value -= 0.1,
    }

    value
}

fn get_rng_probability(s: usize, parameters: &GameParameters) -> f64 {

    let n = parameters.n_faces as usize;
    let d = parameters.n_dice as usize;

    // dp[i][j] will be storing the number of ways to get sum j using i dices
    let mut dp = vec![vec![0; s + 1]; d + 1];
    
    // Base case: There's one way to get sum 0 with 0 dice
    dp[0][0] = 1;

    // Fill the table using a bottom-up approach
    for dice in 1..=d {
        for sum in 1..=s {
            for face in 1..=n {
                if sum >= face {
                    dp[dice][sum] += dp[dice - 1][sum - face];
                }
            }
        }
    }

    // Total number of possible outcomes when rolling d dice with n faces each
    let total_outcomes = (n as f64).powi(d as i32);

    // Number of ways to get sum S with D dice
    let ways_to_get_s = dp[d][s];

    // Calculate the probability
    (ways_to_get_s as f64) / total_outcomes
}



fn evaluate_settlement(i_node: usize, v_resources: &Vec<f64>, game: &Game) -> f64 {
    evaluate_node(i_node, &v_resources, game) + 1.0
}

fn evaluate_city(i_node: usize, v_resources: &Vec<f64>, game: &Game) -> f64 {
    1.5 * evaluate_settlement(i_node, &v_resources, game)
}

fn evaluate_road(first_node: usize, second_node: usize, v_resources: &Vec<f64>, game: &Game) -> f64 {
    (evaluate_node(first_node, &v_resources, game) + evaluate_node(second_node, &v_resources, game)) + 0.5
}


fn evaluate_robber(i_robber: &u32, i_tile: &u32, i_victim: &Option<u32>, game: &Game) -> f64 {
    let v_resources = evaluate_resources(game);
                
                let player_id = game.round.active_player as usize;
                let tile = &game.round.board.tiles[*i_tile as usize];

                let tile_value = evaluate_tile(*i_tile as usize, &v_resources, game);

                // check whether robber is on the player's own field
                let old_robber_tile = &game.round.board.tiles[game.round.board.v_robbers[*i_robber as usize] as usize];  
                let old_r_nodes = &old_robber_tile.nodes;

                let mut own_node_bonus = 0.0;
                for i_node in old_r_nodes {
                    let node = &game.round.board.nodes[*i_node as usize];

                    match &node.node_status {
                        Free => (),
                        Adjacent => (),
                        Settled(owner) => {
                            if *owner as usize == player_id {
                                own_node_bonus += 100.0;
                            }
                        },
                        Citied(owner) => {
                            if *owner as usize == player_id {
                                own_node_bonus += 100.0;
                            }
                        },
                    }
                }

                // check whether victim is leading in public score
                let mut leading_victim_bonus = 0.0;
                match i_victim {
                    Some(idx_victim) => {
                        let leading_score = game.round.board.public_scores.iter().fold(0_u32, |acc, x| acc.max(*x));


                        let leaders: Vec<usize> = game.round.board.public_scores.iter()
                                            .enumerate()
                                            .filter_map(|(index, &value)| {
                                                if value == leading_score {
                                                    Some(index)
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect();

                        for leader in leaders {
                            if leader == *idx_victim as usize {
                                leading_victim_bonus += 1.0
                            }
                        }
                            },
                    None => (),
                }
                
                tile_value + own_node_bonus + leading_victim_bonus
}


fn evaluate_trade(r_supplied: usize, q_supplied: f64, r_demanded: usize, q_demanded: f64, game: &Game) -> f64 {
    let v_resources = evaluate_resources(game);

    q_demanded * v_resources[r_demanded] - q_supplied * v_resources[r_supplied]
}