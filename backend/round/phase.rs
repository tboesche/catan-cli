use serde::Deserialize;

use crate::backend::setup::{game::Game, node_status::NodeStatus::{Citied, Settled}};

use super::action::Action;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Phase {
    SetUp,
    RobberDiscard,
    RobberMove,
    FirstCardPhase,
    TradingQuote,
    TradingResponse,
    Building,
    SecondCardPhase,
    Terminal
}

impl Game {
    pub fn iterate_phase(&mut self, legal_action: &Action) {

        match legal_action {
            Action::SetUpMove(_, _) => {
                let n_players = &self.parameters.n_players;
                let i_setup = self.round.phase_count;
                
                if &i_setup < &(n_players - 1) {
                    // iterate player forward
                    self.round.phase_count += 1;
                    self.round.active_player = (self.round.active_player + 1) % n_players;
                } else if &i_setup == &(n_players - 1) {
                    // if i_setup == n_players, do not iterate player forward OR backward (snake draw)
                    self.round.phase_count += 1;
                } else if (i_setup < self.parameters.n_setup_rounds * n_players - 1) {
                    // iterate player backward
                    self.round.phase_count += 1;
                    self.round.active_player = (self.round.active_player - 1) % n_players;
                } else {
                    // throw dice after all setup has been done
                    self.throw_dice();
                    // println!("{:?}", self.round.board.dice_outcome);
                }

            },

            Action::Robber(_, _, _) => {
                self.round.phase = Phase::FirstCardPhase;
                self.round.active_player = self.round.throwing_player;
            },

            Action::DiscardCards(_) => {
                let n_players = &self.parameters.n_players;
                let i_discard = &self.round.phase_count;

                if i_discard < n_players {
                    // move forward the active player (TO DO: Random sequence of discarding players?)
                    self.round.active_player = (self.round.active_player + 1) % n_players;
                    self.round.phase_count += 1;
                } else {
                    self.round.active_player = self.round.throwing_player;
                    self.round.phase = Phase::RobberMove;
                    self.round.phase_count = 0;
                }
            },

            Action::NoDiscard => {
                let n_players = &self.parameters.n_players;
                let i_discard = &self.round.phase_count;

                if i_discard < n_players {
                    // move forward the active player (TO DO: Random sequence of discarding players?)
                    self.round.active_player = (self.round.active_player + 1) % n_players;
                    self.round.phase_count += 1;
                } else {
                    self.round.active_player = self.round.throwing_player;
                    self.round.phase = Phase::RobberMove;
                    self.round.phase_count = 0;
                }
            },

            Action::NoCardPlay => {
                let phase = &self.round.phase;

                // throw dice if end of second card phase has been reached
                if phase == &Phase::SecondCardPhase {
                    self.throw_dice();
                } else {
                    self.round.phase = Phase::TradingQuote;
                    self.round.phase_count = 0;
                }
            },

            Action::CardPlay(_) => {
                let phase = &self.round.phase;

                self.round.card_count += 1;

                // throw dice if end of second card phase has been reached
                if phase == &Phase::SecondCardPhase {
                    self.throw_dice();
                }
            },

            Action::NoTrade => {
                self.round.phase = Phase::Building;

            },

            Action::BankTrade(_, _) => {
                self.round.phase_count = ((&self.round.phase_count / self.parameters.n_players) + 1) * self.parameters.n_players;

                self.round.active_player = self.round.throwing_player;
            },

            Action::HarborTrade(_,_,_) => {
                self.round.phase_count = ((&self.round.phase_count / self.parameters.n_players) + 1) * self.parameters.n_players;

                self.round.active_player = self.round.throwing_player;
            },

            Action::TradeQuote(_) => {
                self.iterate_trade();
            },

            Action::TradeResponse(i_player, accept) => {
                let count_trades = &self.round.phase_count / self.parameters.n_players;
                if (*accept) & (count_trades < self.parameters.max_trades - 1) {
                    self.round.phase_count = ((&self.round.phase_count / self.parameters.n_players) + 1) * self.parameters.n_players;

                    self.round.active_player = self.round.throwing_player;
                    self.round.phase = Phase::TradingQuote;
                } else if *accept {
                    self.round.active_player = self.round.throwing_player;
                    self.round.phase = Phase::Building;
                    self.round.phase_count = 0;
                } else {
                    self.iterate_trade();
                }
            },

            Action::NoBuying => {
                self.round.phase = Phase::SecondCardPhase;
                self.round.phase_count = 0;
            },

            Action::BuildRoad(_, _) => {
                self.iterate_building();
            },

            Action::BuildSettlement(_) => {
                self.iterate_building();
            },

            Action::BuildCity(_) => {
                self.iterate_building();
            },

            Action::BuyDevCard => {
                self.iterate_building();
            },

            Action::FinishRound => {
                self.throw_dice();
            },

            Action::Save => {
                ()
            },

            Action::Quit => {
                unimplemented!()
            },
        }

        self.round.outcome = None;

    }

