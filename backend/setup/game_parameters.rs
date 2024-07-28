use std::{collections::HashMap, vec};

use crate::backend::{io::read_parameters::{read_csv_to_option, read_deserialized_csv, read_harbors_csv, read_matrix_csv}, round::phase::Phase};

use super::{city::City, edge::{create_unique_edges, edge_index_map}, harbor::Harbor, player::Player, road::Road, settlement::Settlement, shape::{get_n_node, get_node_adjacency, get_tile_nodes, TileShape}};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameParameters {
    pub id: String,
    pub title: Option<String>,

    pub dice_seed: u64,
    pub n_dice: u32,
    pub n_faces: u32,

    pub robber_seed: u64,
    pub init_v_robber: Option<Vec<u32>>,
    pub robber_nos: Vec<u32>,

    tile_seed: u64,
    pub tile_shape: TileShape,
    pub n_tile_rings: u32,

    pub n_players: u32,
    pub v_players: Vec<Player>,

    pub n_resources: u32,

    pub init_player: u32,
    pub init_phase: Phase,
    pub init_harbors: Option<Vec<Harbor>>,
    pub init_roads: Option<Vec<Road>>,
    pub init_settlements: Option<Vec<Settlement>>,
    pub init_cities: Option<Vec<City>>,

    pub init_longest_road: Option<(u32, u32)>,
    pub init_largest_army: Option<(u32, u32)>,

    pub init_budgets: Option<Vec<Vec<u32>>>,
    pub init_public_budgets: Option<Vec<Vec<u32>>>,
    pub init_total_drawn_resources: Option<Vec<Vec<u32>>>,

    pub dev_card_seed: u64,
    pub n_dev_card_types: usize,
    pub init_undrawn_dev_cards: Vec<u32>,
    pub init_drawn_dev_cards: Option<Vec<Vec<u32>>>,
    pub init_public_dev_cards: Option<Vec<Vec<u32>>>,
    pub init_unknown_dev_cards: Option<Vec<Vec<Vec<f64>>>>,

    pub init_unknown_score: Option<Vec<Vec<f64>>>,

    pub v_tile_resources: Vec<Option<u32>>,
    pub init_tile_rng: Vec<Option<u32>>,

    pub node_adjacency: Vec<Vec<Option<u32>>>,
    pub node_tiles_adjacency: Vec<Vec<usize>>,
    pub tile_nodes: Vec<Vec<u32>>,
    pub edge_map: HashMap<(u32, u32), usize>,

    pub n_winning_vp: u32,

    pub max_trades: u32,
    pub max_cards: u32,
    pub max_builds: u32,

    pub n_setup_rounds: u32,

    pub building_costs: Vec<Vec<u32>>,
}

impl Default for GameParameters {
    fn default() -> Self {

        let n_tile_rings = 3;

        // read in tile resource assignment
        let tile_resource_path = "data/templates/default/tile_resource.csv";
        let v_tile_resource: Vec<Option<u32>> = read_csv_to_option(tile_resource_path).expect("An error occurred while reading tile resource file.");

        // read in harbours
        let harbor_path = "data/templates/default/harbors.csv";
        let v_harbors_raw = read_harbors_csv(harbor_path).expect("Error while reading harbors.");
        
        let mut v_harbors: Vec<Harbor> = vec![];
        for row in v_harbors_raw {
            v_harbors.push(Harbor::new(row))
        }

        // read initial dice outcome assignments
        let rng_path = "data/templates/default/init_tile_rng.csv";
        let v_rng: Vec<Option<u32>> = read_csv_to_option(rng_path).expect("An error occurred while reading rng file.");


        // node adjacency
        let node_adjacency = get_node_adjacency(n_tile_rings, &TileShape::Hexagon);

        let unique_edges = create_unique_edges(node_adjacency.clone());
        let edge_map = edge_index_map(&unique_edges);

        // tile-node adjacency
        let tile_nodes = get_tile_nodes(n_tile_rings, &TileShape::Hexagon);

        let n_players = 4;
        let v_players: Vec<Player> = (0..n_players).map(|i| Player::new_empty(i)).collect();

        let n_dev_card_types = 5;
        // order of development cards: 5 VP cards, 14 knight cards, 2 roads, 2 years of plenty, 2 monopolies
        let init_undrawn_dev_cards = vec![5, 14, 2, 2, 2];

        let n_nodes = get_n_node(n_tile_rings, &TileShape::Hexagon) as usize;
        
        let mut node_tiles_adjacency: Vec<Vec<usize>> = vec![vec![]; n_nodes];
        for (i_tile, v_nodes) in tile_nodes.iter().enumerate() {
            for i_node in v_nodes.iter() {
                node_tiles_adjacency[*i_node as usize].push(i_tile)
            }
        }
        

        Self { 
            id: Uuid::new_v4().to_string(),
            title: None,

            dice_seed: 42, 
            n_dice: 2, 
            n_faces: 6,

            robber_seed: 4444,
            init_v_robber: None,
            robber_nos: vec![7],

            tile_seed: 123, 
            tile_shape: TileShape::Hexagon,
            n_tile_rings, 

            n_players, 
            v_players,

            n_resources: 5,

            init_player: 0,
            init_phase: Phase::SetUp,
            init_harbors: Some(v_harbors), 
            init_roads: None, 
            init_settlements: None, 
            init_cities: None, 

            init_longest_road: None, 
            init_largest_army: None,

            init_budgets: None,
            init_public_budgets: None,
            init_total_drawn_resources: None, 

            dev_card_seed: 8888,
            n_dev_card_types,
            init_undrawn_dev_cards,
            init_drawn_dev_cards: None,
            init_public_dev_cards: None,
            init_unknown_dev_cards: None,

            init_unknown_score: None,

            v_tile_resources: v_tile_resource, 
            init_tile_rng: v_rng, 

            node_adjacency, 
            node_tiles_adjacency,
            tile_nodes,
            edge_map,

            n_winning_vp: 10,

            max_trades: 5,
            max_cards: 1,
            max_builds: 5,
            
            n_setup_rounds: 2,

            building_costs: vec![vec![1, 0, 1, 0, 0], vec![1, 1, 1, 0, 1], vec![0, 2, 0, 3, 0], vec![0, 1, 0, 1, 1]],
        }
    } 
}


