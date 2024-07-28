

#[derive(Debug, Clone)]
pub enum TileShape {
    Hexagon,
}

pub fn get_n_node(n_tile_rings: u32, shape: &TileShape) -> u32 {
    match shape {
        TileShape::Hexagon => {
            get_n_node_hexagon(n_tile_rings)
        }
    }
}

fn get_n_node_hexagon(n_tile_rings: u32) -> u32 {
    6 * n_tile_rings.pow(2)
}


pub fn get_n_node_rings(n_tile_rings: u32, shape: &TileShape) -> u32 {
    match shape {
        TileShape::Hexagon => {
            get_n_node_rings_hexagon(n_tile_rings)
        }
    }
}

fn get_n_node_rings_hexagon(n_tile_rings: u32) -> u32 {
    1 + 2 * (n_tile_rings - 1)
}


pub fn get_n_tiles(n_tile_rings: u32, shape: &TileShape) -> u32 {
    match shape {
        TileShape::Hexagon => {
            get_n_tiles_hexagon(n_tile_rings)
        }
    }
}

fn get_n_tiles_hexagon(n_tile_rings: u32) -> u32 {
    if n_tile_rings == 1 {
        1
    } else {
        1 + 6 * ( n_tile_rings * (n_tile_rings - 1) / 2)
    }
}


pub fn get_node_adjacency(n_tile_rings: u32, shape: &TileShape) -> Vec<Vec<Option<u32>>> {
    match shape {
        TileShape::Hexagon => {
            get_node_adjacency_hexagon(n_tile_rings)
        },
    }
}

fn get_node_adjacency_hexagon(n_tile_rings: u32) -> Vec<Vec<Option<u32>>> {
    // get number of nodes and node rings
    let n_node_rings = get_n_node_rings(n_tile_rings, &TileShape::Hexagon);
    let n_nodes = get_n_node(n_tile_rings, &TileShape::Hexagon);

    // Each interior node is connected to 3 other nodes (note: all valid indices < n_nodes). Some exterior nodes will only have two adjacent nodes.
    let mut node_adjacency: Vec<Vec<Option<u32>>> = vec![vec![None; 3]; n_nodes as usize];

    let mut idx_node = 0;
    let mut n_cum_rings: u32 = 0;
    for i_node_ring in 0..n_node_rings {
        let n_ring = 6 * (1 + (i_node_ring / 2));

        // nodes around the central tile
        if i_node_ring == 0 {
            for i_ring in 0..n_ring {

                node_adjacency[idx_node][0] = if i_ring == 0 {
                    Some(n_cum_rings + n_ring - 1)
                } else {
                    Some(n_cum_rings + i_ring - 1)
                };

                node_adjacency[idx_node][1] = if i_ring == n_ring-1 {
                    Some(n_cum_rings)
                } else {
                    Some(n_cum_rings + i_ring + 1)
                };

                node_adjacency[idx_node][2] = Some(i_ring + n_ring);

                idx_node += 1;
            }
        } else {
            idx_node = n_cum_rings as usize;

            // println!("{:?}", n_ring);

            // odd node rings
            if i_node_ring % 2 == 1 {
                let mut i_next_ring = 0;

                for i_ring in 0..n_ring {
                    node_adjacency[idx_node][0] = Some(idx_node as u32 - n_ring);

                    // println!("{:?} / {:?} \t = {:?}", i_next_ring, i_node_ring+1, i_next_ring / (i_node_ring + 1));

                    node_adjacency[idx_node][1] = Some(idx_node as u32 + n_ring  + (i_next_ring / (i_node_ring + 1)));
                    
                    if i_ring != 0 {
                        i_next_ring += 1;
                    }

                    node_adjacency[idx_node][2] = if i_ring == 0 {
                        Some(n_cum_rings - 1 + n_ring + 6 * (1 + ((i_node_ring + 1) / 2)))
                    } else {
                        Some(node_adjacency[idx_node][1].unwrap() + 1)
                    };
                    i_next_ring += 1;

                    
                    idx_node += 1;
                }
            }

            // even node rings
            if i_node_ring % 2 == 0 {
                let mut i_next_ring = 0;

                for i_ring in 0..n_ring {

                    if i_ring % (n_ring / 6) == 0 {
                        node_adjacency[idx_node][0] = Some(idx_node as u32 - 6 * (1 + ((i_node_ring-1) / 2)) - (i_ring / (n_ring / 6)));
                        node_adjacency[idx_node][1] = Some(idx_node as u32 + 1);
                    } else if i_ring % (n_ring / 6) == 1 {
                        node_adjacency[idx_node][0] = Some(idx_node as u32 - 1);
                        node_adjacency[idx_node][1] = Some(idx_node as u32 - 6 * (1 + ((i_node_ring-1) / 2)) - (i_ring / (n_ring / 6)));
                    } else {
                        node_adjacency[idx_node][0] = node_adjacency[idx_node-1][1];
                        node_adjacency[idx_node][1] = Some(node_adjacency[idx_node][0].unwrap() + 1);
                    }
                    
                    if i_ring == n_ring-1 {
                        node_adjacency[idx_node][1] = Some(idx_node as u32 - n_ring + 1 - 6 * (1 + ((i_node_ring-1) / 2)));
                    }

                    if i_node_ring < n_node_rings - 1 {
                        node_adjacency[idx_node][2] = Some(idx_node as u32 + n_ring);
                    }

                    idx_node += 1;
                }
            }

        }

        n_cum_rings += n_ring;
    }

    node_adjacency
}


