use prettytable::{Cell, Row, Table};

#[derive(Debug)]
pub struct Summary {
    pub total_chapters: usize,
}

impl Summary {
    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
        table.set_titles(Row::new(vec![Cell::new("Summary")]));

        table.add_row(Row::new(vec![Cell::new("key"), Cell::new("value")]));

        table.add_row(Row::new(vec![
            Cell::new("Total Chapters"),
            Cell::new(&self.total_chapters.to_string()),
        ]));

        table.printstd();
    }
}
