use std::{error::Error, fs::OpenOptions, io::{BufReader, Read}, path::Path};

use csv::Writer;

use crate::backend::setup::{game::Game, node_status::NodeStatus, shape::{get_n_node, TileShape}};

impl Game {
    pub fn write_board_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        self.write_board_by_player(file_path)?;
        self.write_board_nodes(file_path)?;
        self.write_board_roads(file_path)?;
        self.write_board_robbers(file_path)?;
        self.write_board_harbors(file_path)?;

        Ok(())
    }


    fn write_board_by_player(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/boards_by_player.csv";

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

        let mut wtr = Writer::from_writer(file);

        // Check if the file is empty to write headers only once
        if file_empty {
            let mut headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "player_id".to_string(),
                "score".to_string(),
                "public_score".to_string(),
                "longest_road".to_string(),
                "prev_longest_road_holder".to_string(),
                "prev_longest_road_length".to_string(),
            ];

            for i in 0..self.parameters.n_resources {
                headers.push(format!("drawn_resource_{}", i))
            }

            for i in 0..self.parameters.n_resources {
                headers.push(format!("budget_{}", i))
            }

            for i in 0..self.parameters.n_resources {
                headers.push(format!("public_budget_{}", i))
            }
            
            for i in 0..self.parameters.n_dev_card_types {
                headers.push(format!("drawn_cards_{}", i))
            }

            for i in 0..self.parameters.n_dev_card_types {
                headers.push(format!("public_cards_{}", i))
            }


            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for log_id in 0..n_logs {
            if let Some(round) = &self.log[log_id].round {
                for i_player in 0..self.parameters.n_players {
                    let mut row = vec![
                        self.parameters.id.clone(),
                        log_id.to_string(),
                        i_player.to_string(),
                        round.board.scores[i_player as usize].to_string(),
                        round.board.public_scores[i_player as usize].to_string()
                    ];

                    if let Some(v_lr) = &round.board.longest_roads {
                        row.push(v_lr[i_player as usize].to_string());
                    } else {
                        row.extend(vec![String::new()]);
                    }

                    if let Some(longest_road) = round.board.prev_longest_road {
                        if longest_road.0 == i_player {
                            row.push(true.to_string());
                        } else {
                            row.push(false.to_string());
                        }

                        row.push(longest_road.1.to_string());
                    } else {
                        row.extend(vec![String::new(); 2])
                    }

                    row.extend(round.board.total_drawn_resources[i_player as usize].iter().map(|dr| dr.to_string()));

                    row.extend(round.board.budgets[i_player as usize].iter().map(|b| b.to_string()));

                    row.extend(round.board.public_budgets[i_player as usize].iter().map(|pb| pb.to_string()));

                    row.extend(round.board.drawn_dev_cards[i_player as usize].iter().map(|dc| dc.to_string()));

                    row.extend(round.board.public_dev_cards[i_player as usize].iter().map(|pdc| pdc.to_string()));

                    wtr.write_record(&row)?;
                }
            } 
        }
            
        wtr.flush()?;

        Ok(())
    }


    fn write_board_nodes(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/nodes.csv";

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

        let mut wtr = Writer::from_writer(file);

        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "node_id".to_string(),
                "status".to_string(),
                "owner_id".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();
        let n_nodes = get_n_node(self.parameters.n_tile_rings, &TileShape::Hexagon) as usize;

        for log_id in 0..n_logs { 
            for node_id in 0..n_nodes {
                if let Some(round) = &self.log[log_id].round {
                    let mut row = vec![
                        self.parameters.id.to_string(),
                        log_id.to_string(),
                        node_id.to_string(),
                    ];

                    match round.board.nodes[node_id].node_status {
                        NodeStatus::Free => row.extend(vec!["0".to_string(), String::new()]),
                        NodeStatus::Adjacent => row.extend(vec![1.to_string(), String::new()]),
                        NodeStatus::Settled(i_owner) => row.extend(vec![2.to_string(), i_owner.to_string()]),
                        NodeStatus::Citied(i_owner) => row.extend(vec![3.to_string(), i_owner.to_string()]),
                    }

                    wtr.write_record(&row)?;
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }


    fn write_board_roads(&self, file_path: &str) -> Result<(), Box<dyn Error>> {

        let file_path = file_path.to_owned() + "/roads.csv";

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

        let mut wtr = Writer::from_writer(file);

        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "start_id".to_string(),
                "end_id".to_string(),
                "owner_id".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                if let Some(roads) = &round.board.roads {
                    for road in roads {
                        let start_node: String;
                        let end_node: String;
                        if road.nodes.0 < road.nodes.1 {
                            start_node = road.nodes.0.to_string();
                            end_node = road.nodes.1.to_string();
                        } else {
                            start_node = road.nodes.1.to_string();
                            end_node = road.nodes.0.to_string();
                        }

                        let row = vec![
                            self.parameters.id.clone(),
                            i_log.to_string(),
                            start_node,
                            end_node,
                            road.player.to_string()
                        ];

                        wtr.write_record(&row)?;
                    }
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }

    fn write_board_robbers(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        let file_path = file_path.to_owned() + "/robbers.csv";

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

        let mut wtr = Writer::from_writer(file);

        if file_empty {
            let mut headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
            ];

            match &self.parameters.init_v_robber {
                Some(v_robbers) => {
                    for i in 0..v_robbers.len() {
                        headers.push(format!("robber_location_{}", i));
                    }
                }
                None => {
                    headers.push("robber_location_0".to_string())
                },
            }

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                let v_robbers = &round.board.v_robbers;

                let mut row = vec![
                    self.parameters.id.clone(),
                    i_log.to_string()
                ];

                for i_tile in v_robbers {
                    row.push(i_tile.to_string());
                }

                wtr.write_record(row)?;
            }
        }

        wtr.flush()?;
        
        Ok(())
    }

    fn write_board_harbors(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        
        let file_path = file_path.to_owned() + "/harbors.csv";

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

        let mut wtr = Writer::from_writer(file);

        if file_empty {
            let headers = vec![
                "game_id".to_string(),
                "log_id".to_string(),
                "harbor_type".to_string(),
                "start_id".to_string(),
                "end_id".to_string(),
                "owner_id".to_string(),
            ];

            wtr.write_record(&headers)?;
        }

        let n_logs = self.log.len();

        for i_log in 0..n_logs {
            if let Some(round) = &self.log[i_log].round {
                for node in &round.board.nodes {
                    if let Some(harbor) = &node.harbor {
                        
                        let first_node: String; 
                        let second_node: String;
                        if harbor.nodes.0 < harbor.nodes.1 {
                            first_node  = harbor.nodes.0.to_string();
                            second_node = harbor.nodes.1.to_string();
                        } else {
                            first_node  = harbor.nodes.1.to_string();
                            second_node = harbor.nodes.0.to_string();
                        }

                        let mut row = vec![
                            self.parameters.id.clone(),
                            i_log.to_string(),
                            harbor.harbor_type.to_string(),
                            first_node,
                            second_node
                        ];

                        if let Some(i_owner) = harbor.player  {
                            row.push(i_owner.to_string());
                        } else {
                            row.extend(vec![String::new()]);
                        }

                        wtr.write_record(row)?;
                    }
                }
            }
        }
        
        wtr.flush()?;

        Ok(())
    }

}