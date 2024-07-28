use serde::Deserialize;

use crate::backend::io::read_parameters::deserialize_tuple;

use super::node::Node;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Road {
    pub player: u32,
    #[serde(deserialize_with = "deserialize_tuple")]
    pub nodes: (u32, u32),
}

impl Road {
    pub fn place(self, mut v_nodes: Vec<Node>) -> Result<Vec<Node>, &'static str> {
        let player_id = self.player;
        let first_node = v_nodes[self.nodes.0 as usize].id;
        let second_node = v_nodes[self.nodes.1 as usize].id;
    
        match v_nodes[self.nodes.0 as usize].roads {
            Some(ref mut node_roads) => {
                for node_road in &mut *node_roads {
                    if node_road.1 == second_node {
                        return Err("Cannot place a road on top of another road.")
                    }
                }

                node_roads.push((player_id, second_node));
            },
            None => {
                v_nodes[self.nodes.0 as usize].roads =  Some(vec![(player_id, second_node)]);
            },
        }
        
        match v_nodes[self.nodes.1 as usize].roads {
            Some(ref mut node_roads) => {

                for node_road in &mut *node_roads {
                    if node_road.1 == first_node {
                        return Err("Cannot place a road on top of another road.")
                    } 
                }

                node_roads.push((player_id, first_node));
            },
            None => {
                v_nodes[self.nodes.1 as usize].roads =  Some(vec![(player_id, first_node)]);
            },
        }
    
        Ok(v_nodes)
    }
}
