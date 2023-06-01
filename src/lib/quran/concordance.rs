use std::{collections::HashMap, fs::File};

use prettytable::Table;

use crate::remove_diacritics;

use super::Quran;

pub struct Concordance<'a> {
    quran: &'a Quran,
    concordance: HashMap<String, usize>,
}

impl<'a> Concordance<'a> {
    pub fn new(quran: &'a Quran) -> Self {
        Self {
            quran,
            concordance: HashMap::new(),
        }
    }

    pub fn generate(&mut self) -> &HashMap<String, usize> {
        for surah in &self.quran.surahs {
            for ayah in surah.ayas() {
                for word in ayah.words() {
                    let stripped = remove_diacritics(word);
                    *self.concordance.entry(stripped).or_insert(0) += 1;
                }
            }
        }
        &self.concordance
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

        table.add_row(vec!["Word", "Count"].into());

        for (word, count) in &self.concordance {
            table.add_row(vec![word, &count.to_string()].into());
        }

        table.printstd();
    }

    pub fn print_to_file(&self, file_path: &str) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

        table.add_row(vec!["Word", "Count"].into());

        for (word, count) in &self.concordance {
            table.add_row(vec![word, &count.to_string()].into());
        }

        let mut file = File::create(file_path).expect("Could not create file");
        table.print(&mut file).unwrap();
    }
}
