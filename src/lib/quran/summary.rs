use prettytable::{Cell, Row, Table};

#[derive(Debug)]
pub struct Summary {
    pub total_surahs: usize,
    pub total_ayahs: usize,
    pub average_letters_per_surah: usize,
    pub average_words_per_surah: usize,
}

impl Summary {
    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
        table.set_titles(Row::new(vec![Cell::new("Summary")]));

        table.add_row(Row::new(vec![Cell::new("key"), Cell::new("value")]));

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

        table.printstd();
    }
}
