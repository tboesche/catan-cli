use crate::backend::{round::phase::Phase::{Building, FirstCardPhase, RobberDiscard, RobberMove, SetUp, TradingQuote, TradingResponse, SecondCardPhase, Terminal}, setup::{game::Game, node_status::NodeStatus, player::PlayerType}};

use super::{action::{Action, Quote}, cards::CardType};

impl Game {
    pub fn get_legal_actions(&self) ->Vec<Action> {

        let board = &self.round.board;
        let active_player: usize = self.round.active_player as usize;
        let budget = &self.round.board.budgets[active_player];

        let mut legal_actions: Vec<Action> = vec![];

        match &self.round.phase {

            SetUp => {
                if self.round.phase_count / self.parameters.n_players < self.parameters.n_setup_rounds {
                    
                    for node in &board.nodes {
                        if node.node_status == NodeStatus::Free {
                            let node_id = node.id;

                            for neighbor in &node.neighbors {
                                match &neighbor {
                                    Some(n) => {
                                        legal_actions.push(Action::SetUpMove(node_id, *n))
                                    },
                                    None => (),
                                }
                            }
                        }
                    }

                } else { // if all setup rounds have already happened, can only end the setup
                    legal_actions.push(Action::FinishRound);
                }
            },
            
            RobberDiscard => {
                let sum_budget = &budget.iter().sum::<u32>();
                if sum_budget > &7 {
                    // find all combinations of resources which are within budget and add up to half of all the player's resourcess
                    let possible_discards = enumerate_discards(budget);

                    // add each combination as a legal action
                    for v_discard in possible_discards {
                        legal_actions.push(Action::DiscardCards(v_discard));
                    }
                } else {
                    legal_actions.push(Action::NoDiscard);
                }
            },

            RobberMove => {
                legal_actions = get_legal_robber_moves(self, legal_actions)
            },

            FirstCardPhase => {
                legal_actions = get_legal_cards(self, legal_actions)      
            },

            TradingQuote => {
                legal_actions.push(Action::NoTrade);

                let n_resources = &self.parameters.n_resources;

                for (i_resource, b) in budget.iter().enumerate() {
                    // check whether resource can be traded with the bank
                    if b >= &4 {
                        for resource_demanded in 0..*n_resources {
                            legal_actions.push(Action::BankTrade(i_resource as u32, resource_demanded));
                        }
                    }

                    // interplayer trades
                    if b <= &3 {
                        for q_supplied in 0..=*b {
                            for q_demanded in 1..=3_u32 {
                                for r_demanded in 0..*n_resources {
                                    let quote = Quote {
                                        quoting_player: active_player,
                                        resource_supplied: i_resource as u32,
                                        quantity_supplied: q_supplied,
                                        resource_demanded: r_demanded,
                                        quantity_demanded: q_demanded,
                                    };
                                    legal_actions.push(Action::TradeQuote(quote));
                                }
                            }
                        }
                    } else {
                        for q_supplied in 0..=3 {
                            for q_demanded in 1..=3_u32 {
                                for r_demanded in 0..*n_resources {
                                    let quote = Quote {
                                        quoting_player: active_player,
                                        resource_supplied: i_resource as u32,
                                        quantity_supplied: q_supplied,
                                        resource_demanded: r_demanded,
                                        quantity_demanded: q_demanded,
                                    };
                                    legal_actions.push(Action::TradeQuote(quote));
                                }
                            }
                        }
                    }
                    
                }


                // harbor trades
                for node in &self.round.board.nodes {
                    match &node.harbor {
                        Some(harbor) => {
                            match &harbor.player {
                                Some(i_player) => {
                                    if i_player == &(active_player as u32) {
                                        if harbor.harbor_type < *n_resources {
                                            if self.round.board.budgets[active_player][harbor.harbor_type as usize] >= 2 { // check whether the player has enough resources
                                                for r_demanded in 0..*n_resources {
                                                    legal_actions.push(Action::HarborTrade(harbor.harbor_type, harbor.harbor_type, r_demanded));
                                                }
                                            }

                                        } else {
                                            for r_supplied in 0..*n_resources {
                                                if self.round.board.budgets[active_player][r_supplied as usize] >= 3 {
                                                    for r_demanded in 0..*n_resources {
                                                        legal_actions.push(Action::HarborTrade(harbor.harbor_type, r_supplied, r_demanded))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                None => continue,
                            }
                        },
                        None => continue,
                    }
                }


            },

            TradingResponse => {

                legal_actions.push(Action::TradeResponse(active_player as u32, false));

                match &self.round.action {
                    Some(action) => {
                        if let Action::TradeQuote(quote) = action {
                            let trade_legal = self.round.board.budgets[active_player][quote.resource_demanded as usize] >= quote.quantity_demanded;

                            if trade_legal {
                                legal_actions.push(Action::TradeResponse(active_player as u32, true));
                            }
                        }
                    },

                    None => (),
                }

            },

            Building => {
                legal_actions.push(Action::NoBuying); // do not build

                let building_costs = &self.parameters.building_costs;

                // build road
                let road_affordable = budget.iter()
                                            .zip(&building_costs[0])
                                            .all(|(b, c)| b >= c);

                if road_affordable { // check affordability
                    legal_actions = get_legal_roads(self, legal_actions);
                }

                // build settlement
                let settlement_affordable = budget.iter()
                                                        .zip(&building_costs[1])
                                                        .all(|(b,c)| b >= c);

                if settlement_affordable { // check affordability
                    for node in &board.nodes {
                        if node.node_status == NodeStatus::Free { // a new settlement only legal if node is free (not settled or adjacent to settled)
                            let mut player_road = false;    // a new settlement needs to be attached to an existing road
                            match &node.roads {         
                                Some(v_roads) => {
                                    for (player, _) in v_roads {
                                        if active_player == *player as usize {
                                            player_road = true;
                                            break
                                        }
                                    }
                                },
                                None => continue,
                            }
                            
                            if player_road {
                                legal_actions.push(Action::BuildSettlement(node.id));
                            }
                        }
                    }
                }
                    
                // build city
                let city_affordable = budget.iter()
                                                .zip(&building_costs[2])
                                                .all(|(b,c)| b >= c);
                if city_affordable { // check affordability
                    for node in &board.nodes {
                        if node.node_status == NodeStatus::Settled(active_player as u32) { // new city only legal if the node has a settlement by the same player
                            legal_actions.push(Action::BuildCity(node.id));
                        }
                    }
                }

                // buy development card
                let dev_card_affordable = budget.iter()
                                                .zip(&building_costs[3])
                                                .all(|(b,c)| b >= c);
                
                if dev_card_affordable { // check affordability
                    legal_actions.push(Action::BuyDevCard);
                }

            },
            
            SecondCardPhase => {
                legal_actions = get_legal_cards(self, legal_actions)
            },

            Terminal => {
                legal_actions = vec![];
            },
        }

        match self.parameters.v_players[active_player].player_type {
            PlayerType::Human => {
                legal_actions.push(Action::Save);
                legal_actions.push(Action::Quit);
            },
            _ => (),
        }


        legal_actions
    }
}



// enumerate all ``hands'' which add up to half of all resources and ``fit'' into the current budget
// algorithm: recursive backtrack
fn enumerate_discards(budget: &Vec<u32>) -> Vec<Vec<u32>> {

    let target_sum = budget.iter().sum::<u32>() / 2;
    let mut results = Vec::new();
    let mut current = vec![0; budget.len()];

    fn backtrack(
        budget: &[u32],
        target_sum: u32,
        index: usize,
        current_sum: u32,
        current: &mut Vec<u32>,
        results: &mut Vec<Vec<u32>>,
    ) {
        if index == budget.len() {
            if current_sum == target_sum {
                results.push(current.clone());
            }
            return;
        }

        for i in 0..=budget[index] {
            current[index] = i;
            backtrack(budget, target_sum, index + 1, current_sum + i, current, results);
        }
    }

    backtrack(&budget, target_sum, 0, 0, &mut current, &mut results);

    results
}

fn get_legal_robber_moves(game: &Game, mut legal_actions: Vec<Action>) -> Vec<Action> {

    let board = &game.round.board;
    let active_player: usize = game.round.active_player as usize;

    let tiles = &board.tiles;

    for (i_tile, tile) in tiles.iter().enumerate() {
        let mut is_robber_tile = false;
        for robber_tile in &board.v_robbers {
            if (i_tile as u32 == *robber_tile) || (tile.rng == None) {
                is_robber_tile = true;
                break;
            }
        }

        if is_robber_tile {
            continue
        }

        // find adjacent players: 
        let mut adjacent_players: Vec<u32> = vec![];
        for i_node in &tile.nodes {
            let node = &board.nodes[*i_node as usize];

            match &node.node_status {
                NodeStatus::Free => continue,
                NodeStatus::Adjacent => continue,
                NodeStatus::Settled(i_player) => {
                    if *i_player != active_player as u32 {
                        adjacent_players.push(*i_player)
                    }
                },
                NodeStatus::Citied(i_player) => {
                    if *i_player != active_player as u32 {
                        adjacent_players.push(*i_player)
                    }
                },
            }
        }

        for robber_id in 0..game.round.board.v_robbers.len() {
            if adjacent_players.is_empty() {
                legal_actions.push(Action::Robber(robber_id as u32, i_tile as u32, None))
            } else {
                for player in &adjacent_players {
                    legal_actions.push(Action::Robber(robber_id as u32, i_tile as u32, Some(*player)))
                }
            }
        }
            

    }


    legal_actions
}

fn get_legal_roads(game: &Game, mut legal_actions: Vec<Action>) -> Vec<Action> {
    
    let active_player: usize = game.round.active_player as usize;
    
    for first_node in &game.round.board.nodes { // find initial nodes for road

        let player_settled = match &first_node.node_status { // check whether player has an adjacent settlement or city
            NodeStatus::Free => false,
            NodeStatus::Adjacent => false,
            NodeStatus::Settled(id) => {
                if *id == active_player as u32 {
                    true
                } else {
                    false
                }
            },
            NodeStatus::Citied(id) => {
                if *id == active_player as u32 {
                    true
                } else {
                    false
                }
            },
        };

        let player_road = match &first_node.roads { // check whether player has a road to initial node
            Some(v_roads) => {
                let mut output: bool = false;
                for road in v_roads {
                    if road.0 == active_player as u32 { // only if its own road
                        output = true;
                        break;
                    } 
                }

                output
            },
            None => false,
        };


        if player_settled || player_road {  // if settled or owned road
            for neighbor_id in &first_node.neighbors { // check potential end points for road
                match neighbor_id {
                    Some(n_id) => {
                        let road_built = match &first_node.roads {
                            Some(v_roads) => {
                                let mut output = false;
                                for road in v_roads {
                                    if road.1 == n_id.clone() { // check whether a road to this neighbor already exists (regardless of owner)
                                        output = true
                                    }
                                }
                                output
                            },
                            None => false,
                        };

                        if !road_built { // if no road between first node and neighbor, new road is legal
                            legal_actions.push(Action::BuildRoad(first_node.id, *n_id))
                        }
                    },
                    None => continue,
                }
                
            }
        }


    }

    legal_actions
}

fn get_legal_cards(game: &Game, mut legal_actions: Vec<Action>) -> Vec<Action> {
    
    legal_actions.push(Action::NoCardPlay);

    let active_player: usize = game.round.active_player as usize;

    if game.round.cards_played < game.parameters.max_cards {
        let private_card_deck: &Vec<u32> = &game.round.board.drawn_dev_cards[active_player].iter()
                                        .zip(&game.round.board.public_dev_cards[active_player])
                                        .map(|(drawn, public)| drawn - public)
                                        .collect();

        // let n_card_types = private_card_deck.len();

        for (card_type, stock) in private_card_deck.iter().enumerate() {
            if stock > &0 {
                match card_type as u32 {
                    0 => { // victory card
                        legal_actions.push(Action::CardPlay(CardType::VPCard));
                    },

                    1 => { // knight card
                        let mut robber_moves: Vec<Action> = vec![];
                        robber_moves = get_legal_robber_moves(game, robber_moves);

                        for action in robber_moves {
                            match action {
                                Action::Robber(i_robber, i_tile, i_victim) => {
                                    legal_actions.push(Action::CardPlay(CardType::KnightCard(i_robber, i_tile, i_victim)));
                                },
                                _ => (),
                            }
                        }
                    },

                    2 => { // road card (build two free roads)
                        let mut legal_roads: Vec<Action> = vec![];
                        legal_roads = get_legal_roads(game, legal_roads);

                        for first_road in &legal_roads {
                            for second_road in &legal_roads {
                                if first_road != second_road {
                                    let mut first_first_node: u32 = 0;
                                    let mut first_second_node: u32 = 0;
                                    match first_road {
                                        Action::BuildRoad(f,s) => {
                                            first_first_node = *f;
                                            first_second_node = *s;
                                        },
                                        _ => ()
                                    }

                                    let mut second_first_node: u32 = 0;
                                    let mut second_second_node: u32 = 0;
                                    match second_road {
                                        Action::BuildRoad(f,s) => {
                                            second_first_node = *f;
                                            second_second_node = *s;
                                        },
                                        _ => ()
                                    }

                                    // check whether the two roads are not the inverse of each other
                                    if (first_first_node != second_second_node) & (first_second_node != second_first_node) {
                                        legal_actions.push(Action::CardPlay(CardType::RoadsCard(first_first_node, first_second_node, second_first_node, second_second_node)));
                                    }
                                }
                            }
                            
                        }
                    },

                    3 => { // year of plenty (2 resource cards)
                        for first_resource in 0..game.parameters.n_players {
                            for second_resource in 0..game.parameters.n_players {
                                legal_actions.push(Action::CardPlay(CardType::PlentyCard(first_resource, second_resource)));
                            }
                        }
                    }

                    4 => { // monopoly (1 resource)
                        for resource in 0..game.parameters.n_players {
                            legal_actions.push(Action::CardPlay(CardType::MonopolyCard(resource)));
                        }
                    }

                    _ => unimplemented!()
                }
                
            }
        }
    }

    legal_actions
}

