use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Outcome {
    RobberOutcome(Option<u32>),
    DrawCardOutcome(u32),
    TradeOutcome(u32, bool),
}