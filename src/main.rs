use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Debug, Error)]
enum QuranError {
    #[error("failed to open file: {0}")]
    FileOpenError(String),
    #[error("failed to parse JSON: {0}")]
    JsonError(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Surah {
    id: u32,
    name: String,
    transliteration: String,
    translation: String,
    #[serde(rename = "type")]
    type_: String,
    total_verses: u32,
    verses: Vec<Verse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Verse {
    id: u32,
    text: String,
    translation: String,
}

fn main() -> Result<(), QuranError> {
    let file_path = "quran.json";
    let file = File::open(file_path)
        .map_err(|e| QuranError::FileOpenError(format!("{}: {}", file_path, e)))?;
    let reader = BufReader::new(file);

    let quran: Vec<Surah> = serde_json::from_reader(reader)
        .map_err(|e| QuranError::JsonError(format!("{}: {}", file_path, e)))?;

    println!("{:?}", quran.len());

    Ok(())
}
