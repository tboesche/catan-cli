use std::error::Error;

use crate::backend::{logging::log_entry::LogEntry, round::{action::Action, cards::CardType, outcome::Outcome, phase::Phase}, setup::{edge::make_edge, game::Game, node_status::{self, NodeStatus}}};

impl Game {
    pub fn encode_log(&self) -> Result<(), Box<dyn Error>> {

        let file_path = "data/saves/".to_string() + &self.parameters.title.clone().unwrap_or("untitled".to_string());

        self.hot_encode_round(&file_path)?;

        Ok(())
        
    }

    pub fn hot_encode_log(&self, log_entry: &LogEntry) -> Vec<u32> {

        let mut v_out: Vec<u32> = vec![];

        let final_scores: Vec<u32>;
        let length_log = self.log.len();

        if length_log > 1 {
            if let Some(round) = &self.log[length_log - 2].round {
                final_scores = round.board.scores.clone();
            } else {
                final_scores = vec![0; self.parameters.n_players as usize];
            }
        } else {
            final_scores = vec![0; self.parameters.n_players as usize];
        }

        if let Some(round) = &log_entry.round {
            let active_player = round.active_player as usize;
            
            let mut row = vec![
                final_scores[active_player],        // final score
                round.board.scores[active_player], // current score
                round.phase_count,
                round.card_count,
                round.card_count,
                round.robber_count,
                log_entry.count_dice_draws,
            ]; // 7 fields

            for i_player in 0..self.parameters.n_players {
                
                if i_player == active_player as u32 { // active player
                    row.push(1);
                } else {
                    row.push(0);
                }

                if i_player == round.throwing_player {  // throwing player
                    row.push(1);
                } else {
                    row.push(0);
                }

                if let Some(lr) = round.board.prev_longest_road { // prev longest road
                    if i_player == lr.0 {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                } else {
                    row.push(0);
                }
            } // 3 * n_players fields

            // dice outcome
            let min_dice = self.parameters.n_dice;
            let max_dice = self.parameters.n_dice * self.parameters.n_faces;
            let n_dice_outcomes = max_dice - min_dice + 1;

            for i_dice in min_dice..= max_dice {

                if let Some(dice_out) = round.board.dice_outcome {
                    if dice_out == i_dice {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                } else {
                    row.push(0);
                }
            } // n_dice_outcomes fields


            // phase
            let n_phase = 9;

            let current_phase = match &round.phase {
                Phase::SetUp => 0,
                Phase::RobberDiscard => 1,
                Phase::RobberMove => 2,
                Phase::FirstCardPhase => 3,
                Phase::TradingQuote => 4,
                Phase::TradingResponse => 5,
                Phase::Building => 6,
                Phase::SecondCardPhase => 7,
                Phase::Terminal => 8,
            };

            for i_phase in 0..n_phase {
                if current_phase == i_phase {
                    row.push(1);
                } else {
                    row.push(0);
                }
            } // 9 fields

            // action_type
            let n_actions = 19 as usize;
            let n_nodes = round.board.nodes.len();
            let n_robbers = round.board.v_robbers.len();
            let n_tiles = round.board.tiles.len();
            let n_players = self.parameters.n_players as usize;
            let n_resources = self.parameters.n_resources as usize;
            let n_harbor_types = n_resources + 1;
            
            let n_action_fields = n_actions + 9 * n_nodes + n_robbers + n_tiles + n_players + 10 * n_resources + n_harbor_types + 3;

            match &round.action {
                Some(action) => {
                    match &action {
                        Action::SetUpMove(i_settle, i_road) => {
                            for i_action in 0..n_actions {
                                if i_action == 0 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            for i_node in 0..n_nodes as u32 {
                                if i_node == *i_settle {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            for i_node in 0..n_nodes as u32 {
                                if i_node == *i_road {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions - 2 * n_nodes])
                        },
                        Action::Robber(i_robber, i_tile, i_victim) => {
                            for i_action in 0..n_actions {
                                if i_action == 1 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            // i_settle and i_road
                            row.extend(vec![0; 2 * n_nodes]);

                            // i_robber
                            for ir in 0..n_robbers as u32 {
                                if ir == *i_robber {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }
                            }

                            // i_tile
                            for it in 0..n_tiles as u32 {
                                if it == *i_tile {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }
                            }

                            // i_victim 
                            if let Some(victim) = *i_victim {
                                for iv in 0..n_players as u32 {
                                    if iv == victim {
                                        row.push(1);
                                    } else {
                                        row.push(0);
                                    }
                                }
                            } else {
                                row.extend(vec![0; n_players]);
                            }
                            
                            row.extend(vec![0; n_action_fields - n_actions - 2 * n_nodes - n_robbers - n_tiles - n_players])
                        },
                        Action::DiscardCards(v_discard) => {
                            for i_action in 0..n_actions {
                                if i_action == 2 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_resources - n_actions]);

                            row.extend(v_discard);
                        },
                        Action::NoDiscard => {
                            for i_action in 0..n_actions {
                                if i_action == 3 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::NoCardPlay => {
                            for i_action in 0..n_actions {
                                if i_action == 4 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::CardPlay(card) => {
                            for i_action in 0..n_actions {
                                if i_action == 5 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            match &card {
                                CardType::VPCard => {
                                    row.extend(vec![0; n_action_fields - n_actions]); 
                                },
                                CardType::KnightCard(i_robber, i_tile, i_victim) => {
                                    row.extend(vec![0; 2 * n_nodes]);

                                    // i_robber
                                    for ir in 0..n_robbers as u32 {
                                        if ir == *i_robber {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }
                                    }

                                    // i_tile
                                    for it in 0..n_tiles as u32 {
                                        if it == *i_tile {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }
                                    }

                                    // i_victim 
                                    if let Some(victim) = *i_victim {
                                        for iv in 0..n_players as u32 {
                                            if iv == victim {
                                                row.push(1);
                                            } else {
                                                row.push(0);
                                            }
                                        }
                                    } else {
                                        row.extend(vec![0; n_players]);
                                    }
                                    
                                    row.extend(vec![0; n_action_fields - n_actions - 2 * n_nodes - n_robbers - n_tiles - n_players])
                                },
                                CardType::RoadsCard(s1, e1, s2, e2) => {
                                    row.extend(vec![0; 2 * n_nodes + n_robbers + n_tiles + n_players]);

                                    // s1, e1, s2, e2 for each node
                                    for i_node in 0..n_nodes as u32 {
                                        if i_node == *s1 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }

                                        if i_node == *e1 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }

                                        if i_node == *s2 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }

                                        if i_node == *e2 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }
                                    }

                                    row.extend(vec![0; n_action_fields - n_actions - (6 * n_nodes + n_robbers + n_tiles + n_players)])
                                },
                                CardType::PlentyCard(r1, r2) => {
                                    row.extend(vec![0; 6 * n_nodes + n_robbers + n_tiles + n_players]);

                                    for i_resource in 0..n_resources as u32 {
                                        if i_resource == *r1 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }

                                        if i_resource == *r2 {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }
                                    }

                                    row.extend(vec![0; n_action_fields - n_actions - (6 * n_nodes + n_robbers + n_tiles + n_players + 2 * n_resources)])
                                },
                                CardType::MonopolyCard(resource_monopoly) => {
                                    row.extend(vec![0; 6 * n_nodes + n_robbers + n_tiles + n_players + 2 * n_resources]);

                                    for i_resource in 0..n_resources as u32 {
                                        if i_resource == *resource_monopoly {
                                            row.push(1);
                                        } else {
                                            row.push(0);
                                        }
                                    }

                                    row.extend(vec![0; n_action_fields - n_actions - (6 * n_nodes + n_robbers + n_tiles + n_players + 3 * n_resources)])
                                },
                            }
                        },
                        Action::NoTrade => {
                            for i_action in 0..n_actions {
                                if i_action == 6 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::BankTrade(rs, rd) => {
                            for i_action in 0..n_actions {
                                if i_action == 7 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 6 * n_nodes + n_robbers + n_tiles + n_players + 3 * n_resources]);

                            for i_resource in 0..n_resources as u32 {
                                if *rs == i_resource {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }

                                if *rd == i_resource {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions - (6 * n_nodes + n_robbers + n_tiles + n_players + 5 * n_resources)])
                        },
                        Action::HarborTrade(harbor_type, rs, rd) => {
                            for i_action in 0..n_actions {
                                if i_action == 8 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 6 * n_nodes + n_robbers + n_tiles + n_players + 5 * n_resources]);

                            for i_type in 0..n_harbor_types as u32 {
                                if i_type == *harbor_type {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }
                            }

                            for i_resource in 0..n_resources as u32 {
                                if *rs == i_resource {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }

                                if *rd == i_resource {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions - (6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 7 * n_resources)])

                        },
                        Action::TradeQuote(quote) => {
                            for i_action in 0..n_actions {
                                if i_action == 9 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 7 * n_resources]);

                            for i_resource in 0..n_resources as u32 {
                                if quote.resource_supplied == i_resource {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }
                            }

                            row.push(quote.quantity_supplied);

                            for i_resource in 0..n_resources as u32 {
                                if quote.resource_demanded == i_resource {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }
                            }

                            row.push(quote.quantity_demanded);

                            if let Some(Outcome::TradeOutcome(_, accept)) = round.outcome {
                                row.push((accept as u32));
                             } else {
                                 row.push(0)
                             }

                            row.extend(vec![0; n_action_fields - n_actions - (3 + 6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources)]);
                        },
                        Action::TradeResponse(_, accept) => {
                            for i_action in 0..n_actions {
                                if i_action == 10 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 2 + 6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources]);

                            row.push(*accept as u32);


                            row.extend(vec![0; n_action_fields - n_actions - (3 + 6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources)])
                        },
                        Action::NoBuying => {
                            for i_action in 0..n_actions {
                                if i_action == 11 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::BuildRoad(s1, e1) => {
                            for i_action in 0..n_actions {
                                if i_action == 12 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 3 + 6 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources]);

                            for i_node in 0..n_nodes as u32 {
                                if *s1 == i_node {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }

                                if *e1 == i_node {
                                    row.push(1);
                                } else {
                                    row.push(0);
                                }  
                            }

                            row.extend(vec![0; n_action_fields - n_actions - (3 + 8 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources)])
                        },
                        Action::BuildSettlement(settle_node) => {
                            for i_action in 0..n_actions {
                                if i_action == 13 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 3 + 8 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources]);

                            for i_node in 0..n_nodes as u32 {
                                if *settle_node == i_node {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0;  n_action_fields - n_actions - (3 + 9 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources)])
                        },
                        Action::BuildCity(city_node) => {
                            for i_action in 0..n_actions {
                                if i_action == 14 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; 3 + 8 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources]);

                            for i_node in 0..n_nodes as u32 {
                                if *city_node == i_node {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0;  n_action_fields - n_actions - (3 + 9 * n_nodes + n_harbor_types + n_robbers + n_tiles + n_players + 9 * n_resources)])
                        },
                        Action::BuyDevCard => {
                            for i_action in 0..n_actions {
                                if i_action == 15 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::FinishRound => {
                            for i_action in 0..n_actions {
                                if i_action == 16 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::Save => {
                            for i_action in 0..n_actions {
                                if i_action == 17 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                        Action::Quit => {
                            for i_action in 0..n_actions {
                                if i_action == 18 {
                                    row.push(1)
                                } else {
                                    row.push(0)
                                }
                            }

                            row.extend(vec![0; n_action_fields - n_actions]);
                        },
                    }
                },
                None => {
                    row.extend(vec![0; n_action_fields])
                },
            }

            // budget of active player
            for i_resource in 0..n_resources {
                row.push(round.board.budgets[active_player][i_resource]);
            }

            // drawn dev cards of active player 
            for i_card in 0..self.parameters.n_dev_card_types {
                row.push(round.board.drawn_dev_cards[active_player][i_card]);
            }

            // for each player: drawn_resources, public budget, public cards, longest_road
            for i_player in 0..n_players {
                for i in 0..n_resources {
                    row.push(round.board.total_drawn_resources[i_player][i]);
                }

                for i in 0..n_resources {
                    row.push(round.board.public_budgets[i_player][i]);
                }

                for i_card in 0..self.parameters.n_dev_card_types {
                    row.push(round.board.public_dev_cards[i_player][i_card]);
                }

                if let Some(lr) = &round.board.longest_roads {
                    row.push(lr[i_player])
                } else {
                    row.push(0)
                }
            }

            // for each node: status and owner
            let n_node_stati = 4;
            for i_node in 0..n_nodes {
                for i_status in 0..n_node_stati {
                    let node_status = match round.board.nodes[i_node].node_status {
                        NodeStatus::Free => 0,
                        NodeStatus::Adjacent => 1,
                        NodeStatus::Settled(_) => 2,
                        NodeStatus::Citied(_) => 3,
                    };

                    if i_status == node_status {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                }

                let i_owner = match round.board.nodes[i_node].node_status {
                    NodeStatus::Settled(owner) => owner,
                    NodeStatus::Citied(owner) => owner,
                    _ => n_players as u32,
                };

                for i_player in 0..n_players as u32 {
                    if i_player == i_owner {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                }
            } // 4 * n_nodes + n_players * n_nodes

            let n_edges = self.parameters.edge_map.len();
            for i_edge in 0..n_edges {
                if let Some(roads) = &round.board.roads {
                    let mut matched = false;

                        for road in roads {
                            let edge = make_edge(road.nodes.0, road.nodes.1);
                            
                            if let Some(index) = self.parameters.edge_map.get(&edge) {
                                if index == &i_edge {
                                    
                                    for i_player in 0..n_players as u32 {
                                        if i_player == road.player {
                                            row.push(1)
                                        } else {
                                            row.push(0)
                                        }
                                    }

                                    matched = true;
                                    break
                                }
                            }
                        }

                        if !matched {
                            row.extend(vec![0; n_players]);
                        }  
                } else {
                    row.extend(vec![0; n_players]);
                } // n_playerss


                // harbors 
                if let Some(harbors) = &round.board.harbors {

                    let mut matched = false; 

                    for harbor in harbors {
                        let edge = make_edge(harbor.nodes.0, harbor.nodes.1);

                        if let Some(index) = self.parameters.edge_map.get(&edge) {
                            if index == &i_edge {

                                for i_type in 0..n_harbor_types as u32 {
                                    if i_type == harbor.harbor_type {
                                        row.push(1)
                                    } else {
                                        row.push(0)
                                    }
                                }

                                for i_player in 0..n_players as u32 {
                                    if let Some(i) = harbor.player {
                                        if i == i_player {
                                            row.push(1)
                                        } else {
                                            row.push(0)
                                        }
                                    } else {
                                        row.push(0)
                                    }
                                }

                                matched = true;
                                break
                            }
                        }

                    }

                    if !matched {
                        row.extend(vec![0; n_harbor_types + n_players]);
                    }

                } else {
                    row.extend(vec![0; n_harbor_types + n_players]);
                } // n_harbor_types + n_players
            } // n_edges * (2* n_players + n_harbor_types)


            for i_robber in &round.board.v_robbers {
                for i_tile in 0..n_tiles as u32 {
                    if i_tile == *i_robber {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                }
            } // 

            for tile in &round.board.tiles {
                if let Some(tile_rng) = tile.rng {
                    for i_dice in 0..n_dice_outcomes {
                        if i_dice == tile_rng {
                            row.push(1)
                        } else {
                            row.push(0)
                        }
                    }
                } else {
                    row.extend(vec![0; n_dice_outcomes as usize])
                }
            } // n_tiles * n_dice_outcomes

            v_out = row;
        }
        
        v_out

    }
}

