pub mod summary;
pub mod surah;
pub mod verse;

use std::{fs::File, io::BufReader};

use surah::Surah;

use crate::error::QuranError;

const QURAN_FILE_PATH: &str = "quran.json";

pub struct Quran {
    surahs: Vec<Surah>,
}

impl Quran {
    pub fn new() -> Result<Self, QuranError> {
        let file = Self::open_file()?;
        let reader = Self::read_file(file)?;
        let surahs: Vec<Surah> = Self::parse_json(reader)?;

        Ok(Self { surahs })
    }

    pub fn surahs(&self) -> &[Surah] {
        &self.surahs
    }

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
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_open_file() {
        let file = Quran::open_file().unwrap();
        assert!(file.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_read_file() {
        let file = Quran::open_file().unwrap();
        let mut reader = Quran::read_file(file).unwrap();
        let mut buffer = [0; 10];
        let bytes_read = reader.read(&mut buffer).unwrap();
        assert_eq!(bytes_read, 10);
    }

    #[test]
    fn test_parse_json() {
        let file = Quran::open_file().unwrap();
        let reader = Quran::read_file(file).unwrap();
        let surahs = Quran::parse_json(reader).unwrap();
        assert_eq!(surahs.len(), 114);
    }

    #[test]
    fn test_new() {
        let quran = Quran::new().unwrap();
        assert_eq!(quran.surahs.len(), 114);
    }
}
