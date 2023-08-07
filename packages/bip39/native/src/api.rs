/// Language to be used for the mnemonic phrase.
#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    /// The English language.
    #[default]
    English,
    /// The Spanish language.
    Spanish,
}

impl From<Language> for bip39::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::English => bip39::Language::English,
            Language::Spanish => bip39::Language::Spanish,
        }
    }
}

impl From<bip39::Language> for Language {
    fn from(value: bip39::Language) -> Self {
        match value {
            bip39::Language::English => Language::English,
            bip39::Language::Spanish => Language::Spanish,
        }
    }
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![Language::English, Language::Spanish]
    }

    pub fn word_list(&self) -> Vec<String> {
        let language: bip39::Language = (*self).into();
        let words: &[&'static str; 2048] = language.word_list();
        words.iter().map(|&s| s.to_string()).collect()
    }

    pub fn words_by_prefix(&self, prefix: String) -> Vec<String> {
        let language: bip39::Language = (*self).into();
        language
            .words_by_prefix(prefix.as_str())
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn find_word(&self, word: String) -> i32 {
        let language: bip39::Language = (*self).into();
        language
            .find_word(word.as_str())
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}

/// Type describing entropy length (aka word count) in the mnemonic
pub enum WordCount {
    /// 12 words mnemonic (128 bits entropy)
    Words12 = 12,
    /// 15 words mnemonic (160 bits entropy)
    Words15 = 15,
    /// 18 words mnemonic (192 bits entropy)
    Words18 = 18,
    /// 21 words mnemonic (224 bits entropy)
    Words21 = 21,
    /// 24 words mnemonic (256 bits entropy)
    Words24 = 24,
}

pub struct Mnemonic {
    pub language: Language,
    pub words: Vec<String>,
}

impl From<&Mnemonic> for bip39::Mnemonic {
    fn from(value: &Mnemonic) -> Self {
        bip39::Mnemonic::parse_in(value.language.into(), value.words.join(" ").as_str()).unwrap()
    }
}

impl From<bip39::Mnemonic> for Mnemonic {
    fn from(value: bip39::Mnemonic) -> Self {
        Self {
            language: value.language().into(),
            words: value.word_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Mnemonic {
    pub fn generate_in(language: Language, word_count: WordCount) -> Mnemonic {
        let mnemonic = bip39::Mnemonic::generate_in(language.into(), word_count as usize).unwrap();
        Self {
            language,
            words: mnemonic.word_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn parse(words: Vec<String>) -> Mnemonic {
        bip39::Mnemonic::parse(words.join(" ").as_str())
            .unwrap()
            .into()
    }

    pub fn parse_in(language: Language, words: Vec<String>) -> Mnemonic {
        bip39::Mnemonic::parse_in(language.into(), words.join(" ").as_str())
            .unwrap()
            .into()
    }

    pub fn to_seed(&self, passphrase: String) -> [u8; 64] {
        let bip39_mnemonic: bip39::Mnemonic = self.into();
        bip39_mnemonic.to_seed(passphrase.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default_language = Language::default();
        assert_eq!(default_language, Language::English);
    }

    #[test]
    fn test_conversion_to_bip39() {
        let lang = Language::Spanish;
        let bip39_lang: bip39::Language = lang.into();
        assert_eq!(bip39_lang, bip39::Language::Spanish);
    }

    #[test]
    fn test_conversion_from_bip39() {
        let bip39_lang = bip39::Language::English;
        let lang: Language = bip39_lang.into();
        assert_eq!(lang, Language::English);
    }

    #[test]
    fn test_all_languages() {
        let languages = Language::all();
        assert_eq!(languages, vec![Language::English, Language::Spanish]);
    }

    #[test]
    fn test_word_list() {
        let lang = Language::English;
        let bip39_lang: bip39::Language = lang.into();
        assert_eq!(
            lang.word_list(),
            bip39_lang
                .word_list()
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_words_by_prefix() {
        let prefix = "ab";
        let lang = Language::English;
        assert_eq!(
            lang.words_by_prefix(prefix.to_string()),
            vec![
                "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
                "absurd", "abuse"
            ]
        );
    }

    #[test]
    fn test_find_word() {
        let word = "about".to_string();
        let lang = Language::English;
        assert_eq!(lang.find_word(word), 3);
    }

    #[test]
    fn test_find_nonexistent_word() {
        let word = "nonexistent".to_string();
        let lang = Language::English;
        assert_eq!(lang.find_word(word), -1);
    }

    #[test]
    fn test_generate_in() {
        let language = Language::English;
        let word_count = WordCount::Words12;
        let mnemonic = Mnemonic::generate_in(language, word_count);
        assert_eq!(mnemonic.language, language);
        assert_eq!(mnemonic.words.len(), WordCount::Words12 as usize);
    }

    #[test]
    fn test_parse() {
        let words = vec![
            "goat", "magnet", "speed", "sweet", "release", "pill", "tiny", "decline", "talent",
            "extra", "sunny", "diamond",
        ];
        let mnemonic = Mnemonic::parse(words.clone().into_iter().map(|s| s.to_string()).collect());
        assert_eq!(mnemonic.words, words);
    }

    #[test]
    fn test_parse_in() {
        let language = Language::Spanish;
        let words = vec![
            "fiel",
            "acoger",
            "acoger",
            "pereza",
            "torero",
            "ábaco",
            "gimnasio",
            "certeza",
            "piso",
            "vampiro",
            "culpa",
            "pista",
            "bozal",
            "acoger",
            "topar",
            "triste",
            "óptica",
            "forro",
            "diez",
            "firma",
            "lástima",
            "apodo",
            "víspera",
            "filial",
        ];
        let mnemonic = Mnemonic::parse_in(
            language,
            words.clone().into_iter().map(|s| s.to_string()).collect(),
        );
        assert_eq!(mnemonic.language, language);
        assert_eq!(mnemonic.words, words);
    }

    #[test]
    fn test_conversion_to_bip39_mnemonic() {
        let words = vec![
            "goat", "magnet", "speed", "sweet", "release", "pill", "tiny", "decline", "talent",
            "extra", "sunny", "diamond",
        ];
        let mnemonic = Mnemonic {
            language: Language::English,
            words: words.clone().into_iter().map(|s| s.to_string()).collect(),
        };
        let bip39_mnemonic: bip39::Mnemonic = (&mnemonic).into();
        assert_eq!(bip39_mnemonic.language(), bip39::Language::English);
        assert_eq!(bip39_mnemonic.word_iter().collect::<Vec<&str>>(), words);
    }

    #[test]
    fn test_conversion_from_bip39_mnemonic() {
        let bip39_mnemonic = bip39::Mnemonic::parse(
            "goat magnet speed sweet release pill tiny decline talent extra sunny diamond",
        )
        .unwrap();
        let mnemonic: Mnemonic = bip39_mnemonic.into();
        assert_eq!(mnemonic.language, Language::English);
        assert_eq!(
            mnemonic.words,
            vec![
                "goat", "magnet", "speed", "sweet", "release", "pill", "tiny", "decline", "talent",
                "extra", "sunny", "diamond",
            ]
        );
    }

    #[test]
    fn test_to_seed() {
        let mnemonic_words = vec![
            "fiel",
            "acoger",
            "acoger",
            "pereza",
            "torero",
            "ábaco",
            "gimnasio",
            "certeza",
            "piso",
            "vampiro",
            "culpa",
            "pista",
            "bozal",
            "acoger",
            "topar",
            "triste",
            "óptica",
            "forro",
            "diez",
            "firma",
            "lástima",
            "apodo",
            "víspera",
            "filial",
        ];
        let mnemonic = Mnemonic::parse_in(
            Language::Spanish,
            mnemonic_words
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let passphrase = "";

        // Replace this with the expected seed value
        let expected_seed_hex = "cffcf0a2c8767a71d318d0eefeda0e9066d149759af70b9c76131e3bad5ddea14026bdd289c8913e4a1a9e2e0f240e9bf1398a259ac4b306210cb45fc3bc1a8d";

        let seed = mnemonic.to_seed(passphrase.to_string());

        assert_eq!(hex::encode(seed), expected_seed_hex);
    }
}
