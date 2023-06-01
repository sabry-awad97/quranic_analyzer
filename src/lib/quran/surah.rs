use serde::{Deserialize, Serialize};

use super::verse::Verse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Surah {
    id: u32,
    name: String,
    transliteration: String,
    translation: String,
    #[serde(rename = "type")]
    type_: String,
    total_verses: u32,
    verses: Vec<Verse>,
}