    fn produce_resources(&mut self) {

        let dice_outcome = self.round.board.dice_outcome;

        if let Some(d) = dice_outcome {
                let tiles = &self.round.board.tiles;

                for tile in tiles {
                    let tile_rng = match tile.rng {
                        Some(tr) => tr,
                        None => self.parameters.n_dice * self.parameters.n_faces + 1,
                    };

                    if tile_rng == d {

                        match tile.resource {
                            Some(i_resource) => {
                                let nodes = &tile.nodes;

                            for i_node in nodes {

                                match &self.round.board.nodes[*i_node as usize].node_status {
                                    Settled(i_player) => {
                                        self.round.board.budgets[*i_player as usize][i_resource as usize] += 1;
                                        self.round.board.public_budgets[*i_player as usize][i_resource as usize] += 1;
                                        self.round.board.total_drawn_resources[*i_player as usize][i_resource as usize] += 1;
                                    },
                                    Citied(i_player) => {
                                        self.round.board.budgets[*i_player as usize][i_resource as usize] += 2;
                                        self.round.board.total_drawn_resources[*i_player as usize][i_resource as usize] += 2;
                                    },
                                    _ => continue
                                }
                            }
                            },
                            None => continue,
                        }
                        
                    }
                }
            
        }
    
    }


    fn throw_dice(&mut self) {
        // throw dice
        self.round.board.dice = self.round.board.dice.clone().throw();
        self.round.board.dice_outcome = self.round.board.dice.draw;
        // println!("{:#?}",self.round.throwing_player);
        // println!("Phase: {:#?}", self.round.phase);


        // reset the phase counters to zero after throw of dice
        self.round.phase_count = 0;
        self.round.cards_played = 0;

        // After SetUp, reset the active player to 0. Otherwise, iterate active player by one.
        match &self.round.phase {
            Phase::SetUp => {
                self.round.throwing_player = 0;
            },
            _ => {
                self.round.throwing_player = (self.round.throwing_player + 1) % self.parameters.n_players;
            }
        }

        self.round.active_player = self.round.throwing_player.clone();

        // advance phase based on dice outcome
        let mut matched_robber_no = false;
        match self.round.board.dice_outcome {
            Some(dice_sum) => {
                for robber_no in &self.parameters.robber_nos {
                    if dice_sum == *robber_no {
                        self.round.phase = Phase::RobberDiscard;
                        matched_robber_no = true;
                        break
                    } else {
                        continue
                    }
                }
            },
            None => (),
        }

        if !matched_robber_no {
            self.produce_resources();
            self.round.phase = Phase::FirstCardPhase;
        }

        
    }

    fn iterate_trade(&mut self) {
        let count_trades = &self.round.phase_count / self.parameters.n_players;
        // println!("{}", count_trades);

        if count_trades <= self.parameters.max_trades {
            
            self.round.phase_count += 1;

            self.round.active_player = (self.round.throwing_player + self.round.phase_count) % self.parameters.n_players;

            if self.round.active_player == self.round.throwing_player {
                if count_trades < self.parameters.max_trades {
                    self.round.phase = Phase::TradingQuote;
                } else {
                    self.round.phase = Phase::Building;
                    self.round.phase_count = 0;
                }
                
            } else {
                self.round.phase = Phase::TradingResponse;
            }
        } else {
            self.round.phase = Phase::Building;
            self.round.phase_count = 0;
            self.round.active_player = self.round.throwing_player;
                }
    }

    fn iterate_building(&mut self) {
        self.round.phase_count += 1;

        if self.round.phase_count < self.parameters.max_builds {
            self.round.phase = Phase::Building
        } else {
            self.round.phase = Phase::SecondCardPhase;
            self.round.phase_count = 0;
        }
    }

}