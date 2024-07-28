
use serde::Deserialize;

use crate::backend::{round::{longest_road::get_longest_road, score::{get_public_score, get_score}}, setup::shape::get_n_tiles};

use super::{dice::Dice, game_parameters::GameParameters, harbor::Harbor, node::Node, road::Road, shape::get_n_node, tile::Tile};

#[derive(Debug, Clone, Deserialize)]
pub struct Board {
    pub nodes: Vec<Node>,
    pub tiles: Vec<Tile>,

    pub v_robbers: Vec<u32>,

    pub roads: Option<Vec<Road>>,
    pub harbors: Option<Vec<Harbor>>,

    pub tile_rng: Vec<Option<u32>>,

    pub budgets: Vec<Vec<u32>>,
    pub public_budgets: Vec<Vec<u32>>,
    pub total_drawn_resources: Vec<Vec<u32>>,

    pub undrawn_dev_cards: Vec<u32>,
    pub drawn_dev_cards: Vec<Vec<u32>>,
    pub public_dev_cards: Vec<Vec<u32>>,
    pub unknown_dev_cards: Vec<Vec<Vec<f64>>>,

    pub longest_roads: Option<Vec<u32>>,
    pub prev_longest_road: Option<(u32, u32)>,
    pub armies: Vec<u32>,
    pub prev_largest_army: Option<(u32, u32)>,

    pub scores: Vec<u32>,
    pub public_scores: Vec<u32>,
    pub unknown_scores: Vec<Vec<f64>>,

    pub dice: Dice,
    pub dice_outcome: Option<u32>,
}

impl Board {
    pub fn new(parameters: &GameParameters) -> Result<Self, &'static str> {

        let tile_shape = &parameters.tile_shape;
        let n_tile_rings = parameters.n_tile_rings;

        let n_nodes = get_n_node(n_tile_rings, tile_shape);
        let n_tiles = get_n_tiles(n_tile_rings, tile_shape);

        // initialize tiles
        let mut tiles = vec![Tile::default(); n_tiles as usize];

        
        for i_tile in 0..n_tiles as usize {
            tiles[i_tile].id = i_tile as u32;
            tiles[i_tile].nodes = parameters.tile_nodes[i_tile].clone();
        }  

        for (i_tile, rng) in parameters.init_tile_rng.iter().enumerate() {
            tiles[i_tile].rng = *rng;
        }  

        for (i_tile, resource) in parameters.v_tile_resources.iter().enumerate() {
            tiles[i_tile].resource = *resource;
        }

        // initialize robbers
        let v_robbers = match &parameters.init_v_robber {
            Some(robbers) => {
                robbers.clone()
            },
            None => { // set robbers on first tile with rng = None
                let mut first_free_tile = 0_u32;
                for (tile, resource) in parameters.init_tile_rng.iter().enumerate() {
                    if *resource == None {
                        first_free_tile = tile as u32;
                        break;
                    }
                }
                vec![first_free_tile]
            },
        };

        // initialize nodes
        let mut nodes = vec![Node::default(); n_nodes as usize];

        for i_node in 0..n_nodes {
            nodes[i_node as usize].id = i_node;
            nodes[i_node as usize].neighbors = parameters.node_adjacency[i_node as usize].clone();
        }

        // initialize harbors 
        let harbors = &parameters.init_harbors;

        match harbors {
            Some(ref v_harbors) => {
                for harbor in v_harbors {
                    nodes = harbor.clone().place(nodes)?;
                }
            },
            None => (),
        }
        
        // place initial settlements
        match &parameters.init_settlements {
            Some(v_settlements) => {
                for settlement in v_settlements {
                    nodes = settlement.clone().place(nodes)?;
                }
            },
            None => (),
        }

        // place initial cities
        match &parameters.init_cities {
            Some(v_cities) => {
                for city in v_cities {
                    nodes = city.clone().place(nodes)?;
                }
            },
            None => (),
        }

        // initialize roads
        let roads = &parameters.init_roads;

        match roads {
            Some(ref v_roads) => {
                for road in v_roads {
                    nodes = road.clone().place(nodes)?;
                }
            },
            None => (),
        }

        // initialize budgets
        let budgets: Vec<Vec<u32>>;
        match &parameters.init_budgets {
            Some(b) => {
                budgets = b.clone();
            },
            None => {
                budgets = vec![vec![0; parameters.n_resources as usize]; parameters.n_players as usize];
            },
        }

