use crate::backend::setup::node::Node;



pub fn get_score(
    i_player: u32, 
    nodes: &Vec<Node>, 
    drawn_dev_cards: &Vec<Vec<u32>>,
    longest_roads: &Option<Vec<u32>>,
    prev_longest_road: &Option<(u32, u32)>,
    armies: &Vec<u32>, 
    prev_largest_army: &Option<(u32, u32)>
) -> u32 {

    // count the player's settlements
    let n_settlements = count_settlements(i_player, &nodes);

    // count the player's cities
    let n_cities = count_cities(i_player, &nodes);

    // count the player's victory cards
    let n_vp_cards = count_vp_cards(i_player, &drawn_dev_cards);

    // check whether longest road applies
    let longest_road = check_longest_road(i_player, &longest_roads, prev_longest_road);

    // check whether the largest army applies.
    let largest_army = check_largest_army(i_player,  &armies, prev_largest_army);

    // compute score
    n_settlements + n_cities + n_vp_cards + 2 * longest_road as u32 + 2 * largest_army as u32
}

pub fn get_public_score(
    i_player: u32, 
    nodes: &Vec<Node>, 
    public_dev_cards: &Vec<Vec<u32>>,
    longest_roads: &Option<Vec<u32>>,
    prev_longest_road: &Option<(u32, u32)>,
    armies: &Vec<u32>, 
    prev_largest_army: &Option<(u32, u32)>
) -> u32 {

    // count the player's settlements
    let n_settlements = count_settlements(i_player, &nodes);

    // count the player's cities
    let n_cities = count_cities(i_player, &nodes);

    // check whether longest road applies
    let longest_road = check_longest_road(i_player, &longest_roads, prev_longest_road);

    // check whether the largest army applies.
    let largest_army = check_largest_army(i_player,  &armies, prev_largest_army);

    let public_vp_cards = public_dev_cards[i_player as usize][0];

    // compute score
    public_vp_cards + n_settlements + n_cities + 2 * longest_road as u32 + 2 * largest_army as u32
}


fn count_settlements(i_player: u32, nodes: &Vec<Node>) -> u32 {

    let mut count = 0;
    for node in nodes.iter() {
        match &node.settlement {
            Some(settlement) => {
                if settlement.player_id == i_player {
                    count += 1;
                }
            },
            None => continue,
        };
    }

    count
}


fn count_cities(i_player: u32, nodes: &Vec<Node>) -> u32 {

    let mut count = 0;
    for node in nodes.iter() {
        match &node.city {
            Some(city) => {
                if city.player_id == i_player {
                    count += 1;
                }
            },
            None => continue,
        };
    }

    count
}


fn count_vp_cards(i_player: u32, drawn_dev_cards: &Vec<Vec<u32>>) -> u32 {
    drawn_dev_cards[i_player as usize][0]
}

// longest road is only assigned 
pub fn check_longest_road(i_player: u32, longest_roads: &Option<Vec<u32>>, prev_longest_road: &Option<(u32, u32)>) -> bool {

    match longest_roads {
        Some(lr) => {
            let len_longest_road = lr.iter().fold(0, |acc, x| acc.max(*x));

            if lr[i_player as usize] < len_longest_road {
                return false
            } else {
                match prev_longest_road {
                    Some(plr) => {
                        if lr[i_player as usize] > plr.1 {
                            return true
                        } else if plr.0 == i_player {
                            return true
                        } else {
                            return false
                        }
                    },
                    None => {
                        if len_longest_road > 4 {
                            return true
                        } else {
                            return false
                        }
                    },
                }
            }
        },
        None => {
            return false
        },
    }
}

// largest army is only assigned if (i) player has teh largest army AND (ii) army > two.
fn check_largest_army(i_player: u32, armies: &Vec<u32>, prev_largest_army: &Option<(u32, u32)>) -> bool {
    let size_largest_army = armies.iter().fold(0,|acc, &x| acc.max(x));

    if armies[i_player as usize] < size_largest_army {
        return false
    }

    match prev_largest_army {
        Some(pla) => {
            if armies[i_player as usize] > pla.1 {
                return true
            } else if pla.0 == i_player {
                return true
            } else {
                return false
            }
        },
        None => {
            if size_largest_army > 2 {
                return true
            } else {
                return false
            }
        },
    }
}
