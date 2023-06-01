use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayah {
    id: u32,
    text: String,
    translation: String,
}
