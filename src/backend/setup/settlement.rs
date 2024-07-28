
use serde::Deserialize;

use super::{node::Node, node_status::NodeStatus};

#[derive(Debug, Deserialize, Clone)]
pub struct Settlement {
    pub name: Option<String>,
    pub node_id: u32,
    pub player_id: u32,
}

impl Settlement {
    pub fn place(self, mut nodes: Vec<Node>) -> Result<Vec<Node>, &'static str> {

        let node_id = self.node_id;
        let player_id = self.player_id;
        let name = self.name;

        let node = nodes[node_id as usize].clone();
    
        match node.node_status {
            NodeStatus::Free => {
                let mut output = nodes.clone();
    
                let settlement = Settlement {
                    name,
                    node_id,
                    player_id,
                };
    
                output[node_id as usize].settlement = Some(settlement);
                output[node_id as usize].node_status = NodeStatus::Settled(player_id);
    
                // update harbor, if present
                match output[node_id as usize].harbor {
                    Some(ref mut harbor) => {
                        harbor.player = Some(player_id);
                    },
                    None => (),
                }

                // update neighbors status to "near a settlement"
                for &i_neighbor in node.neighbors.iter() {
                    match i_neighbor {
                        Some(id) => {
                            output[id as usize].node_status = NodeStatus::Adjacent;
                        },
                        None => continue,
                    }
                }
                
                Ok(output)
            },
            NodeStatus::Adjacent => return Err("Cannot place a settlement directly adjacent to another settlement or city."),
            NodeStatus::Settled(_) => return Err("Cannot place a new settlement on top of an existing settlement."),
            NodeStatus::Citied(_) => return Err("Cannot place a new settlement on top of a city."),
        }   
    }
}
