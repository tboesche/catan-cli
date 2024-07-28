use crate::backend::{logging::summary::Summary, round::{action::Action, outcome::Outcome}};
use csv::Writer;
use std::{error::Error, io::{BufReader, Read}, path::Path};
use std::fs::OpenOptions;
use std::io::Write;

impl Summary {
    pub fn write_to_csv(self, file_path: String) -> Result<(), Box<dyn Error>> {
        
        self.write_main_results(&file_path)?;
        self.write_settlements(&file_path)?;
        self.write_cities(&file_path)?;
        self.write_roads(&file_path)?;
        self.write_actions_and_outcomes(&file_path)?;

        Ok(())

    } 

    fn write_main_results(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/main_results.csv";

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
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_path.as_str())?;

        let mut wtr = Writer::from_writer(file);

        // Check if the file is empty to write headers only once
        if file_empty {
            let mut headers = vec![
                "title".to_string(),
                "player_id".to_string(),
                "n_rounds".to_string(),
                "winner_id".to_string(),
                "duration".to_string(),
                "score".to_string(),
            ];

            for i in 0..self.player_summaries[0].drawn_resources.len() {
                headers.push(format!("drawn_resource_{}", i))
            }
            wtr.write_record(&headers)?;
        }
            
    
        for ps in &self.player_summaries {
            let mut row = vec![
                self.game_title.clone().unwrap_or("".to_string()),
                ps.player_id.to_string(),
                self.n_rounds.to_string(),
                self.winner_id.to_string(),
                self.duration.to_string(),
                ps.score.to_string(),
            ];
            for dr in &ps.drawn_resources {
                row.push(dr.to_string());
            }
            wtr.write_record(&row)?;
        }
        wtr.flush()?;
        Ok(())
    } 


    fn write_settlements(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/settlements.csv";

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
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_path.as_str())?;

        let mut wtr = Writer::from_writer(file);
        
        if file_empty {
            wtr.write_record(&["game_title", "player_id", "settlement"])?;
        };
        
    
        for ps in &self.player_summaries {
            for settlement in &ps.settlements {
                wtr.write_record(&[
                    self.game_title.clone().unwrap_or("".to_string()),
                    ps.player_id.to_string(),
                    settlement.to_string(),
                ])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    fn write_cities(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/cities.csv";

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
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_path.as_str())?;

        let mut wtr = Writer::from_writer(file);
        
        if file_empty {
            wtr.write_record(&["game_title","player_id", "city"])?;
        };
    
        for ps in &self.player_summaries {
            for city in &ps.cities {
                wtr.write_record(&[
                    self.game_title.clone().unwrap_or("".to_string()),
                    ps.player_id.to_string(),
                    city.to_string(),
                ])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    fn write_roads(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.to_owned() + "/roads.csv";
        let mut wtr = Writer::from_path(file_path.as_str())?;
        wtr.write_record(&["game_title","player_id", "road_start", "road_end"])?;
    
        for ps in &self.player_summaries {
            for road in &ps.roads {
                wtr.write_record(&[
                    self.game_title.clone().unwrap_or("".to_string()),
                    ps.player_id.to_string(),
                    road.0.to_string(),
                    road.1.to_string(),
                ])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }



fn write_actions_and_outcomes(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
    let file_path = file_path.to_owned() + "actions_and_outcomes.csv";
    
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
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_path.as_str())?;

    let mut wtr = Writer::from_writer(file);

    // Check if the file is empty to write headers only once
    if file_empty {
        wtr.write_record(&[
            "game_title",
            "player_id",
            "active_player",
            "action_type",
            "action_details",
            "outcome_type",
            "outcome_details",
        ])?;
    }

    for ps in &self.player_summaries {
        for (active_player, action, outcome) in &ps.actions_and_outcomes {
            let mut row = vec![
                self.game_title.clone().unwrap_or("".to_string()),
                ps.player_id.to_string(),
                active_player.to_string(),
            ];

            if let Some(action) = action {
                row.push(match action {
                    Action::SetUpMove(_, _) => "SetUpMove",
                    Action::Robber(_, _, _) => "Robber",
                    Action::DiscardCards(_) => "DiscardCards",
                    Action::NoCardPlay => "NoCardPlay",
                    Action::CardPlay(_) => "CardPlay",
                    Action::NoTrade => "NoTrade",
                    Action::BankTrade(_, _) => "BankTrade",
                    Action::HarborTrade(_, _, _) => "HarborTrade",
                    Action::TradeQuote(_) => "TradeQuote",
                    Action::TradeResponse(_, _) => "TradeResponse",
                    Action::NoBuying => "NoBuying",
                    Action::BuildRoad(_, _) => "BuildRoad",
                    Action::BuildSettlement(_) => "BuildSettlement",
                    Action::BuildCity(_) => "BuildCity",
                    Action::BuyDevCard => "BuyDevCard",
                    Action::FinishRound => "FinishRound",
                    Action::Save => "Save",
                    Action::Quit => "Quit",
                }.to_string());

                row.push(match action {
                    Action::SetUpMove(a, b) => format!("{}, {}", a, b),
                    Action::Robber(a, b, c) => format!("{}, {}, {:?}", a, b, c),
                    Action::DiscardCards(cards) => format!("{:?}", cards),
                    Action::NoCardPlay => String::new(),
                    Action::CardPlay(card) => format!("{:?}", card),
                    Action::NoTrade => String::new(),
                    Action::BankTrade(a, b) => format!("{}, {}", a, b),
                    Action::HarborTrade(a, b, c) => format!("{}, {}, {}", a, b, c),
                    Action::TradeQuote(quote) => format!("{:?}", quote),
                    Action::TradeResponse(a, b) => format!("{}, {}", a, b),
                    Action::NoBuying => String::new(),
                    Action::BuildRoad(a, b) => format!("{}, {}", a, b),
                    Action::BuildSettlement(a) => a.to_string(),
                    Action::BuildCity(a) => a.to_string(),
                    Action::BuyDevCard => String::new(),
                    Action::FinishRound => String::new(),
                    Action::Save => String::new(),
                    Action::Quit => String::new(),
                });
            } else {
                row.extend(vec![String::new(); 2]);
            }

            if let Some(outcome) = outcome {
                row.push(match outcome {
                    Outcome::RobberOutcome(_) => "RobberOutcome",
                    Outcome::DrawCardOutcome(_) => "DrawCardOutcome",
                    Outcome::TradeOutcome(_, _) => "TradeOutcome",
                }.to_string());

                row.push(match outcome {
                    Outcome::RobberOutcome(a) => a.to_string(),
                    Outcome::DrawCardOutcome(a) => a.to_string(),
                    Outcome::TradeOutcome(a, b) => format!("{}, {}", a, b),
                });
            } else {
                row.extend(vec![String::new(); 2]);
            }

            wtr.write_record(&row)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

}