use serde::{Deserialize, Serialize};

use crate::traits::{TotalLetters, TotalWords};

use super::verse::Ayah;

#[derive(Debug, Serialize, Deserialize)]
pub struct Surah {
    id: u32,
    name: String,
    transliteration: String,
    translation: String,
    #[serde(rename = "type")]
    revelation_type: String,
    total_verses: u32,
    #[serde(rename = "verses")]
    ayahs: Vec<Ayah>,
}

impl Surah {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn ayas(&self) -> &Vec<Ayah> {
        &self.ayahs
    }

    pub fn total_ayahs(&self) -> usize {
        self.ayahs.len()
    }
}

impl TotalLetters for Surah {
    fn total_letters(&self) -> usize {
        self.ayahs.iter().map(|verse| verse.total_letters()).sum()
    }
}

impl TotalWords for Surah {
    fn total_words(&self) -> usize {
        self.ayahs.iter().map(|verse| verse.total_words()).sum()
    }
}
