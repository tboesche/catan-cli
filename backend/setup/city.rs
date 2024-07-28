
use serde::Deserialize;

use super::{node::Node, node_status::NodeStatus};

#[derive(Debug, Deserialize, Clone)]
pub struct City {
    pub name: Option<String>,
    pub node_id: u32,
    pub player_id: u32,
}

impl City {
    pub fn place(self, mut nodes: Vec<Node>) -> Result<Vec<Node>, &'static str> {

        let node_id = self.node_id;
        let player_id = self.player_id;
        let name = self.name;
    
        let node = nodes[node_id as usize].clone();
    
        match node.node_status {
            NodeStatus::Free =>  Err("Please build a settlement first, before upgrading to a city."),
            NodeStatus::Adjacent => Err("Cannot place a city directly adjacent to another settlement or city."),
            NodeStatus::Settled(old_player_id) => {
                if old_player_id != player_id {
                    Err("Cannot upgrade another player's settlement.")
                } else {
                    let mut output = nodes.clone();
    
                let city = City {
                    name,
                    node_id,
                    player_id,
                };
    
                output[node_id as usize].city = Some(city);
                output[node_id as usize].node_status = NodeStatus::Citied(player_id);

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
                }
                
            }
            NodeStatus::Citied(_) => Err("Cannot place a new city on top of an existing city."),
        }   
    }
}

