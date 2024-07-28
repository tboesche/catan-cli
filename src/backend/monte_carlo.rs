use std::{error::Error, time::Instant};

use crate::{backend::setup::game::Game, frontend::board_parameters::UIBoardParameters};

use super::{logging::summary::Summary, setup::game_parameters::GameParameters};

use rayon::prelude::*;
use uuid::Uuid;

pub fn simulate_games(n_sims: u32, game_parameters: &GameParameters, template: String, title: String) -> Result<(), Box<dyn Error>> {

    let start = Instant::now();

    let n_blocks = 10;
    let block_size = n_sims / n_blocks;

    let mut first_sim = 0;
    let mut last_sim = block_size.min(n_sims);
    
    for i_block in 0..n_blocks {

        let indices = (first_sim..last_sim).collect::<Vec<u32>>();
        let games: Vec<Game> = indices.par_iter()
                    .map(|&i_game| {

                        let game_time = Instant::now();
                        // println!("Simulation {:?} begun.", i_game);

                        let mut game = Game::from_template(template.clone()).unwrap();

                        game.parameters = game_parameters.clone();

                        game.parameters.id = Uuid::new_v4().to_string();
                        game.parameters.title = Some(title.clone());

                        game.parameters.dice_seed += i_game as u64;
                        game.parameters.robber_seed += i_game as u64;
                        game.parameters.dev_card_seed += i_game as u64;
                        
                        println!("Simulation {:?} set up.", i_game);

                        // game.run().unwrap_or({
                        //     game = original_game;
                        // });

                        match game.run() {
                            Ok(_) => (),
                            Err(e) => println!("{}",e),
                        };

                        println!("Simulation {:?} done.", i_game);

                        // if game_time.elapsed().as_secs() > 5 {
                        //     let ui_parameters = UIBoardParameters::default();
                        //     let board_name = ("board_".to_owned() + &i_game.to_string()).to_string();

                        //     game.draw_board(ui_parameters, board_name);
                        // }

                        game
                        

                    })
                    .collect();

        for game in games {
            game.encode_log()?;
        }

        first_sim = ((i_block+1) * block_size).min(n_sims);
        last_sim = ((i_block+2) * block_size).min(n_sims);

    }

    println!("Simulations finished in {:?} seconds.", start.elapsed().as_secs());
    
    Ok(())
}