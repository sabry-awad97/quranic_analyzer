use std::{collections::HashMap, fs::File};

use prettytable::Table;

use crate::remove_diacritics;

use super::Quran;

pub struct Concordance<'a> {
    quran: &'a Quran,
    concordance: HashMap<String, (usize, Vec<(usize, String)>)>,
}

impl<'a> Concordance<'a> {
    pub fn new(quran: &'a Quran) -> Self {
        Self {
            quran,
            concordance: HashMap::new(),
        }
    }

    pub fn generate(&mut self) {
        for surah in self.quran.surahs() {
            for ayah in surah.ayas() {
                for word in ayah.words() {
                    let word = remove_diacritics(word);
                    self.concordance
                        .entry(word.to_string())
                        .or_insert((0, vec![]))
                        .0 += 1;
                    self.concordance
                        .get_mut(&word)
                        .unwrap()
                        .1
                        .push((ayah.number(), surah.name().to_owned()));
                }
            }
        }
    }

    pub fn print_to_file(&self, file_path: &str) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

        table.add_row(vec!["Word", "Count", "Ayah", "Sura"].into());

        let mut sorted_pairs = self.concordance.iter().collect::<Vec<_>>();
        sorted_pairs.sort_by_key(|(key, _)| *key);
        for (word, (count, ayahs)) in sorted_pairs {
            for (ayah, sura) in ayahs {
                table.add_row(
                    vec![
                        word,
                        &count.to_string(),
                        &ayah.to_string(),
                        &sura.to_string(),
                    ]
                    .into(),
                );
            }
        }

        let mut file = File::create(file_path).expect("Could not create file");
        table.print(&mut file).unwrap();
    }
}
