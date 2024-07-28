use std::{vec, collections::HashSet};

use serde::{Deserialize, Serialize};

use crate::backend::{round::{action::Action, outcome::Outcome}, setup::{game::Game, node_status::NodeStatus::{Citied, Settled}, player}};


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PlayerSummary {
    pub player_id: usize,
    pub scores: Vec<(u32, u32)>,
    pub actions_and_outcomes: Vec<(u32, Option<Action>, Option<Outcome>)>,
    pub budgets: Vec<(u32, Vec<u32>)>,
    pub drawn_resources: Vec<(u32, Vec<u32>)>,
    pub settlements: Vec<u32>,
    pub cities: Vec<u32>,
    pub roads: Vec<(u32, u32)>,
}

impl PlayerSummary {
    pub fn new(game: &Game, player_id: usize) -> Self {
        
        let n_rounds = game.log.len();

        // add settlements, cities, and roads in the final round (by player)
        // add final settlements, final cities and final roads (sorted by player)
        let mut settlements: Vec<u32> = vec![];
        let mut cities: Vec<u32> = vec![];
        let mut roads: Vec<(u32, u32)> = vec![];
        match &game.log[n_rounds-1].round {
            Some(round) => {
                for node in &round.board.nodes {
                    match &node.node_status {
                        Settled(owner) => {
                            if owner == &(player_id as u32) {
                                settlements.push(node.id);
                            }
                        },
                        Citied(owner) => {
                            if owner == &(player_id as u32) {
                                cities.push(node.id);
                            }
                        },
                        _ => ()
                    }

                    match &node.roads {
                        Some(v_roads) => {
                            for road in v_roads {
                                if road.0 == player_id as u32 {
                                    let start_id = node.id;
                                    let end_id = road.1;

                                    if start_id < end_id {
                                        roads.push((start_id, end_id));
                                    } else {
                                        roads.push((end_id, start_id));
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                    
                }
            },
            None => (),
        };

        // remove duplicate roads
        remove_duplicates(&mut roads);



        // add actions and outcomes for the entire history
        let mut actions_and_outcomes: Vec<(u32, Option<Action>, Option<Outcome>)> = vec![];
        let mut scores: Vec<(u32, u32)> = vec![];
        let mut drawn_resources: Vec<(u32, Vec<u32>)> = vec![];
        let mut budgets: Vec<(u32, Vec<u32>)> = vec![];
        
        for entry in game.log.iter() {
            let action: Option<Action>;
            let outcome: Option<Outcome>;
            let score: u32;
            let v_drawn_resources: Vec<u32>;
            let budget: Vec<u32>;
            
            match &entry.round {
                Some(round) => {
                    if round.active_player == player_id as u32 {
                        action = round.clone().action;
                        outcome = round.clone().outcome;
                        score = round.clone().board.scores[player_id];
                        v_drawn_resources = round.clone().board.total_drawn_resources[player_id].clone();
                        budget = round.clone().board.budgets[player_id].clone();
                    
                        actions_and_outcomes.push((entry.log_id, action, outcome));
                        scores.push((entry.log_id, score));
                        drawn_resources.push((entry.log_id, v_drawn_resources));
                        budgets.push((entry.log_id, budget));
                    } 
                },
                None => {
                    ();
                },
            };

            
        }



        PlayerSummary {
            player_id,
            scores,
            actions_and_outcomes,
            budgets,
            drawn_resources,
            settlements,
            cities,
            roads,
        }
    }
}


fn remove_duplicates(vec: &mut Vec<(u32, u32)>) {
    let mut set = HashSet::new();
    vec.retain(|item| set.insert(*item));
}