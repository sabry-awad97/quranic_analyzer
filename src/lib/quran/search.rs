use super::Quran;

pub struct QuranSearch<'a> {
    quran: &'a Quran,
    search_results: Vec<(String, String)>,
}

impl<'a> QuranSearch<'a> {
    pub fn new(quran: &'a Quran) -> QuranSearch<'a> {
        QuranSearch {
            quran,
            search_results: vec![],
        }
    }

    pub fn search(&mut self, search_term: &str) -> Vec<(String, String)> {
        let search_results = self
            .quran
            .surahs()
            .iter()
            .flat_map(|surah| {
                surah
                    .ayas()
                    .iter()
                    .filter(|ayah| ayah.contains_word(search_term))
                    .map(move |ayah| (surah.name().to_string(), ayah.text().to_string()))
            })
            .collect::<Vec<_>>();

        self.search_results = search_results.clone();

        search_results
    }

    pub fn print_search_results(&self) {
        println!("Search Results:");
        if self.search_results.is_empty() {
            println!("No matching results found.");
        } else {
            for (surah_name, ayah_text) in &self.search_results {
                println!("Surah: {}", surah_name);
                println!("Ayah: {}", ayah_text);
                println!("---");
            }
        }
        println!("{}", self.search_results.len());
    }
}