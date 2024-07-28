use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Tile {
    pub id: u32,
    pub nodes: Vec<u32>,
    pub rng: Option<u32>,
    pub resource: Option<u32>,
    pub coords_cart: Option<(f64,f64)>,
}