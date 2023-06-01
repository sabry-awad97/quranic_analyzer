pub mod summary;
pub mod surah;
pub mod verse;

use std::{collections::HashMap, fs::File, io::BufReader};

use summary::Summary;
use surah::Surah;

use crate::{
    error::QuranError,
    traits::{TotalLetters, TotalWords},
};

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
        let longest_surah = self.longest_surah().unwrap();
        let shortest_surah = self.shortest_surah().unwrap();
        let most_common_word = self.most_common_word();
        let word = most_common_word.clone().map(|(word, _)| word);
        let count = most_common_word.map(|(_, count)| count);
        Summary {
            total_surahs: self.total_surahs(),
            total_ayahs: self.total_ayahs(),
            average_letters_per_surah: self.average_letters_per_surah(),
            average_words_per_surah: self.average_words_per_surah(),
            longest_surah_name: longest_surah.name().to_string(),
            longest_surah_letters: longest_surah.total_letters(),
            shortest_surah_name: shortest_surah.name().to_string(),
            shortest_surah_letters: shortest_surah.total_letters(),
            most_common_word: word.unwrap(),
            most_common_word_occurrences: count.unwrap(),
        }
    }

    fn most_common_word(&self) -> Option<(String, i32)> {
        let mut word_counts = HashMap::new();
        for surah in &self.surahs {
            for ayah in surah.ayas() {
                for word in ayah.text().split_whitespace() {
                    let lowercase_word = word.to_lowercase();
                    *word_counts.entry(lowercase_word).or_insert(0) += 1;
                }
            }
        }

        word_counts.into_iter().max_by(|a, b| a.1.cmp(&b.1))
    }

    fn shortest_surah(&self) -> Option<&Surah> {
        self.surahs
            .iter()
            .min_by(|&a, &b| a.total_letters().cmp(&b.total_letters()))
    }

    fn longest_surah(&self) -> Option<&Surah> {
        self.surahs
            .iter()
            .max_by(|&a, &b| a.total_letters().cmp(&b.total_letters()))
    }

    fn average_words_per_surah(&self) -> usize {
        self.surahs
            .iter()
            .map(|chapter| chapter.total_words())
            .sum::<usize>()
            / self.surahs.len()
    }

    fn average_letters_per_surah(&self) -> usize {
        self.surahs
            .iter()
            .map(|chapter| chapter.total_letters())
            .sum::<usize>()
            / self.surahs.len()
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

    fn total_surahs(&self) -> usize {
        self.surahs.len()
    }

    fn total_ayahs(&self) -> usize {
        self.surahs.iter().map(|surah| surah.total_ayahs()).sum()
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