        // initialize public budgets
        let public_budgets: Vec<Vec<u32>>;
        match &parameters.init_public_budgets {
            Some(b) => {
                public_budgets = b.clone();
            },
            None => {
                public_budgets = vec![vec![0; parameters.n_resources as usize]; parameters.n_players as usize];
            },
        }

        // initialize total drawn resources
        let total_drawn_resources: Vec<Vec<u32>>;
        match &parameters.init_total_drawn_resources {
            Some(b) => {
                total_drawn_resources = b.clone();
            },
            None => {
                total_drawn_resources = vec![vec![0; parameters.n_resources as usize]; parameters.n_players as usize];
            },
        }

        // initialize drawn development cards
        let drawn_dev_cards: Vec<Vec<u32>>;
        match &parameters.init_drawn_dev_cards {
            Some(dc) => {
                drawn_dev_cards = dc.clone();
            },
            None => {
                drawn_dev_cards = vec![vec![0; parameters.n_dev_card_types as usize]; parameters.n_players as usize];
                // drawn_dev_cards = vec![vec![0, 5, 0, 0, 0], vec![1, 0, 0, 0, 0], vec![0,0,0,0,0], vec![0,0,0,0,0]]
            }
        }

        // initialize public development cards
        let public_dev_cards: Vec<Vec<u32>>;
        match &parameters.init_public_dev_cards {
            Some(dc) => {
                public_dev_cards = dc.clone();
            },
            None => {
                public_dev_cards = vec![vec![0; parameters.n_dev_card_types as usize]; parameters.n_players as usize];
                // public_dev_cards = vec![vec![0, 5, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0,0,0,0,0], vec![0,0,0,0,0]]
            }
        }

        // initialize unknown development cards
        let unknown_dev_cards: Vec<Vec<Vec<f64>>>;
        match &parameters.init_unknown_dev_cards {
            Some(dc) => {
                unknown_dev_cards = dc.clone();
            },
            None => {
                unknown_dev_cards = vec![vec![vec![0.0; parameters.n_dev_card_types as usize]; parameters.n_players as usize]; parameters.n_players as usize];
            }
        }

        // initialize previously longest road (from init roads)
        let prev_longest_road: Option<(u32, u32)> = parameters.init_longest_road;

        // initialize longest roads
        let longest_roads: Option<Vec<u32>> = match &roads {
            Some(v_roads) => {
                let v_longest = (0..parameters.n_players)
                                    .map(|i_player| get_longest_road(i_player, v_roads)).collect();
                Some(v_longest)
            },
            None => {
                None
            },
        };

        // initialize previously largest army
        let prev_largest_army = parameters.init_largest_army.clone();

        // initialize largest army
        let armies: Vec<u32> = public_dev_cards.iter().map(|v_player| v_player[1]).collect();


        // println!("{:?}", parameters.n_players);
        let scores: Vec<u32> = (0..parameters.n_players)
                                .map(|i_player| get_score(i_player, &nodes,  &drawn_dev_cards, &longest_roads, &prev_longest_road,
                                &armies, &prev_largest_army))
                                .collect();

        let public_scores: Vec<u32> = (0..parameters.n_players)
                                .map(|i_player| get_public_score(i_player, &nodes, &public_dev_cards, &longest_roads, &prev_longest_road,
                                &armies, &prev_largest_army))
                                .collect();     

        // initialize unknown score
        let unknown_scores: Vec<Vec<f64>>;
        match &parameters.init_unknown_score {
            Some(s) => {
                unknown_scores = s.clone();
            },
            None => {
                unknown_scores = vec![vec![0.0; parameters.n_players as usize]; parameters.n_players as usize];
            },
        }  


        let dice = Dice::new(parameters.n_dice, parameters.n_faces, parameters.dice_seed);                     
        

        Ok(Self {
            nodes,
            tiles,

            v_robbers,

            roads: roads.clone(),
            harbors: harbors.clone(),

            tile_rng: parameters.init_tile_rng.clone(),

            budgets,
            public_budgets,
            total_drawn_resources,

            undrawn_dev_cards: parameters.init_undrawn_dev_cards.clone(),
            drawn_dev_cards,
            public_dev_cards,
            unknown_dev_cards,

            longest_roads,
            armies,

            scores,
            public_scores,
            unknown_scores,
            
            dice: dice.clone(),
            dice_outcome: dice.draw,
            prev_longest_road,
            prev_largest_army,    
        })
    }

}