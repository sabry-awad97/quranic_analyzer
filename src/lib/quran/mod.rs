pub mod surah;
pub mod verse;

use std::{fs::File, io::BufReader};

use surah::Surah;

use crate::error::QuranError;

const QURAN_FILE_PATH: &str = "quran.json";

struct Quran {
    surahs: Vec<Surah>,
}

impl Quran {
    pub fn new() -> Result<Self, QuranError> {
        let file = File::open(QURAN_FILE_PATH)
            .map_err(|e| QuranError::FileOpenError(format!("{}: {}", QURAN_FILE_PATH, e)))?;
        let reader = BufReader::new(file);

        let surahs: Vec<Surah> = serde_json::from_reader(reader)
            .map_err(|e| QuranError::JsonError(format!("{}: {}", QURAN_FILE_PATH, e)))?;

        Ok(Self { surahs })
    }
}
