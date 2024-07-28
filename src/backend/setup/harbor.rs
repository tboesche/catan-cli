use serde::Deserialize;

use super::node::Node;


#[derive(Debug, Default, Clone, Deserialize)]
pub struct Harbor {
    pub nodes: (u32, u32),
    pub harbor_type: u32,
    pub player: Option<u32>,
}

impl Harbor {
    pub fn new(row: (u32, u32, u32)) -> Self {
        Harbor { nodes: (row.0, row.1), harbor_type: row.2, player: None}
    }

    pub fn place(mut self, mut v_nodes: Vec<Node>) -> Result<Vec<Node>, &'static str> {

        // habors can only be ``acqquired'' by placement of cities or settlements
        self.player = None;

        match v_nodes[self.nodes.0 as usize].harbor {
            Some(_) => Err("Please ensure that no node is adjacent to more than one harbor."),
            None => {
                match v_nodes[self.nodes.1 as usize].harbor {
                    Some(_) => Err("Please ensure that no node is adjacent to more than one harbor."),
                    None => {
                        v_nodes[self.nodes.0 as usize].harbor = Some(self.clone());
                        v_nodes[self.nodes.1 as usize].harbor = Some(self.clone());

                        Ok(v_nodes)       
                    }
                }
            },
        }

        

    }
    
}