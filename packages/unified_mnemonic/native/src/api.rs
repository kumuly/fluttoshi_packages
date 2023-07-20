use bdk::bitcoin;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::ChildNumber;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::keys::bip39;

/// Language to be used for the mnemonic phrase.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    /// The English language.
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<Network> for bitcoin::Network {
    fn from(value: Network) -> Self {
        match value {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Signet => bitcoin::Network::Signet,
            Network::Regtest => bitcoin::Network::Regtest,
        }
    }
}

pub struct Mnemonic {
    pub phrase: String,
    /// The language the mnemonic.
    pub language: Language,
}

impl Mnemonic {
    // Create a new mnemonic from Language and WordCount
    pub fn new(language: Language, word_count: WordCount) -> Mnemonic {
        let m = bip39::Mnemonic::generate_in(language.into(), word_count as usize).unwrap();
        let phrase = m.to_string();
        Self { phrase, language }
    }

    pub fn from_phrase(phrase: String) -> Mnemonic {
        let lang: bip39::Language = bip39::Mnemonic::language_of(&phrase).unwrap();
        Self {
            phrase,
            language: lang.into(),
        }
    }

    pub fn derive_lightning_seed(
        &self,
        network: Network,
        hardened_child_index: Option<u32>,
    ) -> [u8; 32] {
        let mnemonic =
            bip39::Mnemonic::parse_in_normalized(self.language.into(), &self.phrase).unwrap();
        let seed = mnemonic.to_seed_normalized("");
        let master_xprv = ExtendedPrivKey::new_master(network.into(), &seed).unwrap();
        let secp = Secp256k1::new();
        let hardened_child_index = hardened_child_index.unwrap_or(535);
        let xprv: ExtendedPrivKey = master_xprv
            .ckd_priv(
                &secp,
                ChildNumber::Hardened {
                    index: hardened_child_index,
                },
            )
            .unwrap();
        xprv.private_key.secret_bytes()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generating_12_word_mnemonic_works() {
        let m = Mnemonic::new(Language::English, WordCount::Words12);
        assert_eq!(m.phrase.split_whitespace().count(), 12);
        let parsed = bip39::Mnemonic::parse_in_normalized(m.language.into(), &m.phrase);
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
    }

    #[test]
    fn generating_24_word_mnemonic_works() {
        let m = Mnemonic::new(Language::English, WordCount::Words24);
        assert_eq!(m.phrase.split_whitespace().count(), 24);
        let parsed = bip39::Mnemonic::parse_in_normalized(m.language.into(), &m.phrase);
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
    }

    #[test]
    fn generating_12_word_mnemonic_in_spanish_works() {
        let m = Mnemonic::new(Language::Spanish, WordCount::Words12);
        assert_eq!(m.phrase.split_whitespace().count(), 12);
        let parsed = bip39::Mnemonic::parse_in_normalized(m.language.into(), &m.phrase);
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
    }

    #[test]
    fn generating_24_word_mnemonic_in_spanish_works() {
        let m = Mnemonic::new(Language::Spanish, WordCount::Words24);
        assert_eq!(m.phrase.split_whitespace().count(), 24);
        let parsed = bip39::Mnemonic::parse_in_normalized(m.language.into(), &m.phrase);
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
    }

    #[test]
    fn recover_mnemonic_from_english_phrase_works() {
        let phrase =
            "panther apart spike furnace card shine ordinary visa machine tornado shallow art";
        let m = Mnemonic::from_phrase(phrase.to_string());
        assert_eq!(m.language, Language::English);
    }

    #[test]
    fn recover_mnemonic_from_spanish_phrase_works() {
        let phrase= "fiel acoger acoger pereza torero ábaco gimnasio certeza piso vampiro culpa pista bozal acoger topar triste óptica forro diez firma lástima apodo víspera filial";
        let m = Mnemonic::from_phrase(phrase.to_string());
        assert_eq!(m.language, Language::Spanish);
    }

    #[test]
    fn derive_lightning_seed_works() {
        let phrase: &str =
            "goat magnet speed sweet release pill tiny decline talent extra sunny diamond";
        let mnemonic = Mnemonic::from_phrase(phrase.to_string());
        let seed = mnemonic.derive_lightning_seed(Network::Bitcoin, None);
        // print the seed as a string
        // println!("String: {}", hex::encode(seed));
        let xpriv_from_seed = ExtendedPrivKey::new_master(Network::Bitcoin.into(), &seed).unwrap();
        assert_eq!(xpriv_from_seed.to_string(), "xprv9s21ZrQH143K3YUjhWwtzDugFyCFYsmo4F4Ve8TDkdKiwrDHekHZoWZWc8XFGvC1iAiCf82fJBNVSGnkhyhWwAYSyuQoc9R4U6HSKS3YWgJ");
    }
}
