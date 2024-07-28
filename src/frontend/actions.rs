use crate::backend::{round::{action::Action, cards::CardType}, setup::{game::Game, shape::get_n_node_rings}};

use super::{board_parameters::UIBoardParameters, coords::add_conc_coords_nodes};


pub fn get_pretty_action(action: &Action, game: &Game, ui_parameters: &UIBoardParameters) -> String {
    
    let n_tile_rings = game.parameters.n_tile_rings.clone();
    let tile_shape = &game.parameters.tile_shape;
    let n_node_rings = get_n_node_rings(n_tile_rings, tile_shape);
    let nodes = add_conc_coords_nodes(game.round.board.nodes.clone(), n_node_rings, &tile_shape);
    
    match action {
        Action::SetUpMove(settle_node, road_end) => {
            format!("Build settlement on {:?} and road to {:?},", nodes[*settle_node as usize].coords_conc.unwrap(), nodes[*road_end as usize].coords_conc.unwrap())
        },
        Action::Robber(i_robber, i_tile, i_victim) => {
            format!("Move robber {:?} to tile {:?} and steal from player {:?},", i_robber, i_tile, i_victim)
        },
        Action::DiscardCards(v_discard) => {
            let names = &ui_parameters.v_resource_names;
            format!("Discard: ({}: {:?}, {}: {:?}, {}: {:?}, {}: {:?}, {}: {:?})", names[0], v_discard[0], names[1], v_discard[1],names[2], v_discard[2],names[3], v_discard[3],names[4], v_discard[4])
        },
        Action::NoDiscard => {
            "Do not discard any resources.".to_string()
        }
        Action::NoCardPlay => {
            "Do not play a development card,".to_string()
        },
        Action::CardPlay(card_type) => {
            match &card_type {
                CardType::VPCard => {
                    "Play victory point card,".to_string()
                },
                CardType::KnightCard(i_robber, i_tile, i_victim) => {
                    format!("Move robber {:?} to tile {:?} and steal from player {:?}", i_robber, i_tile, i_victim)
                },
                CardType::RoadsCard(first_start, first_end, second_start, second_end) => {
                    format!("Play roads card and build roads from {:?} to {:?} and from {:?} to {:?},", nodes[*first_start as usize].coords_conc.unwrap(),nodes[*first_end as usize].coords_conc.unwrap(), nodes[*second_start as usize].coords_conc.unwrap(),nodes[*second_end as usize].coords_conc.unwrap())
                },
                CardType::PlentyCard(first_resource, second_resource) => {
                    let names = &ui_parameters.v_resource_names;
                    format!("Play 'Year of Plenty' card and get one {} and one {},", names[*first_resource as usize], names[*second_resource as usize])
                },
                CardType::MonopolyCard(i_resource) => {
                    let names = &ui_parameters.v_resource_names;
                    format!("Play 'Monopoly' card and get all cards of resource {},", names[*i_resource as usize])
                },
            }
        },
        Action::NoTrade => {
            "Do not trade,".to_string()
        },
        Action::BankTrade(r_supplied, r_demanded) => {
            let names = &ui_parameters.v_resource_names;
            format!("Trade with the bank: 4 {} for 1 {},", names[*r_supplied as usize], names[*r_demanded as usize])
        },
        Action::HarborTrade(harbor_type, r_supplied, r_demanded) => {
            let names = &ui_parameters.v_resource_names;
            let harbor_names = &ui_parameters.v_harbor_symbols;
            if harbor_type == &game.parameters.n_resources {
                format!("Trade at harbor {}: 3 {} for 1 {},", harbor_names[*harbor_type as usize], names[*r_supplied as usize], names[*r_demanded as usize])
            } else {
                format!("Trade at harbor {}: 2 {} for 1 {},", harbor_names[*harbor_type as usize], names[*r_supplied as usize], names[*r_demanded as usize])
            }
            
        },
        Action::TradeQuote(quote) => {
            let rs = quote.resource_supplied as usize;
            let rd = quote.resource_demanded as usize;
            let qs = quote.quantity_supplied;
            let qd = quote.quantity_demanded;
            let names = &ui_parameters.v_resource_names;

            format!("Trade offer to co-players: {:?} {} for {:?} {},", qs, names[rs], qd, names[rd])
        },
        Action::TradeResponse(_, accept) => {
            if *accept {
                format!("Accept trade offer,")
            } else {
                format!("Reject trade offer,")
            }
            
        },
        Action::NoBuying => {
            "Do not buy (more),".to_string()
        },
        Action::BuildRoad(start_node, end_node) => {
            // let nodes = &game.round.board.nodes;
            format!("Build road from {:?} to {:?},", nodes[*start_node as usize].coords_conc.unwrap(), nodes[*end_node as usize].coords_conc.unwrap())
        },
        Action::BuildSettlement(settle_node) => {
            // let nodes = &game.round.board.nodes;
            format!("Build settlement on {:?},", nodes[*settle_node as usize].coords_conc.unwrap())
        },
        Action::BuildCity(city_node) => {
            // let nodes = &game.round.board.nodes;
            format!("Build city on {:?},", nodes[*city_node as usize].coords_conc.unwrap())
        },
        Action::BuyDevCard => {
            "Buy a development card".to_string()
        },
        Action::FinishRound => {
            "Finish round,".to_string()
        },
        Action::Save => {
            "Save game, or".to_string()
        },
        Action::Quit => {
            "Quit game.".to_string()
        },
    }
    
}