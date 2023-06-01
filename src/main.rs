use quranic_analyzer::quran::{concordance::Concordance, Quran};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "s", long = "surah")]
    surah_number: usize,

    #[structopt(short = "a", long = "start_ayah")]
    start_ayah_number: usize,

    #[structopt(short = "e", long = "end_ayah")]
    end_ayah_number: usize,

    #[structopt(short = "o", long = "output_file", default_value = "concordance.txt")]
    output_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let quran = Quran::new()?;
    let mut concordance = Concordance::new(&quran);

    concordance.generate(
        options.surah_number,
        options.start_ayah_number,
        options.end_ayah_number,
    );
    concordance.print_to_file(&options.output_file);

    Ok(())
}