pub fn get_tile_nodes(n_tile_rings: u32, shape: &TileShape) -> Vec<Vec<u32>> {
    match shape {
        TileShape::Hexagon => {
            get_tile_nodes_hexagon(n_tile_rings)
        },
    }
}

fn get_tile_nodes_hexagon(n_tile_rings: u32) -> Vec<Vec<u32>> {

    let n_tiles = get_n_tiles(n_tile_rings, &TileShape::Hexagon);

    // Each tile has six adjacent nodes
    let mut tile_node_adjacency: Vec<Vec<u32>> = vec![vec![0; 6]; n_tiles as usize];

    // The central tile is a special case
    tile_node_adjacency[0] = (0..6).collect();

    let mut idx_tile = 1;
    let mut lowest_node_ring: i32 = -1;
    for i_ring in 1..n_tile_rings {
        let n_ring = 6 * i_ring;
        let mut node_first_ring = if i_ring == 1 {
            0
        } else {
            6 * (i_ring-1).pow(2) as u32
        };
        
        let mut node_second_ring = node_first_ring + 6 * (1 + ((lowest_node_ring) / 2)) as u32;
        let mut node_third_ring  = node_second_ring + 6 * (1 + ((lowest_node_ring+1) / 2)) as u32;
        let mut node_fourth_ring = node_third_ring + 6 * (1 + ((lowest_node_ring+2) / 2)) as u32;

        for i_tile in 0..n_ring {

            if i_tile % (n_ring / 6) == 0 {
                if i_ring == 1 {
                    
                    tile_node_adjacency[idx_tile][0] = node_first_ring + 1;
                    if i_tile != n_ring - 1{
                        tile_node_adjacency[idx_tile][5] = node_first_ring;
                        tile_node_adjacency[idx_tile][1] = node_second_ring + 1;
                        tile_node_adjacency[idx_tile][4] = node_second_ring;
                        tile_node_adjacency[idx_tile][3] = node_third_ring;
                        tile_node_adjacency[idx_tile][2] = node_third_ring + 1;
                    } else {
                        tile_node_adjacency[idx_tile][1] = 0;
                        tile_node_adjacency[idx_tile][2] = 5;
                        tile_node_adjacency[idx_tile][3] = node_second_ring;
                        tile_node_adjacency[idx_tile][4] = node_third_ring;
                        tile_node_adjacency[idx_tile][5] = node_third_ring + 1;
                    }
                    

                    node_first_ring += 1;
                    node_second_ring += 1;
                    node_third_ring += 2;
                } else {
                    tile_node_adjacency[idx_tile][5] = node_second_ring;
                    tile_node_adjacency[idx_tile][0] = node_second_ring + 1;
                    tile_node_adjacency[idx_tile][4] = node_third_ring;
                    tile_node_adjacency[idx_tile][1] = node_third_ring + 1;
                    tile_node_adjacency[idx_tile][3] = node_fourth_ring;
                    tile_node_adjacency[idx_tile][2] = node_fourth_ring + 1;

                    node_second_ring += 1;
                    node_third_ring += 1;
                    node_fourth_ring += 2;
                }

            } else {
                if i_tile != n_ring - 1{
                    tile_node_adjacency[idx_tile][0] = node_first_ring + 1;
                    tile_node_adjacency[idx_tile][1] = node_second_ring + 1;
                    tile_node_adjacency[idx_tile][2] = node_third_ring + 1;
                } else {
                    tile_node_adjacency[idx_tile][0] = (node_first_ring as i32 - 6 * (1 + ((lowest_node_ring) / 2)) as i32  + 1) as u32;
                    tile_node_adjacency[idx_tile][1] = (node_second_ring as i32 - 6 * (1 + ((lowest_node_ring+1) / 2)) as i32  + 1) as u32;
                    tile_node_adjacency[idx_tile][2] = (node_third_ring as i32 - 6 * (1 + ((lowest_node_ring+2) / 2)) as i32  + 1) as u32;
                }
                
                tile_node_adjacency[idx_tile][5] = node_second_ring;
                tile_node_adjacency[idx_tile][4] = node_third_ring;
                tile_node_adjacency[idx_tile][3] = node_fourth_ring;

                node_first_ring     += 1;
                node_second_ring    += 1;
                node_third_ring     += 1;
                node_fourth_ring    += 1;
            }

            idx_tile += 1;
        }

        lowest_node_ring += 2;
    }

    tile_node_adjacency
}


pub fn get_n_node_neighbors(shape: &TileShape) -> u32 {
    match shape {
        TileShape::Hexagon => 3,
    }
}

pub fn get_n_tile_neighbors(shape: &TileShape) -> u32 {
    match shape {
        TileShape::Hexagon => 6,
    }
}