use serde::{Deserialize, Serialize};
use super::StatName;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Stat {
    pub name: StatName,
    pub value: f32,
    pub max: f32,
}

impl Stat {
    pub fn new(name: StatName, value: f32) -> Self {
        Self {
            name,
            value,
            max: 0.0,
        }
    }
}
