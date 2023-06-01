use quranic_analyzer::quran::{analyze::Analyzer, search::QuranSearch, Quran};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quran = Quran::new()?;
    let summary = Analyzer::new(&quran);
    summary.print();

    let mut searcher = QuranSearch::new(&quran);
    searcher.search("الله");
    Ok(())
}
