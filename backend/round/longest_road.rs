use std::collections::{HashMap, HashSet};

use crate::backend::setup::road::Road;


pub fn get_longest_road(i_player: u32, roads: &Vec<Road>) -> u32 {

    let mut player_roads: Vec<Road> = vec![];
    for road in roads {
        if road.player == i_player {
            player_roads.push(road.clone())
        }
    }

    if player_roads.is_empty() {
        return 0
    } else if player_roads.len() == 1 {
        return 1
    } else {
        find_players_longest_road(player_roads)
    }
}


// Function to find nodes that occur only once and roads containing those nodes
fn find_unique_nodes_and_roads(roads: &[Road]) -> (Vec<u32>, Vec<Road>) {
    let mut node_counts = HashMap::new();
    let mut road_map = HashMap::new();

    // Count occurrences of each node and record roads containing those nodes
    for road in roads.iter().cloned() {
        let (node1, node2) = road.nodes;
        *node_counts.entry(node1).or_insert(0) += 1;
        *node_counts.entry(node2).or_insert(0) += 1;
        road_map.entry(node1).or_insert(Vec::new()).push(road.clone());
        road_map.entry(node2).or_insert(Vec::new()).push(road);
    }

    // Find nodes that occur only once
    let mut unique_nodes = Vec::new();
    let mut unique_roads = HashSet::new();
    for (node, count) in &node_counts {
        if *count == 1 {
            unique_nodes.push(*node);
            if let Some(roads) = road_map.get(node) {
                for road in roads {
                    unique_roads.insert(road.clone());
                }
            }
        } else {
            continue
        }
    }

    (unique_nodes, unique_roads.into_iter().collect())
}


struct ConnectedRoadIterator {
    all_roads: Vec<Road>,
    connected_roads: Vec<Vec<Road>>,
    end_points: Vec<u32>,
    unique_nodes: Vec<u32>,
}

impl ConnectedRoadIterator {
    fn new(all_roads: Vec<Road>, unique_road: Road) -> Self {
        let connected_roads = vec![vec![unique_road.clone()]];
        let end_points = vec![unique_road.nodes.0, unique_road.nodes.1];

        let (unique_nodes, unique_roads) = find_unique_nodes_and_roads(&all_roads);

        ConnectedRoadIterator {
            all_roads,
            connected_roads,
            end_points,
            unique_nodes,
        }
    }
}


impl Iterator for ConnectedRoadIterator {
    type Item = Vec<Vec<Road>>;

    fn next(&mut self) -> Option<Self::Item> {

        let init_len_connected_roads = self.connected_roads.iter().fold(0, |acc, x| acc.max(x.len()));

        let mut new_connected_roads = self.connected_roads.clone();
        
        // println!("len. con roads: {:?}", new_connected_roads.len());

        // TO DO: CHANGE THIS AD HOC CONSTRAINT TO SOMETHING JUSTIFIED
        if new_connected_roads.len() > 2 * self.unique_nodes.len() {
            return None;
        };

        for connected_road in &self.connected_roads {

            // find the remaining roads for a given set of connected roads
            let remaining_roads: Vec<&Road> = self.all_roads.iter()
                                            .filter(|road| !connected_road.iter().any(|r| r.nodes == road.nodes))
                                            .collect();

            // println!("{:?}",remaining_roads);

            // if no roads remain, go to next connected road
            if remaining_roads.is_empty() {
                continue;
            }

            // find unique end points in this connected road
            let (unique_nodes, unique_roads) = find_unique_nodes_and_roads(&connected_road);
            // println!("unique nodes found");

            // println!("Unique nodes: {:?}", unique_nodes);
            // if unique_nodes.is_empty() {
            //     continue;
            // }

            // compare these unique points to the end points
            let end_points = self.end_points.clone();
            // println!("End points: {:?}", end_points);
            for end_point in &end_points {
                if unique_nodes.contains(&end_point) {
                    for &remaining_road in &remaining_roads {

                        if &remaining_road.nodes.0 == end_point ||  &remaining_road.nodes.1 == end_point {
                            let mut temp_road = connected_road.clone();
                            temp_road.push(remaining_road.clone());
                            new_connected_roads.push(temp_road);

                            // remove the end point and add new one
                            self.end_points.retain(|&x| x != end_point.clone());

                            if &remaining_road.nodes.0 == end_point {
                                self.end_points.push(remaining_road.clone().nodes.1);
                            } else {
                                self.end_points.push(remaining_road.clone().nodes.0);
                            }

                        }

                    }
                }
            } 

        }


        let final_len_connected_roads = new_connected_roads.iter().fold(0, |acc, x| acc.max(x.len()));

        // println!("ilcr: {:?}", init_len_connected_roads);
        // println!("flcr: {:?}", final_len_connected_roads);
        
        if final_len_connected_roads <= init_len_connected_roads {
            return None;
        }

        self.connected_roads = new_connected_roads.clone();

        Some(new_connected_roads)

    }
}


fn find_players_longest_road(roads: Vec<Road>) -> u32 {
    // let (unique_nodes, mut unique_roads) = find_unique_nodes_and_roads(&roads);

    // if unique_roads.is_empty() {
    //     unique_roads.push(roads[1].clone())
    // }

    let mut max_lengths: Vec<usize> = vec![];
    for road in &roads {
        let iterator = ConnectedRoadIterator::new(roads.clone(), road.clone());
        let last_iteration = iterator.last().unwrap_or(vec![vec![road.clone()]]);
        max_lengths.push(last_iteration.iter().fold(0, |acc, x| acc.max(x.len())));
    }
    
    max_lengths.iter().fold(0, |acc, x| acc.max(*x as u32))
}