const FATHATAN: char = '\u{064B}';
const DAMMATAN: char = '\u{064C}';
const KASRATAN: char = '\u{064D}';
const FATHA: char = '\u{064E}';
const DAMMA: char = '\u{064F}';
const KASRA: char = '\u{0650}';
const SHADDA: char = '\u{0651}';
const SUKUN: char = '\u{0652}';

const TASHKEEL: [char; 8] = [
    FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SUKUN, SHADDA,
];

#[allow(unused)]
fn strip_tashkeel(text: &str) -> String {
    let mut stripped = String::new();
    for c in text.chars() {
        if !is_tashkeel(c) {
            stripped.push(c);
        }
    }
    stripped
}

#[allow(unused)]
fn is_vocalized(word: &str) -> bool {
    word.chars().any(|c| is_tashkeel(c))
}

fn is_tashkeel(archar: char) -> bool {
    TASHKEEL.contains(&archar)
}


fn replace_character(input: &str, search_char: char, replace_char: char) -> String {
    let mut output = String::new();
    for c in input.chars() {
        if c == search_char {
            output.push(replace_char);
        } else {
            output.push(c);
        }
    }
    output
}

pub fn remove_diacritics(input: &str) -> String {
    let input = replace_character(input, 'ٱ', 'ا');
    let arabic_chars: Vec<char> = vec![
        'َ', 'ً', 'ُ', 'ٌ', 'ِ', 'ٍ', 'ْ', 'ّ', 'ٰ', 'ٓ', 'ٔ', 'ٕ', 'ٰ', 'ٖ', 'ٗ', '٘', 'ٙ', 'ٚ', 'ٛ', 'ٰ', 'ٔ', 'ٕ', 'ۢ',
        'ۚ', 'ۖ', 'ۗ', 'ۘ', 'ۙ', 'ۖ', 'ۗ', 'ۚ', 'ۛ', 'ۜ', '۟', '۠', 'ۡ', 'ۢ', 'ۣ', 'ۤ', 'ۥ', 'ۦ', 'ۧ', 'ۨ', 'ۨ', '۪',
        '۫', '۬', 'ۭ', 'ۮ', 'ۯ', '۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹', '٠', '١', '٢', '٣',
        '٤', '٥', '٦', '٧', '٨', '٩', '؟', '،', '؛',
    ];

    let mut output = String::new();
    for c in input.chars() {
        if !arabic_chars.contains(&c) {
            output.push(c);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_tashkeel() {
        // Test case with mixed text and tashkeel marks
        let text = "الْعَرَبِيّةُ";
        let stripped = strip_tashkeel(text);
        assert_eq!(stripped, "العربية");

        // Test case with only tashkeel marks
        let text = "َُِّ";
        let stripped = strip_tashkeel(text);
        assert_eq!(stripped, "");

        // Test case with no tashkeel marks
        let text = "Hello World!";
        let stripped = strip_tashkeel(text);
        assert_eq!(stripped, "Hello World!");

        let text = "بِسۡمِ ٱللَّهِ ٱلرَّحۡمَٰنِ ٱلرَّحِيمِ";
        let stripped = strip_tashkeel(text);
        println!("{}", stripped);
    }

    #[test]
    fn test_is_vocalized() {
        // Test case with vocalized word
        let word = "مَرْحَبًا";
        let vocalized = is_vocalized(word);
        assert_eq!(vocalized, true);

        // Test case with non-vocalized word
        let word = "سلام";
        let vocalized = is_vocalized(word);
        assert_eq!(vocalized, false);
    }

    #[test]
    fn test_is_tashkeel() {
        // Test case with tashkeel mark
        let tashkeel = 'َ';
        assert_eq!(is_tashkeel(tashkeel), true);

        // Test case with non-tashkeel character
        let non_tashkeel = 'م';
        assert_eq!(is_tashkeel(non_tashkeel), false);
    }
}
