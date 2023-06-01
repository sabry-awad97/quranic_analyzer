use quranic_analyzer::error::QuranError;
use quranic_analyzer::quran::surah::Surah;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), QuranError> {
    let file_path = "quran.json";
    let file = File::open(file_path)
        .map_err(|e| QuranError::FileOpenError(format!("{}: {}", file_path, e)))?;
    let reader = BufReader::new(file);

    let quran: Vec<Surah> = serde_json::from_reader(reader)
        .map_err(|e| QuranError::JsonError(format!("{}: {}", file_path, e)))?;

    println!("{:?}", quran.len());

    Ok(())
}
