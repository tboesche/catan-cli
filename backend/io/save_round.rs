use std::{error::Error, fs::{File, OpenOptions}, io::{BufReader, Read}, path::Path};

use csv::Writer;

use crate::backend::{round::{action::Action, cards::CardType, outcome::Outcome, phase::Phase}, setup::{edge::make_edge, game::Game, node_status::NodeStatus, shape::get_n_tiles}};


impl Game {

    pub fn hot_encode_round(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        let (mut wtr, _) = open_writer(file_path, &format!("{}.csv",self.parameters.id))?;

        
        for i_log in 0..self.log.len() {
            if let Some(round) = &self.log[i_log].round {
                let mut row = vec![
                    i_log.to_string(), 
                ];

                row.extend(self.hot_encode_log(&self.log[i_log]).iter().map(|d| d.to_string()));
                // write row
                wtr.write_record(row)?;

            }
        }
        
        wtr.flush()?;
        
        Ok(())
    }



    pub fn encode_round(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        let (mut wtr, file_empty) = open_writer(file_path, &format!("{}.csv",self.parameters.id))?;

        if file_empty {
            let mut headers = vec![
                "log_id".to_string(),
                "active_player".to_string(),            //hot (n_players) [but add max_score]
                "score".to_string(),
                "throwing_player".to_string(),          // hot (n_players)
                "cards_played".to_string(),
                "dice_outcome".to_string(),             // hot (n_dice_outcomes = n_faces to n_faces*n_dice)
                "phase".to_string(),                    // phase, hot (n_phases)
                "phase_count".to_string(),              
                "prev_longest_road_holder".to_string(),     // longest road, hot (n_players)
                "prev_longest_road_length".to_string(), 
                "card_count".to_string(),
                "robber_count".to_string(),
                "count_dice_draws".to_string(),                             
                "action_type".to_string(),      // actions, hot (n_actions)

                "setup_settle_node".to_string(),   // hot
                "setup_road_node".to_string(),     // hot

                "i_robber".to_string(),            // hot (n_robbers)
                "i_tile".to_string(),              // hot (n_tiles)
                "i_victim".to_string(),            // hot (n_players)

                "roads_s1".to_string(),            // hot (n_nodes)
                "roads_e1".to_string(),             // hot (n_nodes)
                "roads_s2".to_string(),             // hot (n_nodes)
                "roads_e2".to_string(),             // hot (n_nodes)

                "plenty_r1".to_string(),            // hot (n_resources)
                "plenty_r2".to_string(),            // hot (n_resources)

                "monopoly_rd".to_string(),          // hot (n_resources)

                "bank_rs".to_string(),              // hot (n_resources)
                "bank_rd".to_string(),              // hot (n_resources)

                "harbor_trade_type".to_string(),    // hot (n_harbor_types)
                "harbor_trade_rs".to_string(),      // hot (n_resources)
                "harbor_trade_rd".to_string(),      // hot (n_resources)

                "quoting_player".to_string(),       // hot (n_player) [not necessary because = throwing_player]
                "trade_rs".to_string(),             // hot (n_resources)
                "trade_qs".to_string(),             
                "trade_rd".to_string(),             // hot (n_resources)
                "trade_qd".to_string(),

                "trade_response".to_string(),       // hot (already)

                "build_road_start".to_string(),     // hot (n_node)
                "build_road_end".to_string(),       // hot (n_node)
                "building_node".to_string()         // hot (n_node)
                
            ];

            for i in 0..self.parameters.n_resources {
                headers.push(format!("discard_resource_{}", i))
            }

            for i in 0..self.parameters.n_resources {
                headers.push(format!("budget_{}", i))
            }

            for i in 0..self.parameters.n_dev_card_types {
                headers.push(format!("drawn_cards_{}", i))
            }

            for i_player in 0..self.parameters.n_players {
                
                for i in 0..self.parameters.n_resources {
                    headers.push(format!("p{}_drawn_resource_{}", i_player, i))
                }
    
                for i in 0..self.parameters.n_resources {
                    headers.push(format!("p{}_public_budget_{}", i_player, i))
                }
            
                for i in 0..self.parameters.n_dev_card_types {
                    headers.push(format!("p{}_public_cards_{}", i_player, i))
                }
                
                headers.push(format!("p{}_longest_road", i_player))
            }

            let n_nodes = self.parameters.node_adjacency.len();

            for i_node in 0..n_nodes {
                headers.push(format!("node_{}_status", i_node));  // hot (n_status)
                headers.push(format!("node_{}_owner", i_node));   // hot (n_player + NA)
            }


            let n_edges = self.parameters.edge_map.len();
            for i_edge in 0..n_edges {
                headers.push(format!("road_{}_owner", i_edge));     // hot (n_player)
                headers.push(format!("harbor_{}_type", i_edge));    // hot (n_harbor_types)
                headers.push(format!("harbor_{}_owner", i_edge));   // hot (n_player + NA)
            }

            match &self.parameters.init_v_robber {
                Some(v_robbers) => {
                    for i in 0..v_robbers.len() {
                        headers.push(format!("robber_{}_location", i));     // hot (n_tiles)
                    }
                }
                None => {
                    headers.push("robber_0_location".to_string())
                },
            }

            let n_tiles = get_n_tiles(self.parameters.n_tile_rings, &self.parameters.tile_shape);
            for i_tile in 0..n_tiles {
                headers.push(format!("tile_{}_rng", i_tile)); // hot (n_dice_outcomes)
            }

            wtr.write_record(headers)?;
        }



        for i_log in 0..self.log.len() {
            if let Some(round) = &self.log[i_log].round {
                let mut row = vec![
                    i_log.to_string(), 
                    round.active_player.to_string(),
                    round.board.scores[round.active_player as usize].to_string(),
                    round.throwing_player.to_string(),
                    round.cards_played.to_string(),
                ];

                if let Some(dice) = round.board.dice_outcome {
                    row.push(dice.to_string());
                } else {
                    row.extend(vec![String::new()]);
                }

                match &round.phase {
                    Phase::SetUp => row.push(0.to_string()),
                    Phase::RobberDiscard => row.push(1.to_string()),
                    Phase::RobberMove => row.push(2.to_string()),
                    Phase::FirstCardPhase => row.push(3.to_string()),
                    Phase::TradingQuote => row.push(4.to_string()),
                    Phase::TradingResponse => row.push(5.to_string()),
                    Phase::Building => row.push(6.to_string()),
                    Phase::SecondCardPhase => row.push(7.to_string()),
                    Phase::Terminal => row.push(8.to_string()),
                }

                row.push(round.phase_count.to_string());

                if let Some(prev_lr) = &round.board.prev_longest_road {
                    row.push(prev_lr.0.to_string());
                    row.push(prev_lr.1.to_string());
                } else {
                    row.extend(vec![String::new(); 2]);
                }

                row.push(round.card_count.to_string());
                row.push(round.robber_count.to_string());
                row.push(self.log[i_log].count_dice_draws.to_string());


                let n_action_fields = 26 + self.parameters.n_resources;
                match &round.action {
                    Some(action) => {
                        match &action {
                            Action::SetUpMove(i_settle, i_road) => {
                                row.push(0.to_string());
                                row.push(i_settle.to_string());
                                row.push(i_road.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 2) as usize])
                            },
                            Action::Robber(i_robber, i_tile, i_victim) => {
                                row.push(1.to_string());
                                row.extend(vec![String::new(); 2]);
                                row.push(i_robber.to_string());
                                row.push(i_tile.to_string());
                                if let Some(victim) = i_victim {
                                    row.push(victim.to_string());
                                } else {
                                    row.extend(vec![String::new()])
                                }
                                
                                row.extend(vec![String::new(); (n_action_fields - 5) as usize])
                            },
                            Action::DiscardCards(v_discard) => {
                                row.push(2.to_string());
                                row.extend(vec![String::new(); (n_action_fields - self.parameters.n_resources) as usize]);
                                row.extend(v_discard.iter().map(|d| d.to_string()));
                            },
                            Action::NoDiscard => {
                                row.push(3.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::NoCardPlay => {
                                row.push(4.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::CardPlay(card) => {
                                row.push(5.to_string());

                                match &card {
                                    CardType::VPCard => {
                                        row.extend(vec![String::new(); n_action_fields as usize]);
                                    },
                                    CardType::KnightCard(i_robber, i_tile, i_victim) => {
                                        row.extend(vec![String::new(); 2]);
                                        row.push(i_robber.to_string());
                                        row.push(i_tile.to_string());
                                        if let Some(victim) = i_victim {
                                            row.push(victim.to_string());
                                        } else {
                                            row.extend(vec![String::new()])
                                        }
                                        
                                        row.extend(vec![String::new(); (n_action_fields - 5) as usize])
                                    },
                                    CardType::RoadsCard(s1, e1, s2, e2) => {
                                        row.extend(vec![String::new(); 5]);
                                        row.extend(vec![s1.to_string(), e1.to_string(), s2.to_string(), e2.to_string()]);
                                        row.extend(vec![String::new(); (n_action_fields - 9) as usize]);
                                    },
                                    CardType::PlentyCard(r1, r2) => {
                                        row.extend(vec![String::new(); 9]);
                                        row.push(r1.to_string());
                                        row.push(r2.to_string());
                                        row.extend(vec![String::new(); (n_action_fields - 11) as usize]);
                                    },
                                    CardType::MonopolyCard(resource_monopoly) => {
                                        row.extend(vec![String::new(); 11]);
                                        row.push(resource_monopoly.to_string());
                                        row.extend(vec![String::new(); (n_action_fields - 12) as usize]);
                                    },
                                }
                            },
                            Action::NoTrade => {
                                row.push(6.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize]);
                            },
                            Action::BankTrade(rs, rd) => {
                                row.push(7.to_string());
                                row.extend(vec![String::new(); 12]);
                                row.push(rs.to_string());
                                row.push(rd.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 14) as usize]);
                            },
                            Action::HarborTrade(harbor_type, rs, rd) => {
                                row.push(8.to_string());
                                row.extend(vec![String::new(); 14]);
                                row.push(harbor_type.to_string());
                                row.push(rs.to_string());
                                row.push(rd.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 17) as usize])
                            },
                            Action::TradeQuote(quote) => {
                                row.push(9.to_string());
                                row.extend(vec![String::new(); 17]);
                                row.push(quote.quoting_player.to_string());
                                row.push(quote.resource_supplied.to_string());
                                row.push(quote.quantity_supplied.to_string());
                                row.push(quote.resource_demanded.to_string());
                                row.push(quote.quantity_demanded.to_string());

                                if let Some(Outcome::TradeOutcome(_, accept)) = round.outcome {
                                   row.push((accept as usize).to_string());
                                } else {
                                    row.push(String::new())
                                }

                                row.extend(vec![String::new(); (n_action_fields - 23) as usize])
                            },
                            Action::TradeResponse(_, accept) => {
                                row.push(10.to_string());
                                row.extend(vec![String::new(); 22]);
                                row.push((*accept as usize).to_string());
                                row.extend(vec![String::new(); (n_action_fields - 23) as usize]);
                            },
                            Action::NoBuying => {
                                row.push(11.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::BuildRoad(s1, e1) => {
                                row.push(12.to_string());
                                row.extend(vec![String::new(); 23]);
                                row.push(s1.to_string());
                                row.push(e1.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 25) as usize]);
                            },
                            Action::BuildSettlement(node) => {
                                row.push(13.to_string());
                                row.extend(vec![String::new(); 25]);
                                row.push(node.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 26) as usize]);
                            },
                            Action::BuildCity(node) => {
                                row.push(14.to_string());
                                row.extend(vec![String::new(); 25]);
                                row.push(node.to_string());
                                row.extend(vec![String::new(); (n_action_fields - 26) as usize]);
                            },
                            Action::BuyDevCard => {
                                row.push(15.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::FinishRound => {
                                row.push(16.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::Save => {
                                row.push(17.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                            Action::Quit => {
                                row.push(18.to_string());
                                row.extend(vec![String::new(); n_action_fields as usize])
                            },
                        }
                    },
                    None => row.extend(vec![String::new(); (n_action_fields + 1) as usize]),
                }

                // active player's budget
                for b in &round.board.budgets[self.round.active_player as usize] {
                    row.push(b.to_string());
                }

                // active player's drawn resources
                for dc in &round.board.drawn_dev_cards[self.round.active_player as usize] {
                    row.push(dc.to_string());
                }

                for i_player in 0..(self.parameters.n_players as usize) {

                    for dr in &round.board.total_drawn_resources[i_player] {
                        row.push(dr.to_string());
                    }

                    for pb in &round.board.public_budgets[i_player] {
                        row.push(pb.to_string());
                    }

                    for pc in &round.board.public_dev_cards[i_player] {
                        row.push(pc.to_string());
                    }

                    if let Some(lr) = &round.board.longest_roads {
                        row.push(lr[i_player].to_string())
                    } else {
                        row.push(String::new());
                    }
                }

                
                for node in &round.board.nodes {
                    match &node.node_status {
                        NodeStatus::Free => {
                            row.push(0.to_string());
                            row.push(String::new());
                        },
                        NodeStatus::Adjacent => {
                            row.push(1.to_string());
                            row.push(String::new());
                        },
                        NodeStatus::Settled(i_owner) => {
                            row.push(2.to_string());
                            row.push(i_owner.to_string());
                        },
                        NodeStatus::Citied(i_owner) => {
                            row.push(3.to_string());
                            row.push(i_owner.to_string());
                        },
                    }
                }


                let n_edges = self.parameters.edge_map.len();
                for i_edge in 0..n_edges {

                    // roads
                    if let Some(roads) = &round.board.roads {
                        
                        let mut matched = false;

                        for road in roads {
                            let edge = make_edge(road.nodes.0, road.nodes.1);
                            
                            if let Some(index) = self.parameters.edge_map.get(&edge) {
                                if index == &i_edge {
                                    row.push(road.player.to_string());
                                    matched = true;
                                    break
                                }
                            }
                        }

                        if !matched {
                            row.push(String::new());
                        }
                    } else {
                        row.push(String::new());
                    }

                    // harbors 
                    if let Some(harbors) = &round.board.harbors {

                        let mut matched = false; 

                        for harbor in harbors {
                            let edge = make_edge(harbor.nodes.0, harbor.nodes.1);

                            if let Some(index) = self.parameters.edge_map.get(&edge) {
                                if index == &i_edge {
                                    row.push(harbor.harbor_type.to_string());
                                    if let Some(player) = harbor.player {
                                        row.push(player.to_string());
                                    } else {
                                        row.push(String::new());
                                    }
                                    matched = true;
                                    break
                                }
                            }

                        }

                        if !matched {
                            row.extend(vec![String::new(); 2]);
                        }

                    } else {
                        row.extend(vec![String::new(); 2]);
                    }

                } // end edges


                // robbers
                for i_tile in &round.board.v_robbers {
                    row.push(i_tile.to_string());
                }

                // tile rng
                for tile in &round.board.tiles {
                    if let Some(tile_rng) = tile.rng {
                        row.push(tile_rng.to_string());
                    } else {
                        row.push(String::new());
                    }
                }

                // write row
                wtr.write_record(row)?;

            }
        }
        
        wtr.flush()?;
        
        Ok(())
    }


    pub fn write_round_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        self.write_round_units(file_path)?;

        self.write_round_setup(file_path)?;

        self.write_round_robber(file_path)?;

        self.write_round_discard(file_path)?;

        self.write_round_knight(file_path)?;

        self.write_round_card_roads(file_path)?;

        self.write_round_plenty(file_path)?;

        self.write_round_monopoly(file_path)?;

        self.write_round_bank(file_path)?;

        self.write_round_harbor(file_path)?;

        self.write_round_trades(file_path)?;

        self.write_round_build_road(file_path)?;

        self.write_round_buildings(file_path)?;
        
        Ok(())
    }


