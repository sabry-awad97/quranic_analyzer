use serde::{Deserialize, Serialize};

use crate::traits::TotalLetters;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayah {
    id: u32,
    text: String,
    translation: String,
}

impl TotalLetters for Ayah {
    fn total_letters(&self) -> usize {
        self.text.chars().count()
    }
}
