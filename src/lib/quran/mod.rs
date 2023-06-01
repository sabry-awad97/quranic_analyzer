pub mod summary;
pub mod surah;
pub mod verse;

use std::{collections::HashMap, fs::File, io::BufReader};

use summary::Summary;
use surah::Surah;

use crate::{error::QuranError, traits::TotalLetters};

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

    pub fn summarize(&self) -> Summary {
        let mut word_counts = HashMap::new();

        let mut total_surahs = 0;
        let mut total_ayahs = 0;
        let mut total_letters = 0;
        let mut total_words = 0;
        let mut longest_surah_name = "";
        let mut longest_surah_letters = 0;
        let mut shortest_surah_name = "";
        let mut shortest_surah_letters = usize::MAX;
        for surah in &self.surahs {
            let mut surah_letters = 0;
            for ayah in surah.ayas() {
                for word in ayah.words() {
                    let lowercase_word = word.to_lowercase();
                    *word_counts.entry(lowercase_word).or_insert(0) += 1;
                    total_words += 1;
                }

                total_ayahs += 1;
                surah_letters += ayah.total_letters();
            }

            if surah_letters > longest_surah_letters {
                longest_surah_letters = surah_letters;
                longest_surah_name = surah.name();
            }

            if surah_letters < shortest_surah_letters {
                shortest_surah_letters = surah_letters;
                shortest_surah_name = surah.name();
            }

            total_letters += surah_letters;
            total_surahs += 1;
        }

        let most_common_word = word_counts.into_iter().max_by(|a, b| a.1.cmp(&b.1));

        Summary {
            total_surahs,
            total_ayahs,
            average_letters_per_surah: total_letters / total_surahs,
            average_words_per_surah: total_words / total_surahs,
            longest_surah_name: longest_surah_name.to_string(),
            longest_surah_letters,
            shortest_surah_name: shortest_surah_name.to_string(),
            shortest_surah_letters,
            most_common_word,
        }
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
