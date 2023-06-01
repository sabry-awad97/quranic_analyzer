use quranic_analyzer::quran::{summary::Summary, Quran};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quran = Quran::new()?;
    let summary = Summary::new(quran, "الله");
    summary.print();
    summary.print_search_results();
    Ok(())
}
