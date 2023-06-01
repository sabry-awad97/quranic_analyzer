use quranic_analyzer::quran::Quran;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut quran = Quran::new()?;
    let concordance = quran.generate_concordance();

    for (word, count) in concordance {
        println!("The word {} appears {} times in the Quran.", word, count);
    }

    println!("The concordance length is {}", concordance.len());
    Ok(())
}
