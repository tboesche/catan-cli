
use serde::Deserialize;

use super::{city::City, harbor::Harbor, node_status::NodeStatus, settlement::Settlement};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Node {
    pub id: u32,
    pub node_status: NodeStatus,
    pub neighbors: Vec<Option<u32>>,
    pub roads: Option<Vec<(u32, u32)>>,
    pub harbor: Option<Harbor>,
    pub settlement: Option<Settlement>,
    pub city: Option<City>,
    pub coords_conc: Option<(u32, u32)>,
    pub coords_cart: Option<(f64, f64)>,
}
