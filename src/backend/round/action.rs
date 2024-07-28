use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::backend::{logging::log_entry::LogEntry, setup::{city::City, game::Game, road::Road, settlement::Settlement}};

use super::{cards::CardType, longest_road::get_longest_road, outcome::Outcome, score::{get_public_score, get_score}};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub enum Action {
   SetUpMove(u32, u32), 
   Robber(u32, u32, Option<u32>),
   DiscardCards(Vec<u32>),
   NoDiscard,
   NoCardPlay,
   CardPlay(CardType), 
   NoTrade,
   BankTrade(u32, u32),
   HarborTrade(u32, u32, u32),
   TradeQuote(Quote),
   TradeResponse(u32, bool),
   NoBuying,
   BuildRoad(u32, u32),
   BuildSettlement(u32),
   BuildCity(u32),
   BuyDevCard,
   FinishRound, 
   Save,
   Quit
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Quote {
    pub quoting_player: usize,
    pub resource_supplied: u32,
    pub quantity_supplied: u32,
    pub resource_demanded: u32,
    pub quantity_demanded: u32,
}


impl Game {
    pub fn take_action(&mut self, legal_action: Action, building_name: Option<String>) {
        
        self.log.push(LogEntry::new(self));

        let active_player = self.round.active_player as usize;

        let budget = &self.round.board.budgets[active_player];

        match legal_action.clone() {
            Action::SetUpMove(settlement_node, road_end) => {
                place_settlement(self, settlement_node, building_name);

                // println!("Tile nodes: {:?}", self.parameters.node_tiles_adjacency[settlement_node as usize]);

                place_road(self, settlement_node, road_end);
            },

            Action::Robber(i_robber, i_tile, opt_victim) => {
                // move the robber (change v_robber and the tile rng)
                let i_orig_tile = self.round.board.v_robbers[i_robber as usize] as usize;
                self.round.board.tiles[i_orig_tile].rng = self.round.board.tile_rng[i_orig_tile];
                self.round.board.tiles[i_tile as usize].rng = None;

                for robber_id in 0..self.round.board.v_robbers.len() {
                    if i_robber as usize == robber_id {
                        self.round.board.v_robbers[robber_id] = i_tile;
                    }
                }

                
                if let Some(i_victim) = opt_victim {
                    // select random resource from victim
                    let total_resources: u32 = self.round.board.budgets[i_victim as usize].iter().sum();

                    if total_resources > 0 {
                        let cum_resources: Vec<u32> = self.round.board.budgets[i_victim as usize].iter()
                                                        .scan(0_u32, |acc, &x| {
                                                            *acc += x;
                                                            Some(*acc)
                                                        })
                                                        .collect();

                        let mut rng = StdRng::seed_from_u64(self.parameters.robber_seed);
                        let random_card = rng.gen_range(0..total_resources as i32) as u32;
                        
                        self.parameters.robber_seed += 1;

                        // subtract resource from victim's budget
                        let mut resource_type = 0_usize;
                        while cum_resources[resource_type] < random_card {
                            resource_type += 1;
            
                            if cum_resources[resource_type] > random_card {
                                resource_type -= 1;
                                break
                            }
                        }
                        
                        if self.round.board.budgets[i_victim as usize][resource_type] != 0 {
                            self.round.board.budgets[i_victim as usize][resource_type] -= 1;

                            // add resource to active player's budget
                            self.round.board.budgets[active_player][resource_type] += 1;
                        }

                        self.round.outcome = Some(Outcome::RobberOutcome(Some(resource_type as u32) ));
                    } else {
                        self.round.outcome = Some(Outcome::RobberOutcome(None));
                    }
                    
                }


            },

            Action::DiscardCards(discarded) => {
                // subtract discarded cards from budget
                self.round.board.budgets[active_player] = budget.iter()
                                                            .zip(discarded.iter())
                                                            .map(|(b, &d)| b - d)
                                                            .collect();

                // discard is public. Comment out to keep it private
                self.round.board.public_budgets[active_player] = self.round.board.public_budgets[active_player].iter()
                                                                .zip(discarded.iter())
                                                                .map(|(b, &d)| b.checked_sub(d).unwrap_or(0))
                                                                .collect();
                                                                
            },

            Action::NoDiscard => {
                
            }

            Action::NoCardPlay => {
            
            },

            Action::CardPlay(card_type) => {
                // add card to public cards (legal action = only non-public cards may be played)
                let card_type_id: usize = match card_type {
                    CardType::VPCard => 0,
                    CardType::KnightCard(_,_, _) => 1,
                    CardType::RoadsCard(_, _, _, _) => 2,
                    CardType::PlentyCard(_, _) =>  3,
                    CardType::MonopolyCard(_) => 4,
                };

                self.round.board.public_dev_cards[active_player][card_type_id] += 1;

                // change board based on type of card
                match card_type {
                    CardType::VPCard => { // victory points
                        self.round.action = Some(Action::CardPlay(CardType::VPCard));
                    },

                    CardType::KnightCard(robber_id, tile_id, opt_victim) => {  // knight card
                        self.round.action = Some(Action::CardPlay(CardType::KnightCard(robber_id, tile_id, opt_victim)));
                        self.take_action(Action::Robber(robber_id, tile_id, opt_victim), None);

                    },

                    CardType::RoadsCard(f1, s1, f2, s2) => { // road card
                        self.round.action = Some(Action::CardPlay(CardType::RoadsCard(f1,s1,f2, s2)));
                        place_road(self, f1, s1);
                        place_road(self, f2, s2);
                    },

                    CardType::PlentyCard(first_resource, second_resource ) => { // year of plenty
                        self.round.board.budgets[active_player][first_resource as usize] += 1;
                        self.round.board.budgets[active_player][second_resource as usize] += 1;

                        self.round.board.public_budgets[active_player][first_resource as usize] += 1;
                        self.round.board.public_budgets[active_player][second_resource as usize] += 1;

                        self.round.board.total_drawn_resources[active_player][first_resource as usize] += 1;
                        self.round.board.total_drawn_resources[active_player][second_resource as usize] += 1;

                        self.round.action = Some(Action::CardPlay(CardType::PlentyCard(first_resource, second_resource)));
                    },

                    CardType::MonopolyCard(resource) => { // monopoly

                        self.round.action = Some(Action::CardPlay(CardType::MonopolyCard(resource)));

                        for i_player in 0..self.parameters.n_players {
                            if i_player != active_player as u32 {
                                // add the sum of all other players' budget of this resource to the active player's budget
                                self.round.board.budgets[active_player][resource as usize] += self.round.board.budgets[i_player as usize][resource as usize];
                                self.round.board.public_budgets[active_player][resource as usize] += self.round.board.budgets[i_player as usize][resource as usize];

                                // set everyone else's budget of this resource to zero
                                self.round.board.budgets[i_player as usize][resource as usize] = 0;
                                self.round.board.public_budgets[i_player as usize][resource as usize] = 0;
                            }
                        }
                    },

                }

                self.round.cards_played += 1;
            },

            Action::NoTrade => {
                self.round.action = Some(Action::NoTrade);
            },

            Action::BankTrade(r_supplied, r_demanded) => {
                

                self.round.board.budgets[active_player][r_supplied as usize] -= 4;
                self.round.board.budgets[active_player][r_demanded as usize] += 1;

                self.round.board.public_budgets[active_player][r_supplied as usize] = self.round.board.public_budgets[active_player][r_supplied as usize].checked_sub(4).unwrap_or(0);
                self.round.board.public_budgets[active_player][r_demanded as usize] += 1;
            },

            Action::HarborTrade(harbor_type, r_supplied, r_demanded) => {

                if harbor_type < self.parameters.n_resources {
                    self.round.board.budgets[active_player][r_supplied as usize] -= 2;
                    self.round.board.budgets[active_player][r_demanded as usize] += 1;

                    self.round.board.public_budgets[active_player][r_supplied as usize] = self.round.board.public_budgets[active_player][r_supplied as usize].checked_sub(2).unwrap_or(0);
                    self.round.board.public_budgets[active_player][r_demanded as usize] += 1;
                } else {
                    self.round.board.budgets[active_player][r_supplied as usize] -= 3;
                    self.round.board.budgets[active_player][r_demanded as usize] += 1;

                    self.round.board.public_budgets[active_player][r_supplied as usize] = self.round.board.public_budgets[active_player][r_supplied as usize].checked_sub(3).unwrap_or(0);
                    self.round.board.public_budgets[active_player][r_demanded as usize] += 1;
                }

                
            },

            Action::TradeQuote(quote) => {
                
            },
                                     
            Action::TradeResponse(i_player, accept)=> {
                if accept {
                    match &self.round.action {
                        Some(action) => {
                            if let Action::TradeQuote(quote) = action {
                                let quoting_player = quote.quoting_player;

                                // supply side
                                self.round.board.budgets[quoting_player][quote.resource_supplied as usize] -= quote.quantity_supplied;
                                self.round.board.budgets[active_player][quote.resource_supplied as usize] += quote.quantity_supplied;

                                self.round.board.public_budgets[quoting_player][quote.resource_supplied as usize] = self.round.board.public_budgets[quoting_player][quote.resource_supplied as usize].checked_sub(quote.quantity_supplied).unwrap_or(0);
                                self.round.board.public_budgets[active_player][quote.resource_supplied as usize] += quote.quantity_supplied;

                                // demand side
                                self.round.board.budgets[quoting_player][quote.resource_demanded as usize] += quote.quantity_demanded;
                                self.round.board.budgets[active_player][quote.resource_demanded as usize] -= quote.quantity_demanded;

                                self.round.board.public_budgets[quoting_player][quote.resource_demanded as usize] += quote.quantity_demanded;
                                self.round.board.public_budgets[active_player][quote.resource_demanded as usize] = self.round.board.public_budgets[quoting_player][quote.resource_supplied as usize].checked_sub(quote.quantity_demanded).unwrap_or(0);
                            }
                        },
                        None => (),
                    }
                    self.round.outcome = Some(Outcome::TradeOutcome(active_player as u32, true));
                } else {
                    self.round.outcome = Some(Outcome::TradeOutcome(active_player as u32, false));
                }

                
            },

            Action::NoBuying => {
                
            },

            Action::BuildRoad(start_node, end_node) => {
                place_and_pay_road(self, start_node, end_node);
            },

            Action::BuildSettlement(node_id) => {
                place_and_pay_settlement(self, node_id, building_name);
            },

            Action::BuildCity(node_id) => {
                let new_city = City {
                    name: building_name,
                    node_id,
                    player_id: active_player as u32,
                };

                let nodes = &self.round.board.nodes;

                let err_msg = format!("Could not place a city at node {:?}.", node_id);

                self.round.board.nodes = new_city.place(nodes.clone()).expect(&err_msg);

                // pay for settlement
                self.round.board.budgets[active_player] = budget.iter()
                                        .zip(self.parameters.building_costs[2].iter())
                                        .map(|(b, &c)| b - c)
                                        .collect();

                self.round.board.public_budgets[active_player] = self.round.board.public_budgets[active_player].iter()
                                        .zip(self.parameters.building_costs[2].iter())
                                        .map(|(b, &c)| b.checked_sub(c).unwrap_or(0))
                                        .collect();
            },

            Action::BuyDevCard => {
                // pay for development card
                self.round.board.budgets[active_player] = budget.iter()
                                        .zip(self.parameters.building_costs[3].iter())
                                        .map(|(b, &c)| b - c)
                                        .collect();

                self.round.board.public_budgets[active_player] = self.round.board.public_budgets[active_player].iter()
                                        .zip(self.parameters.building_costs[3].iter())
                                        .map(|(b, &c)| b.checked_sub(c).unwrap_or(0))
                                        .collect();

                // determine type of development card
                let undrawn_cards = &self.round.board.undrawn_dev_cards;
                let cum_undrawn_cards: &Vec<u32> = &undrawn_cards.iter()
                                                    .scan(0_u32, |acc, &x| {
                                                        *acc += x;
                                                        Some(*acc)
                                                    })
                                                    .collect();
                let total_undrawn_cards: u32 = cum_undrawn_cards[cum_undrawn_cards.len()-1];

                let mut rng = StdRng::seed_from_u64(self.parameters.dev_card_seed);
                let random_card = rng.gen_range(0..total_undrawn_cards as i32) as u32;

                // increase seed
                self.parameters.dev_card_seed += 1;
                self.round.card_count += 1;
                                                    
                let mut card_type = 0_usize;
                while cum_undrawn_cards[card_type] < random_card {
                    card_type += 1;

                    if cum_undrawn_cards[card_type] > random_card {
                        card_type -= 1;
                        break
                    }
                }
                
                // add development card to "hand"
                self.round.board.drawn_dev_cards[active_player][card_type] += 1;

                self.round.outcome = Some(Outcome::DrawCardOutcome(card_type as u32));

            },

            Action::FinishRound => {
                
            },

            Action::Save => {
                self.encode_log().expect("Encoding of log failed.");
            },

            Action::Quit => {
                unimplemented!()
            },
        }

        // add action to round (unless it is a response)
        match &legal_action {
            Action::TradeResponse(_,_) => (),
            _ => self.round.action = Some(legal_action.clone())
        };

        // println!("Refreshing board.");
        self.refresh_board();  

        // println!("Closing log.");
        let log_length = self.log.len();
        let log_game = self.clone();
        LogEntry::close(&mut self.log[log_length-1], &log_game);

        // println!("Iterating phase.");
        self.iterate_phase(&legal_action);  

    }


    fn refresh_board(&mut self) {

        let roads = &self.round.board.roads;

        // update longest roads
        self.round.board.longest_roads = match &roads {
            Some(v_roads) => {
                let v_longest = (0..self.parameters.n_players)
                                    .map(|i_player| get_longest_road(i_player, v_roads)).collect();
                Some(v_longest)
            },
            None => {
                None
            },
        };
        // println!("longest road updated.");

        let nodes = &self.round.board.nodes;
        let drawn_dev_cards = &self.round.board.drawn_dev_cards;
        let public_dev_cards = &self.round.board.public_dev_cards;
        let longest_roads = &self.round.board.longest_roads;
        let prev_longest_road = &self.round.board.prev_longest_road;
        let armies = &self.round.board.armies;
        let prev_largest_army = &self.round.board.prev_largest_army;

        // update scores
        self.round.board.scores = (0..self.parameters.n_players)
                                .map(|i_player| get_score(i_player, &nodes,  &drawn_dev_cards, &longest_roads, &prev_longest_road,
                                &armies, &prev_largest_army))
                                .collect();

        self.round.board.public_scores = (0..self.parameters.n_players)
                                .map(|i_player| get_public_score(i_player, &nodes, &public_dev_cards, &longest_roads, &prev_longest_road,
                                &armies, &prev_largest_army))
                                .collect();
        // println!("updated score.");
        
        // update previous longest road (if longest road has changed)
        match &self.round.board.longest_roads {
            Some(longest_roads) => {
                match longest_roads.iter().enumerate().max_by_key(|&(_, &val)| val) {
                    Some((index, &max_value)) => { 
                        if max_value > 4 {
                        match &self.round.board.prev_longest_road {
                            Some(prev_longest_road) => {
                                if max_value != prev_longest_road.1 {
                                    self.round.board.prev_longest_road = Some((index as u32, max_value));
                                }
                            },
                            None => {
                                self.round.board.prev_longest_road = Some((index as u32, max_value));
                            },
                        }
                        }
    
                    }
                    None => {
                                    self.round.board.prev_longest_road = None;
                                }
                }
            },
            None => {
                self.round.board.prev_longest_road = None;
            },
        }

        // println!("prev longest road updated");


        // update previous largest army (if largest army has changed)
        if let Some((index, largest_army)) = armies.iter().enumerate().max_by_key(|&(_, &val)| val) {
            if largest_army > &2 {
                match &prev_largest_army {
                    Some(pla) => {
                        if largest_army != &pla.1 {
                            self.round.board.prev_largest_army = Some((index as u32, *largest_army));
                        }
                    },
                    None => {
                        self.round.board.prev_largest_army = Some((index as u32, *largest_army));
                    }
                        ,
                }
            }
        } else {
            self.round.board.prev_largest_army = None;
        };

        // println!("prev largest army updated");



    }
}


fn place_and_pay_road(game: &mut Game, start_node: u32, end_node: u32) {
    place_road(game, start_node, end_node);

    pay_road(game);
}

fn place_road(game: &mut Game, start_node: u32, end_node: u32) {
    let active_player = game.round.active_player as usize;

    let new_road = Road{
        player: active_player as u32,
        nodes: (start_node, end_node),
     };

    //  println!("In road function");
    let nodes = &game.round.board.nodes;

    let err_msg = format!("Could not place a road at ({:?}, {:?}).", start_node, end_node);

    game.round.board.nodes = new_road.clone().place(nodes.clone()).expect(&err_msg);

    // also add roads to the ``central'' roads vector
    match game.round.board.roads {
        Some(ref mut roads) => {
            roads.push(new_road);
        },
        None => {
            game.round.board.roads = Some(vec![new_road]);
        },
    }
}

fn pay_road(game: &mut Game) {
    let active_player = game.round.active_player as usize;

    let budget = &game.round.board.budgets[active_player];

    // pay for road
    game.round.board.budgets[active_player] = budget.iter()
                        .zip(game.parameters.building_costs[0].iter())
                        .map(|(b, &c)| b - c)
                        .collect();

    game.round.board.public_budgets[active_player] = game.round.board.public_budgets[active_player].iter()
                        .zip(game.parameters.building_costs[0].iter())
                        .map(|(b, &c)| b.checked_sub(c).unwrap_or(0))
                        .collect();
}

fn place_and_pay_settlement(game:&mut Game, node_id: u32, building_name: Option<String>) {
    place_settlement(game, node_id, building_name);

    pay_settlement(game);
}

fn place_settlement(game: &mut Game, node_id: u32, building_name: Option<String>) {
    let active_player = game.round.active_player as usize;
    
    let new_settlement = Settlement {
        name: building_name,
        node_id,
        player_id: active_player as u32,
    };

    let nodes = &game.round.board.nodes;

    let err_msg = format!("Could not place a settlement at node {:?}.", node_id);

    game.round.board.nodes = new_settlement.place(nodes.clone()).expect(&err_msg);
}

fn pay_settlement(game: &mut Game) {
    let active_player = game.round.active_player as usize;

    let budget = &game.round.board.budgets[active_player];

    // pay for settlement
    game.round.board.budgets[active_player] = budget.iter()
                .zip(game.parameters.building_costs[1].iter())
                .map(|(b, &c)| b - c)
                .collect();

    game.round.board.public_budgets[active_player] = game.round.board.public_budgets[active_player].iter()
                .zip(game.parameters.building_costs[1].iter())
                .map(|(b, &c)| b.checked_sub(c).unwrap_or(0))
                .collect();
}   