use crate::traits::{TotalLetters, TotalWords};

use super::verse::Ayah;

#[derive(Debug)]
pub struct Surah {
    pub(in crate::quran) id: u32,
    pub(in crate::quran) name: String,
    pub(in crate::quran) total_verses: u32,
    pub(in crate::quran) ayahs: Vec<Ayah>,
}

impl Surah {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn ayas(&self) -> &Vec<Ayah> {
        &self.ayahs
    }

    pub fn total_ayahs(&self) -> usize {
        self.total_verses as usize
    }
}

impl TotalLetters for Surah {
    fn total_letters(&self) -> usize {
        self.ayahs.iter().map(|verse| verse.total_letters()).sum()
    }
}

impl TotalWords for Surah {
    fn total_words(&self) -> usize {
        self.ayahs.iter().map(|verse| verse.total_words()).sum()
    }
}
