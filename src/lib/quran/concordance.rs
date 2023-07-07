use std::{collections::HashMap, fs::File};

use prettytable::Table;

use super::Quran;

pub struct Concordance<'a> {
    quran: &'a Quran,
    concordance: HashMap<String, (usize, Vec<(usize, String, String)>)>,
}

impl<'a> Concordance<'a> {
    pub fn new(quran: &'a Quran) -> Self {
        Self {
            quran,
            concordance: HashMap::new(),
        }
    }

    pub fn generate<T, U, V>(&mut self, surah_number: T, start_ayah_number: U, end_ayah_number: V)
    where
        T: Into<Option<usize>>,
        U: Into<Option<usize>>,
        V: Into<Option<usize>>,
    {
        let start_ayah_number = start_ayah_number.into().unwrap_or(0);

        if let Some(mut surah_number) = surah_number.into() {
            if surah_number == 0 {
                surah_number = 1
            }
            let surah = self.quran.surah(surah_number - 1);
            let ayahs_count = surah.ayahs().len();

            let mut end_ayah_number = end_ayah_number.into().unwrap_or(ayahs_count);

            if end_ayah_number > ayahs_count {
                end_ayah_number = ayahs_count;
            }

            if end_ayah_number < start_ayah_number {
                end_ayah_number = start_ayah_number;
            }

            for ayah in &surah.ayahs()[start_ayah_number..end_ayah_number] {
                for word in ayah.words() {
                    self.concordance
                        .entry(word.to_string())
                        .or_insert((0, vec![]))
                        .0 += 1;
                    self.concordance.get_mut(word).unwrap().1.push((
                        ayah.number(),
                        surah.name().to_owned(),
                        ayah.text().to_string(),
                    ));
                }
            }
        } else {
            for surah in self.quran.surahs() {
                for ayah in surah.ayahs() {
                    for word in ayah.words() {
                        self.concordance
                            .entry(word.to_string())
                            .or_insert((0, vec![]))
                            .0 += 1;
                        self.concordance.get_mut(word).unwrap().1.push((
                            ayah.number(),
                            surah.name().to_owned(),
                            ayah.text().to_string(),
                        ));
                    }
                }
            }
        }
    }

    pub fn print_to_file(&self, file_path: &str) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

        let width = 15;

        table.add_row(vec!["الكلمة", "العدد", "رقم الأية", "السورة", "الأية"].into());

        let mut sorted_pairs = self.concordance.iter().collect::<Vec<_>>();
        sorted_pairs.sort_by_key(|(_, (count, _))| *count);
        sorted_pairs.reverse();

        let mut total_count = 0;
        for (word, (count, ayahs)) in sorted_pairs {
            total_count += count;

            table.add_row(vec![word, &count.to_string(), "", ""].into());

            for (ayah_number, sura, ayah_text) in ayahs {
                table.add_row(vec!["", "", &ayah_number.to_string(), &sura.to_string(), ayah_text].into());
            }
        }

        table.add_row(
            vec![
                "-".repeat(width),
                "-".repeat(width),
                "-".repeat(width),
                "-".repeat(width),
                "-".repeat(width),
            ]
            .into(),
        );

        table.add_row(vec!["العدد الكلى", &total_count.to_string(), "", ""].into());

        for row in table.row_iter_mut() {
            for cell in row.iter_mut() {
                cell.align(prettytable::format::Alignment::CENTER)
            }
        }

        let mut file = File::create(file_path).expect("Could not create file");
        table.print(&mut file).unwrap();
    }
}