impl GameParameters {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn default_with_title(self, title: String) -> Self {

        self.default_from_template(Some(title), "default".to_string())
    }

    pub fn default_from_template(self, title: Option<String>, template: String) -> Self {

        let n_tile_rings = 3;

        // read in tile resource assignment
        let tile_resource_path = "data/templates/".to_owned() + &template + "/tile_resource.csv";
        let v_tile_resource: Vec<Option<u32>> = read_csv_to_option(tile_resource_path).expect("An error occurred while reading tile resource file.");

        // read in harbours
        let harbor_path = "data/templates/".to_owned() + &template + "/harbors.csv";
        let v_harbors_raw = read_harbors_csv(&harbor_path).expect("Error while reading harbors.");
        
        let mut v_harbors: Vec<Harbor> = vec![];
        for row in v_harbors_raw {
            v_harbors.push(Harbor::new(row))
        }

        // read initial dice outcome assignments
        let rng_path = "data/templates/".to_owned() + &template + "/init_tile_rng.csv";
        let v_rng: Vec<Option<u32>> = read_csv_to_option(&rng_path).expect("An error occurred while reading rng file.");

        let budgets_path = "data/templates/".to_owned() + &template + "/init_budgets.csv";
        let init_budgets = read_matrix_csv(&budgets_path).ok();

        let default: GameParameters = Default::default();
        let n_players: u32;
        match &init_budgets {
            Some(vec) => n_players = vec.len() as u32,
            None => n_players = default.n_players,
        }

        Self {
            title: title, 
            init_harbors: Some(v_harbors), 
            v_tile_resources: v_tile_resource, 
            init_tile_rng: v_rng, 
            n_players,
            ..Default::default()
        }

    }

    
    pub fn default_settled(self, title: Option<String>, template: String) -> Self {

        let mut game_parameters = self.default_from_template(title, template.clone());

        

        let roads_path = "data/templates/".to_owned() + &template + "/init_roads.csv";
        let result: Vec<Road> = read_deserialized_csv(&roads_path).expect("Error while reading roads.");
        game_parameters.init_roads = Some(result);

        let settlements_path = "data/templates/".to_owned() + &template + "/init_settlements.csv";
        let result: Vec<Settlement> = read_deserialized_csv(&settlements_path).expect("Error while reading roads.");
        game_parameters.init_settlements = Some(result);

        let cities_path = "data/templates/".to_owned() + &template + "/init_cities.csv";
        game_parameters.init_cities = read_deserialized_csv(&cities_path).ok();

        let budgets_path = "data/templates/".to_owned() + &template + "/init_budgets.csv";
        game_parameters.init_budgets = read_matrix_csv(&budgets_path).ok();

        game_parameters
    }
}