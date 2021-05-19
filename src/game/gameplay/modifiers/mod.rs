use serde::{Deserialize, Serialize};
use self::{curse::Curse, poison::Poison};

pub mod poison;
pub mod curse;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum ModifierBase {
    Poison(Poison),
    Curse(Curse)
}