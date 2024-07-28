use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub enum NodeStatus {
    #[default] Free,
    Adjacent,
    Settled(u32),
    Citied(u32),
}