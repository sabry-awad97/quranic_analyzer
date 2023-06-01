use super::Quran;

pub struct QuranSearch<'a> {
    quran: &'a Quran,
}

impl<'a> QuranSearch<'a> {
    pub fn new(quran: &'a Quran) -> QuranSearch<'a> {
        QuranSearch { quran }
    }

    pub fn search(&mut self, search_term: &str) -> Vec<(String, usize, String)> {
        let search_results = self
            .quran
            .surahs()
            .iter()
            .flat_map(|surah| {
                surah
                    .ayahs()
                    .iter()
                    .filter(|ayah| ayah.contains_word(search_term))
                    .map(move |ayah| {
                        (
                            surah.name().to_string(),
                            ayah.number(),
                            ayah.text().to_string(),
                        )
                    })
            })
            .collect::<Vec<_>>();

        search_results
    }

    /// Searches for verses that contain multiple terms.
    pub fn search_multiple_terms(
        &mut self,
        search_terms: Vec<&str>,
    ) -> Vec<(String, usize, String)> {
        let mut search_results = vec![];

        for search_term in search_terms {
            let mut temp_results = self.search(search_term);
            search_results.append(&mut temp_results);
        }

        search_results
    }

    /// Searches for verses that are located within a certain range of ayahs.
    pub fn search_range(
        &mut self,
        start_ayah_number: usize,
        end_ayah_number: usize,
        search_term: &str,
    ) -> Vec<(String, usize, String)> {
        let mut search_results = vec![];

        for ayah in self
            .quran
            .ayas()
            .iter()
            .skip(start_ayah_number)
            .take(end_ayah_number - start_ayah_number + 1)
        {
            if ayah.contains_word(search_term) {
                search_results.push((
                    ayah.surah_name().to_string(),
                    ayah.number(),
                    ayah.text().to_string(),
                ));
            }
        }

        search_results
    }

    /// Searches for verses that are located in a certain surah.
    pub fn search_surah(
        &mut self,
        search_term: &str,
        surah_number: usize,
    ) -> Vec<(String, usize, String)> {
        let mut search_results = vec![];

        for ayah in self.quran.surahs()[surah_number - 1].ayahs().iter() {
            if ayah.contains_word(search_term) {
                search_results.push((
                    ayah.surah_name().to_string(),
                    ayah.number(),
                    ayah.text().to_string(),
                ));
            }
        }

        search_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test Quran instance
    fn create_test_quran() -> Quran {
        Quran::new().unwrap()
    }

    #[test]
    fn test_search() {
        // Create a test Quran instance
        let quran = create_test_quran();

        // Create a QuranSearch instance
        let mut search = QuranSearch::new(&quran);

        let search_term = "بسم الله الرحمن الرحيم";

        // Perform the search
        let search_results = search.search(search_term);
        assert_eq!(search_results.len(), 2);
    }

    #[test]
    fn test_search_surah() {
        // Create a test Quran instance
        let quran = create_test_quran();

        // Create a QuranSearch instance
        let mut search = QuranSearch::new(&quran);

        // Perform the search within a specific surah
        let search_results = search.search_surah("الله", 1);

        // Assert the expected results
        assert_eq!(search_results.len(), 1);
        // Add more assertions as needed based on your test case
    }

    #[test]
    fn test_search_range() {
        let quran = create_test_quran();

        // Search for verses in the range 1-5 containing the term "Allah"
        let mut search = QuranSearch::new(&quran);

        let search_results = search.search_range(1, 5, "الله");

        assert_eq!(search_results.len(), 0);
    }
}
