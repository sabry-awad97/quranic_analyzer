use quranic_analyzer::quran::{concordance::Concordance, Quran};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quran = Quran::new()?;
    let mut concordance = Concordance::new(&quran);

    concordance.generate(0, None, None);

    concordance.print_to_file("concordance_fatiha.txt");

    Ok(())
}
