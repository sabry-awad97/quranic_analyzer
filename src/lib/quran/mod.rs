pub mod analyze;
pub mod concordance;
pub mod search;
pub mod surah;
pub mod verse;

use surah::Surah;

use crate::error::QuranError;

use self::verse::Ayah;

const QURAN_FILE_PATH: &str = "../../data/quran.json";

pub struct Quran {
    surahs: Vec<Surah>,
}

impl Quran {
    pub fn new() -> Result<Self, QuranError> {
        let str_value = include_str!("../../data/quran.json");
        let json_value: serde_json::Value = serde_json::from_str(str_value)
            .map_err(|e| QuranError::JsonError(format!("{}: {}", QURAN_FILE_PATH, e)))?;
        let surahs: Vec<Surah> = Self::parse_surahs(json_value)?;

        Ok(Self { surahs })
    }

    pub fn surahs(&self) -> &[Surah] {
        &self.surahs
    }

    pub fn surah(&self, surah_number: usize) -> &Surah {
        &self.surahs[surah_number]
    }

    pub fn ayas(&self) -> Vec<&Ayah> {
        self.surahs
            .iter()
            .map(|s| s.ayahs())
            .clone()
            .flatten()
            .collect::<Vec<_>>()
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
    use super::*;

    #[test]
    fn test_new() {
        let quran = Quran::new().unwrap();
        assert_eq!(quran.surahs.len(), 114);
    }
}
