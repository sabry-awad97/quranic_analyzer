use std::collections::HashMap;

use prettytable::{Cell, Row, Table};

use crate::traits::TotalLetters;

use super::Quran;

#[derive(Debug)]
pub struct Summary {
    pub total_surahs: usize,
    pub total_ayahs: usize,
    pub average_letters_per_surah: usize,
    pub average_words_per_surah: usize,
    pub longest_surah_name: String,
    pub longest_surah_letters: usize,
    pub shortest_surah_name: String,
    pub shortest_surah_letters: usize,
    pub most_common_word: Option<(String, i32)>,
    pub search_results: Vec<(String, String)>,
}

impl Summary {
    pub fn new(quran: Quran, search_term: &str) -> Summary {
        let mut word_counts = HashMap::new();

        let mut total_surahs = 0;
        let mut total_ayahs = 0;
        let mut total_letters = 0;
        let mut total_words = 0;
        let mut longest_surah_name = "";
        let mut longest_surah_letters = 0;
        let mut shortest_surah_name = "";
        let mut shortest_surah_letters = usize::MAX;
        for surah in quran.surahs() {
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

        let search_results: Vec<(String, String)> = quran
            .surahs
            .iter()
            .flat_map(|surah| {
                surah
                    .ayas()
                    .iter()
                    .filter(|ayah| ayah.contains_word(search_term))
                    .map(move |ayah| (surah.name().to_string(), ayah.text().to_string()))
            })
            .collect();

        let most_common_word = word_counts.into_iter().max_by(|a, b| a.1.cmp(&b.1));

        let average_letters_per_surah = total_letters / total_surahs;
        let average_words_per_surah = total_words / total_surahs;

        Self {
            total_surahs,
            total_ayahs,
            average_letters_per_surah,
            average_words_per_surah,
            longest_surah_name: longest_surah_name.to_string(),
            longest_surah_letters,
            shortest_surah_name: shortest_surah_name.to_string(),
            shortest_surah_letters,
            most_common_word,
            search_results,
        }
    }

    pub fn print_search_results(&self) {
        println!("Search Results:");
        if self.search_results.is_empty() {
            println!("No matching results found.");
        } else {
            for (surah_name, ayah_text) in &self.search_results {
                println!("Surah: {}", surah_name);
                println!("Ayah: {}", ayah_text);
                println!("---");
            }
        }
        println!("{}", self.search_results.len());
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
        table.set_titles(Row::new(vec![Cell::new("Summary")]));

        table.add_row(Row::new(vec![
            Cell::new("Total Surahs"),
            Cell::new(&self.total_surahs.to_string()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Total Ayahs"),
            Cell::new(&self.total_ayahs.to_string()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Average Letters per Surah"),
            Cell::new(&self.average_letters_per_surah.to_string()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Average Words per Surah"),
            Cell::new(&self.average_words_per_surah.to_string()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Longest Surah Name"),
            Cell::new(&self.longest_surah_name),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Longest Surah Letters"),
            Cell::new(&self.longest_surah_letters.to_string()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Shortest Surah Name"),
            Cell::new(&self.shortest_surah_name),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Shortest Surah Letters"),
            Cell::new(&self.shortest_surah_letters.to_string()),
        ]));

        if let Some((word, count)) = &self.most_common_word {
            table.add_row(Row::new(vec![
                Cell::new("Most Common Word"),
                Cell::new(&format!("{} ", word)),
            ]));

            table.add_row(Row::new(vec![
                Cell::new("Most Common Word Occurrences"),
                Cell::new(&count.to_string()),
            ]));
        }

        table.printstd();
    }
}
