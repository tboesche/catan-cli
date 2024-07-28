use std::error::Error;

use catan_cli::backend::monte_carlo::simulate_games;
use catan_cli::backend::setup::game::Game;

use catan_cli::backend::setup::player::{Player, PlayerType};
use catan_cli::frontend;
use catan_cli::frontend::board_parameters::UIBoardParameters;


fn main() -> Result<(), Box<dyn Error>> {

    let n_sims = 1200;
    let template = "beginner-map".to_string();

    let mut beginner_game = Game::from_template(template.clone())?;
    let mut game_parameters = beginner_game.parameters.clone();
    
    // for i in 0..4 {
    //     game_parameters.v_players[i] = Player::new_classic(i as u32);
    // }

    // game_parameters.v_players[0] = Player::new_simple_nn(0); 

    game_parameters.v_players[0].player_type = PlayerType::Human; 

    beginner_game.parameters = game_parameters;

    beginner_game.parameters.title = Some("test".to_string());

    beginner_game.run()?;

    // simulate_games(n_sims, &game_parameters, template,"test_classic".to_string())?;

    // let ui_parameters = UIBoardParameters::default();

    // frontend::nodes::create_svg(&beginner_game, &ui_parameters);
    // frontend::tiles::create_svg(&beginner_game, &ui_parameters);


    Ok(())
}