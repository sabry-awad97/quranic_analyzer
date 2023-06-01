use serde::{Deserialize, Serialize};

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
    pub fn total_ayahs(&self) -> usize {
        self.ayahs.len()
    }
}
