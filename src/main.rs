use quranic_analyzer::quran::{concordance::Concordance, Quran};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quran = Quran::new()?;
    let mut concordance = Concordance::new(&quran);

    concordance.generate();

    concordance.print_to_file("concordance.txt");

    Ok(())
}
