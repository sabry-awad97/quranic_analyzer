use crate::{
    remove_diacritics,
    traits::{TotalLetters, TotalWords},
};

#[derive(Debug)]
pub struct Ayah {
    pub(in crate::quran) ayah_number: u32,
    pub(in crate::quran) surah_name: String,
    pub(in crate::quran) text: String,
}

impl Ayah {
    pub fn words(&self) -> Vec<&str> {
        self.text.trim().split_whitespace().collect()
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn surah_name(&self) -> &str {
        self.surah_name.as_str()
    }

    pub fn number(&self) -> usize {
        self.ayah_number as usize
    }

    pub fn contains_word(&self, search_term: &str) -> bool {
        let search_term = &remove_diacritics(search_term);
        let stripped = remove_diacritics(&self.text);
        stripped.contains(search_term)
    }
}

impl TotalLetters for Ayah {
    fn total_letters(&self) -> usize {
        self.text.chars().count()
    }
}

impl TotalWords for Ayah {
    fn total_words(&self) -> usize {
        self.words().len()
    }
}
