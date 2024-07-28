use std::error::Error;

use csv::ReaderBuilder;
use tch::{nn, Device};

use crate::{ai::{classic_player, random_player, simple_nn::{self, evaluate::SimpleNN}}, backend::round::action::Action};

use super::game::Game;

#[derive(Debug, Clone)]
pub struct Player {
    id: u32,
    pub name: Option<String>,
    color_id: Option<u32>,
    pub player_type: PlayerType,
    pub player_function: Option<fn(&Game, Vec<Action>) -> Option<Action>>,
}

#[derive(Debug, Clone)]
pub enum PlayerType {
    Human,
    Random, 
    Myopic,
    Fixed,
    Classic(Vec<Vec<f64>>),
    ClassicPlus(Vec<Vec<f64>>),
    SimpleNN
}

impl Default for Player {
    fn default() -> Self {
        Self { id: 0, name: None, color_id: None, player_type: PlayerType::Random, player_function: None}
    }
}

impl Player {
    pub fn new(id: u32, name: String, color_id: u32) -> Player {
        Self {
            id,
            name: Some(name),
            color_id: Some(color_id),
            player_type: PlayerType::Human,
            player_function: None,
        }
    }

    pub fn new_empty(id: u32) -> Player {
        Self {
            id,
            name: None,
            color_id: None,
            player_type: PlayerType::Random,
            player_function: Some(random_player::play::play),
        }
    }

    pub fn new_classic(id: u32) -> Player {
        
        let weights = read_weights("data/ai/classic/weights.csv").unwrap_or(vec![vec![0.0_f64; 13]; 5]);
        
        Self {
            id,
            name: None,
            color_id: None,
            player_type: PlayerType::Classic(weights),
            player_function: Some(classic_player::play::play),
        }
    }

    pub fn new_classic_plus(id: u32) -> Player {
        
        let weights = read_weights("data/ai/classic_plus/weights.csv").unwrap_or(vec![vec![0.0_f64; 13]; 5]);
        
        Self {
            id,
            name: None,
            color_id: None,
            player_type: PlayerType::Classic(weights),
            player_function: Some(classic_player::play::play),
        }
    }

    pub fn new_simple_nn(id: u32) -> Player {

        // let device = if tch::Cuda::is_available() {
        //     Device::Cuda(0)
        // } else {
        //     Device::Cpu
        // };

        // // Load the model
        // let mut vs = nn::VarStore::new(device);
        // let model = SimpleNN::new(&vs.root(), 2368, 1);
        // let model_path = "src/ai/simple_nn/weights_short.safetensors";
        // vs.load(model_path).expect("PyTorch model could not be loaded");


        Self { 
            id, 
            name: None, 
            color_id: None, 
            player_type: PlayerType::SimpleNN, 
            player_function: Some(simple_nn::play::play) 
        }
    }
}


fn read_weights(file_path: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    let mut data: Vec<Vec<f64>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row: Vec<f64> = record.iter()
            .map(|s| s.parse::<f64>())
            .collect::<Result<_, _>>()?;
        data.push(row);
    }

    Ok(data)
}
