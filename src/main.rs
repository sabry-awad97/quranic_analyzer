use quranic_analyzer::quran::Quran;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quran = Quran::new()?;
    let summary = quran.summarize();
    summary.print();
    Ok(())
}
