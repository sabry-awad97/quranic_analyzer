use serde::{Deserialize, Serialize};

use crate::traits::{TotalLetters, TotalWords};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayah {
    id: u32,
    text: String,
    translation: String,
}

impl Ayah {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl TotalLetters for Ayah {
    fn total_letters(&self) -> usize {
        self.text.chars().count()
    }
}

impl TotalWords for Ayah {
    fn total_words(&self) -> usize {
        self.text.trim().split_whitespace().count()
    }
}
