use quranic_analyzer::quran::{analyze::Analyzer, search::QuranSearch, Quran};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// Print the number of total letters in the Quran.
    #[structopt(short, long)]
    total_letters: bool,

    /// Print the number of words in the Quran.
    #[structopt(short, long)]
    words: bool,

    /// Print the longest chapter in the Quran.
    #[structopt(short, long)]
    longest_surah: bool,

    /// Print the shortest chapter in the Quran.
    #[structopt(short, long)]
    shortest_surah: bool,

    /// Print the most common word in the Quran.
    #[structopt(short, long)]
    most_common_word: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let quran = Quran::new()?;
    let analyzer = Analyzer::new(&quran);
    let summary = analyzer.analyze();

    if options.total_letters {
        println!("The Holy Quran has {} letters.", summary.total_letters);
    }

    if options.words {
        println!("The Holy Quran has {} words.", summary.total_words);
    }

    if options.longest_surah {
        println!(
            "The longest surah is {} with {} letters.",
            summary.longest_surah_name, summary.longest_surah_letters
        );
    }

    if options.shortest_surah {
        println!(
            "The shortest chapter is {} with {} letters.",
            summary.shortest_surah_name, summary.shortest_surah_letters
        );
    }

    if options.most_common_word {
        if let Some((word, count)) = summary.most_common_word {
            println!(
                "The most common word is {} with {} occurrences.",
                word, count
            );
        }
    }

    let mut searcher = QuranSearch::new(&quran);
    let search_results = searcher.search("الله");

    println!("Search Results:");
    if search_results.is_empty() {
        println!("No matching results found.");
    } else {
        for (surah_name, ayah_text) in &search_results {
            println!("Surah: {}", surah_name);
            println!("Ayah: {}", ayah_text);
            println!("---");
        }
    }
    println!("{}", search_results.len());
    Ok(())
}
