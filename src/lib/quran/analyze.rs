use std::collections::HashMap;

use crate::traits::TotalLetters;

use super::Quran;

#[derive(Debug)]
pub struct Summary<'a> {
    pub total_surahs: usize,
    pub total_ayahs: usize,
    pub total_letters: usize,
    pub total_words: usize,
    pub longest_surah_name: &'a str,
    pub longest_surah_letters: usize,
    pub shortest_surah_name: &'a str,
    pub shortest_surah_letters: usize,
    pub most_common_word: Option<(String, i32)>,
}

impl Default for Summary<'_> {
    fn default() -> Self {
        Self {
            total_surahs: Default::default(),
            total_ayahs: Default::default(),
            total_letters: Default::default(),
            total_words: Default::default(),
            longest_surah_name: Default::default(),
            longest_surah_letters: Default::default(),
            shortest_surah_name: Default::default(),
            shortest_surah_letters: usize::MAX,
            most_common_word: Default::default(),
        }
    }
}

pub struct Analyzer<'a> {
    quran: &'a Quran,
}

impl Analyzer<'_> {
    pub fn new(quran: &Quran) -> Analyzer {
        Analyzer { quran }
    }

    pub fn analyze(&self) -> Summary {
        let mut word_counts = HashMap::new();
        let mut summary = Summary::default();

        for surah in self.quran.surahs() {
            let mut surah_letters = 0;
            for ayah in surah.ayas() {
                for word in ayah.words() {
                    let lowercase_word = word.to_lowercase();
                    *word_counts.entry(lowercase_word).or_insert(0) += 1;
                    summary.total_words += 1;
                }

                summary.total_ayahs += 1;
                surah_letters += ayah.total_letters();
            }

            if surah_letters > summary.longest_surah_letters {
                summary.longest_surah_letters = surah_letters;
                summary.longest_surah_name = surah.name();
            }

            if surah_letters < summary.shortest_surah_letters {
                summary.shortest_surah_letters = surah_letters;
                summary.shortest_surah_name = surah.name();
            }

            summary.total_letters += surah_letters;
            summary.total_surahs += 1;
        }

        summary.most_common_word = word_counts.into_iter().max_by(|a, b| a.1.cmp(&b.1));

        summary
    }
}
