use serde::{Deserialize, Serialize};

use crate::{
    remove_diacritics,
    traits::{TotalLetters, TotalWords},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayah {
    id: u32,
    text: String,
    translation: String,
}

impl Ayah {
    pub fn words(&self) -> Vec<&str> {
        self.text.trim().split_whitespace().collect()
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn contains_word(&self, search_term: &str) -> bool {
        let stripped = remove_diacritics(&self.text);
        stripped.contains(search_term)
    }
}

impl TotalLetters for Ayah {
    fn total_letters(&self) -> usize {
        self.text.chars().count()
    }
}

impl TotalWords for Ayah {
    fn total_words(&self) -> usize {
        self.words().len()
    }
}