    fn write_round_units(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "logs.csv")?;

        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "dice_outcome".to_string(),
                "active_player".to_string(),
                "throwing_player".to_string(),
                "cards_played".to_string(),
                "phase".to_string(),
                "phase_count".to_string(),
                "card_count".to_string(),
                "robber_count".to_string(),
                "action".to_string(),
                "count_dice_draws".to_string(),
                "duration".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                let mut row = vec![
                    self.parameters.id.clone(),
                    i_log.to_string()
                ];

                if let Some(dice_outcome) = round.board.dice_outcome {
                    row.push(dice_outcome.to_string());
                } else {
                    row.extend(vec![String::new()]);
                }

                row.push(round.active_player.to_string());

                row.push(round.throwing_player.to_string());

                row.push(round.cards_played.to_string());

                match &round.phase {
                    Phase::SetUp => row.push(0.to_string()),
                    Phase::RobberDiscard => row.push(1.to_string()),
                    Phase::RobberMove => row.push(2.to_string()),
                    Phase::FirstCardPhase => row.push(3.to_string()),
                    Phase::TradingQuote => row.push(4.to_string()),
                    Phase::TradingResponse => row.push(5.to_string()),
                    Phase::Building => row.push(6.to_string()),
                    Phase::SecondCardPhase => row.push(7.to_string()),
                    Phase::Terminal => row.push(8.to_string()),
                }

