use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Verse {
    id: u32,
    text: String,
    translation: String,
}
