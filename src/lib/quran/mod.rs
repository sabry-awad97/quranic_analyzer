pub mod analyze;
pub mod concordance;
pub mod search;
pub mod surah;
pub mod verse;

use std::{fs::File, io::BufReader};

use surah::Surah;

use crate::error::QuranError;

use self::verse::Ayah;

const QURAN_FILE_PATH: &str = "quran.json";

pub struct Quran {
    surahs: Vec<Surah>,
}

impl Quran {
    pub fn new() -> Result<Self, QuranError> {
        let json_value = Self::read_json()?;
        let surahs: Vec<Surah> = Self::parse_surahs(json_value)?;

        Ok(Self { surahs })
    }

    pub fn surahs(&self) -> &[Surah] {
        &self.surahs
    }

    fn read_json() -> Result<serde_json::Value, QuranError> {
        let file = Self::open_file()?;
        let reader = Self::read_file(file)?;
        let json_value: serde_json::Value = serde_json::from_reader(reader)
            .map_err(|e| QuranError::JsonError(format!("{}: {}", QURAN_FILE_PATH, e)))?;

        Ok(json_value)
    }

    fn open_file() -> Result<File, QuranError> {
        File::open(QURAN_FILE_PATH)
            .map_err(|e| QuranError::FileOpenError(format!("{}: {}", QURAN_FILE_PATH, e)))
    }

    fn read_file(file: File) -> Result<BufReader<File>, QuranError> {
        Ok(BufReader::new(file))
    }

    fn parse_surahs(json_value: serde_json::Value) -> Result<Vec<Surah>, QuranError> {
        if let serde_json::Value::Array(surahs) = json_value {
            let mut parsed_surahs = Vec::new();

            for surah in surahs {
                let id = surah["id"]
                    .as_u64()
                .ok_or(QuranError::JsonError("Invalid surah id".to_string()))?
                    as u32;

                let surah_name = surah["name"]
                    .as_str()
                    .ok_or(QuranError::JsonError("Invalid surah name".to_string()))?
                    .to_string();

                let total_verses = surah["total_verses"]
                    .as_u64()
                    .ok_or(QuranError::JsonError("Invalid total verses".to_string()))?
                    as u32;

                let ayahs = surah["verses"]
                    .as_array()
                    .ok_or(QuranError::JsonError("Invalid verses".to_string()))?;

                let mut parsed_ayahs = Vec::new();

                for ayah in ayahs {
                    let ayah_number = ayah["id"]
                        .as_u64()
                        .ok_or(QuranError::JsonError("Invalid ayah id".to_string()))?
                        as u32;

                    let ayah_text = ayah["text"]
                        .as_str()
                        .ok_or(QuranError::JsonError("Invalid ayah text".to_string()))?
                        .to_string();

                    let ayah = Ayah {
                        ayah_number,
                        surah_name: surah_name.clone(),
                        text: ayah_text,
                    };

                    parsed_ayahs.push(ayah);
                }

                let surah = Surah {
                    name: surah_name,
                    id,
                    total_verses,
                    ayahs: parsed_ayahs,
                };

                parsed_surahs.push(surah);
            }

            Ok(parsed_surahs)
        } else {
            Err(QuranError::JsonError("Invalid JSON structure".to_string()))
        }
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
    fn test_new() {
        let quran = Quran::new().unwrap();
        assert_eq!(quran.surahs.len(), 114);
    }
}