                row.push(round.phase_count.to_string());

                row.push(round.card_count.to_string());

                row.push(round.robber_count.to_string());

                match &round.action {
                    Some(action) => {
                        match &action {
                            Action::SetUpMove(_, _) => row.push(0.to_string()),
                            Action::Robber(_, _, _) => row.push(1.to_string()),
                            Action::DiscardCards(_) => row.push(2.to_string()),
                            Action::NoDiscard => row.push(3.to_string()),
                            Action::NoCardPlay => row.push(4.to_string()),
                            Action::CardPlay(_) => row.push(5.to_string()),
                            Action::NoTrade => row.push(6.to_string()),
                            Action::BankTrade(_, _) => row.push(7.to_string()),
                            Action::HarborTrade(_, _, _) => row.push(8.to_string()),
                            Action::TradeQuote(_) => row.push(9.to_string()),
                            Action::TradeResponse(_, _) => row.push(10.to_string()),
                            Action::NoBuying => row.push(11.to_string()),
                            Action::BuildRoad(_, _) => row.push(12.to_string()),
                            Action::BuildSettlement(_) => row.push(13.to_string()),
                            Action::BuildCity(_) => row.push(14.to_string()),
                            Action::BuyDevCard => row.push(15.to_string()),
                            Action::FinishRound => row.push(16.to_string()),
                            Action::Save => row.push(17.to_string()),
                            Action::Quit => row.push(18.to_string()),
                        }
                    },
                    None => row.extend(vec![String::new()]),
                }

