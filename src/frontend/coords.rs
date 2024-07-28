use std::f64::consts::PI;

use crate::backend::setup::{node::Node, shape::TileShape, tile::Tile};


pub fn add_conc_coords_nodes(mut nodes: Vec<Node>, n_node_rings: u32, tile_shape: &TileShape) -> Vec<Node> {
    let v_nodes_conc_coords = get_node_conc_coords(n_node_rings, &tile_shape);

    for (i_node, ref mut node_opt) in nodes.clone().iter().enumerate() {
        match node_opt.coords_conc {
            Some(_) => break,  // if one node already has coordinates, all of them do. No need to continue
            None => {
                nodes[i_node].coords_conc = Some(v_nodes_conc_coords[i_node]);
            },
        }
    }

    nodes
}

pub fn add_cart_coords_nodes(mut nodes: Vec<Node>, n_node_rings: u32, tile_shape: &TileShape) -> Vec<Node> {
    let v_nodes_cart_coords = get_node_cart_coords(n_node_rings, &tile_shape);

    for (i_node, node_opt) in &mut nodes.clone().iter().enumerate() {
        match node_opt.coords_cart {
            Some(_) => break, // if one node already has coordinates, all of them do. No need to continue
            None => {
                nodes[i_node].coords_cart = Some(v_nodes_cart_coords[i_node]);
            },
        }
    }

    nodes
}

pub fn add_cart_coords_tiles(mut tiles: Vec<Tile>, n_tile_rings: u32, tile_shape: &TileShape) -> Vec<Tile> {
    let v_tiles_cart_coords = get_tile_cart_coords(n_tile_rings, &tile_shape);

    for (i_node, tile_opt) in &mut tiles.clone().iter().enumerate() {
        match tile_opt.coords_cart {
            Some(_) => break, // if one node already has coordinates, all of them do. No need to continue
            None => {
                tiles[i_node].coords_cart = Some(v_tiles_cart_coords[i_node]);
            },
        }
    }

    tiles
}




fn get_node_conc_coords(n_node_rings: u32, tile_shape: &TileShape) -> Vec<(u32, u32)> {
    match tile_shape {
        TileShape::Hexagon => {
            get_node_conc_coords_hexagon(n_node_rings)
        }
    }
}

fn get_node_conc_coords_hexagon(n_node_rings: u32) -> Vec<(u32, u32)> {
    
    let mut nodes_conc: Vec<(u32, u32)> = vec![];

    for i_node_ring in 0..n_node_rings {
        let n_ring = 6 * (1 + (i_node_ring / 2));
        // println!("{:?}", n_ring);

        for i_ring in 0..n_ring {
            let coord_conc = (i_node_ring as u32, i_ring as u32);
            nodes_conc.push(coord_conc)
        }
    }

    nodes_conc
}


fn get_node_cart_coords(n_node_rings: u32, tile_shape: &TileShape) -> Vec<(f64, f64)> {
    match tile_shape {
        TileShape::Hexagon => {
            get_node_cart_coords_hexagon(n_node_rings)
        }
    }
}

fn get_node_cart_coords_hexagon(n_node_rings: u32) -> Vec<(f64, f64)> {
    let mut nodes_cart: Vec<(f64, f64)> = vec![];

    for i_node_ring in 0..n_node_rings {
        let n_ring = 6 * (1 + (i_node_ring / 2));
        // println!("{:?}", n_ring);

        for i_ring in 0..n_ring {
            let coord_cart = ( -(1.0 + i_node_ring as f64) * (-2.0 * PI * (i_ring as f64/ n_ring as f64) ).cos() / n_node_rings as f64, -(1.0 + i_node_ring as f64) * (-2.0 * PI * (i_ring as f64 / n_ring as f64)).sin() / n_node_rings as f64);
            nodes_cart.push(coord_cart)
        }
    }

    nodes_cart
}

fn get_tile_cart_coords(n_tile_rings: u32, tile_shape: &TileShape) -> Vec<(f64, f64)> {
    match tile_shape {
        TileShape::Hexagon => {
            get_tile_cart_coords_hexagon(n_tile_rings)
        }
    }
}


fn get_tile_cart_coords_hexagon(n_tile_rings: u32) -> Vec<(f64, f64)> {
    let mut tiles_cart: Vec<(f64, f64)> = vec![];

    for i_tile_ring in 0..n_tile_rings {
        let n_ring = if i_tile_ring == 0 {
            1 } else {
                6 *i_tile_ring
            };

        for i_ring in 0..n_ring {
            let coord_cart = ((-1.0 * i_tile_ring as f64) * (-2.0 * PI * ((0.5 + i_ring as f64 )/ n_ring as f64) ).cos() / n_tile_rings as f64, (-1.0 * i_tile_ring as f64) * (-2.0 * PI * ((0.5 + i_ring as f64) / n_ring as f64)).sin() / n_tile_rings as f64);
            tiles_cart.push(coord_cart)
        }
    }

    tiles_cart
}