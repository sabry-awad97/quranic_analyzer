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
    fn open_file() -> Result<File, QuranError> {
        File::open(QURAN_FILE_PATH)
            .map_err(|e| QuranError::FileOpenError(format!("{}: {}", QURAN_FILE_PATH, e)))
    }

    fn read_file(file: File) -> Result<BufReader<File>, QuranError> {
        Ok(BufReader::new(file))
    }

    fn parse_json(reader: BufReader<File>) -> Result<Vec<Surah>, QuranError> {
        serde_json::from_reader(reader)
            .map_err(|e| QuranError::JsonError(format!("{}: {}", QURAN_FILE_PATH, e)))
    }

    pub fn new() -> Result<Self, QuranError> {
        let file = Self::open_file()?;
        let reader = Self::read_file(file)?;
        let surahs = Self::parse_json(reader)?;

        Ok(Self { surahs })
    }
}