                row.push(self.log[i_log].count_dice_draws.to_string());

                if let Some(duration) = &self.log[i_log].duration_ms {
                    row.push(duration.to_string());
                } else {
                    row.extend(vec![String::new()]);
                }

                wtr.write_record(row)?;
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_setup(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_setup.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "settle_node".to_string(),
                "road_node".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match action {
                        Action::SetUpMove(settle_node, road_node) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                settle_node.to_string(),
                                road_node.to_string(),
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }

                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_robber(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_robber.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "i_robber".to_string(),
                "i_tile".to_string(),
                "i_victim".to_string(),
                "robber_outcome".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::Robber(i_robber, i_tile, i_victim) => {
                            let mut row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                i_robber.to_string(),
                                i_tile.to_string()
                            ];

                            if let Some(victim) = i_victim {
                                row.push(victim.to_string());
                            } else {
                                row.extend(vec![String::new()]);
                            }

                            if let Some(Outcome::RobberOutcome(Some(resource))) = &round.outcome {
                                row.push(resource.to_string())
                            } else {
                                row.extend(vec![String::new()]);
                            }

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_discard(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_discard.csv")?;
        
        if file_empty {
            let mut headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
            ];

            for i in 0..self.parameters.n_resources {
                headers.push(format!("discard_resource_{}", i))
            }

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::DiscardCards(v_discard) => {
                            let mut row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string()
                            ];

                            for discard in v_discard {
                                row.push(discard.to_string());
                            }

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_knight(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_knight.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "i_robber".to_string(),
                "i_tile".to_string(),
                "i_victim".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::CardPlay(CardType::KnightCard(i_robber, i_tile, i_victim)) => {
                            let mut row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                i_robber.to_string(),
                                i_tile.to_string()
                            ];

                            if let Some(victim) = i_victim {
                                row.push(victim.to_string());
                            } else {
                                row.extend(vec![String::new()])
                            }

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_card_roads(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_card_roads.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "s1".to_string(),
                "e1".to_string(),
                "s2".to_string(),
                "e2".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::CardPlay(CardType::RoadsCard(s1, e1, s2, e2)) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                s1.to_string(),
                                e1.to_string(),
                                s2.to_string(),
                                e2.to_string()
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_plenty(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_plenty.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "r1".to_string(),
                "r2".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::CardPlay(CardType::PlentyCard(r1, r2)) => {
                                    let row  = vec![
                                        self.parameters.id.clone(),
                                        i_log.to_string(),
                                        r1.to_string(),
                                        r2.to_string()
                                    ];

                                    wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_monopoly(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_monopoly.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "monopoly_resource".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match action {
                        Action::CardPlay(CardType::MonopolyCard(monopoly_resource)) => {
                                let row = vec![
                                    self.parameters.id.clone(),
                                    i_log.to_string(),
                                    monopoly_resource.to_string()
                                ];

                                wtr.write_record(row)?;
                            },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_bank(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_bank.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "rs".to_string(),
                "rd".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::BankTrade(rs, rd) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                rs.to_string(),
                                rd.to_string()
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_harbor(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_harbor.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "harbor_type".to_string(),
                "rs".to_string(),
                "rd".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::HarborTrade(harbor_type, rs, rd) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                harbor_type.to_string(),
                                rs.to_string(),
                                rd.to_string()
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_trades(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_trades.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "quoting_player".to_string(),
                "rs".to_string(),
                "qs".to_string(),
                "rd".to_string(),
                "qd".to_string(),
                "responding_player".to_string(),
                "response".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::TradeQuote(quote) => {
                            match &round.outcome {
                                Some(Outcome::TradeOutcome(responding_player, response)) => {
                                    let row = vec![
                                        self.parameters.id.clone(),
                                        i_log.to_string(),
                                        quote.quoting_player.to_string(),
                                        quote.resource_supplied.to_string(),
                                        quote.quantity_supplied.to_string(),
                                        quote.resource_demanded.to_string(),
                                        quote.quantity_demanded.to_string(),
                                        responding_player.to_string(),
                                        response.to_string()
                                    ];

                                    wtr.write_record(row)?;
                                },
                                _ => continue,
                            }
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_round_build_road(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_build_road.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "start_node".to_string(),
                "end_node".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::BuildRoad(s1, e1) => {
                            let start_node: String;
                            let end_node: String; 
                            if s1 < e1 {
                                start_node = s1.to_string();
                                end_node = e1.to_string();
                            } else {
                                start_node = e1.to_string();
                                end_node = s1.to_string();
                            }

                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                start_node,
                                end_node
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_round_buildings(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (mut wtr, file_empty) = open_writer(file_path, "actions_buildings.csv")?;
        
        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "building_type".to_string(),
                "building_node".to_string()
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(action) = &round.action {

                    match &action {
                        Action::BuildSettlement(settle_node) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                0.to_string(),
                                settle_node.to_string()
                            ];

                            wtr.write_record(row)?;
                        },
                        Action::BuildCity(city_node) => {
                            let row = vec![
                                self.parameters.id.clone(),
                                i_log.to_string(),
                                1.to_string(),
                                city_node.to_string()
                            ];

                            wtr.write_record(row)?;
                        },
                        _ => continue,
                    }
                }
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

}


fn open_writer(path: &str, file: &str) -> Result<(Writer<File>, bool), Box<dyn Error>> {

    // println!("{}", path);
    let file_path = path.to_owned() + "/" + file;

    // Check if the file is empty
    let file_empty = if Path::new(file_path.clone().as_str()).exists() {
        let file = OpenOptions::new().read(true).open(file_path.clone())?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        buffer.is_empty()
    } else {
        true
    };

    // Open the file in append mode
    let file = OpenOptions::new()
    .append(true)
    .create(true) // Create the file if it doesn't exist
    .open(file_path.as_str())?;

    let wtr = Writer::from_writer(file);

    Ok((wtr, file_empty))
}