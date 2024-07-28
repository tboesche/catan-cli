
pub mod backend;
pub mod frontend;

pub mod ai;

#[cfg(test)]
mod tests {

    use backend::setup::game::Game;

    use super::*;

    
    #[test]
    #[should_panic(expected = "Cannot place a road on top of another road.")]
    fn test_double_road() {
        let beginner_game = Game::from_template_settled("test_double_road".to_string()).unwrap();

    }

    #[test]
    #[should_panic(expected = "Cannot place a new settlement on top of an existing settlement.")]
    fn test_double_settlement() {
        let beginner_game = Game::from_template_settled("test_double_settlement".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please build a settlement first, before upgrading to a city.")]
    fn test_city_no_settlement() {
        let beginner_game = Game::from_template_settled("test_city_no_settlement".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot place a settlement directly adjacent to another settlement or city.")]
    fn test_too_close_settlements() {
        let beginner_game = Game::from_template_settled("test_too_close_settlements".to_string()).unwrap();
    }

    #[test]
    fn test_longest_road() {
        let beginner_game = Game::from_template_settled("test_longest_road".to_string()).unwrap();
        assert_eq!(beginner_game.round.board.scores, vec![4,2,2,2]);
        assert_eq!(beginner_game.round.board.longest_roads, Some(vec![5,1,1,1]));
    }

    #[test]
    fn test_round_road() {
        let beginner_game = Game::from_template_settled("test_round_road".to_string()).unwrap();
        // println!("{:#?}", beginner_game.round.board.roads);
        assert_eq!(beginner_game.round.board.scores, vec![4,2,2,2]);
        assert_eq!(beginner_game.round.board.longest_roads, Some(vec![6,1,1,1]));
    }

    #[test]
    fn test_forked_road() {
        let beginner_game = Game::from_template_settled("test_forked_road".to_string()).unwrap();
        // println!("{:#?}", beginner_game.round.board.roads);
        assert_eq!(beginner_game.round.board.scores, vec![4,2,2,2]);
        assert_eq!(beginner_game.round.board.longest_roads, Some(vec![6,1,1,1]));
    }
}
