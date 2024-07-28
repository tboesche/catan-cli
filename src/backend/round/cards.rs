use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub enum CardType {
    VPCard,
    KnightCard(u32, u32, Option<u32>),
    RoadsCard(u32, u32, u32, u32),
    PlentyCard(u32, u32),
    MonopolyCard(u32),
}