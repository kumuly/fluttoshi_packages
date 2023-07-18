use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use bdk::bitcoin;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::ChildNumber;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::keys::bip39;

/// Language to be used for the mnemonic phrase.
#[repr(C)]
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
#[repr(C)]
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

#[repr(C)]
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

#[repr(C)]
pub struct Mnemonic {
    pub phrase: *mut c_char,
    /// The language the mnemonic.
    pub language: Language,
}

impl Mnemonic {
    /// Creates a new mnemonic phrase.
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it returns a raw pointer to a heap-allocated
    /// structure, and the caller (in C or another language) is responsible for deallocating
    /// this structure through the appropriate function (`free_mnemonic` for example).
    ///
    /// It is undefined behavior if the caller does not ensure that the `Mnemonic` is properly
    /// deallocated.
    ///
    /// # Parameters
    ///
    /// - `language`: The language in which the mnemonic phrase should be generated.
    /// - `word_count`: The number of words in the mnemonic phrase.
    ///
    /// # Returns
    ///
    /// A pointer to the newly created `Mnemonic`. The caller must ensure that the `Mnemonic`
    /// is properly deallocated to avoid memory leaks.
    #[no_mangle]
    pub unsafe extern "C" fn new(language: Language, word_count: WordCount) -> *mut Mnemonic {
        let m = bip39::Mnemonic::generate_in(language.into(), word_count as usize).unwrap();
        let phrase = CString::new(m.to_string()).unwrap();
        let phrase_ptr = phrase.into_raw();

        let mnemonic = Mnemonic {
            phrase: phrase_ptr,
            language,
        };
        Box::into_raw(Box::new(mnemonic))
    }

    /// Creates a new `Mnemonic` structure from an existing mnemonic phrase.
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because it takes a raw pointer to a C string as
    /// an argument and returns a raw pointer to a heap-allocated structure.
    ///
    /// The caller must ensure that:
    /// - The `phrase` pointer points to a valid null-terminated string.
    /// - The string data is encoded in UTF-8.
    /// - The `Mnemonic` returned is properly deallocated through the appropriate function
    ///   (`free_mnemonic` for example).
    ///
    /// It is undefined behavior if the caller does not ensure that the `Mnemonic` is properly
    /// deallocated or if the `phrase` pointer is not valid.
    ///
    /// # Parameters
    ///
    /// - `phrase`: A pointer to a null-terminated string that contains the mnemonic phrase.
    ///
    /// # Returns
    ///
    /// A pointer to the newly created `Mnemonic`. The caller must ensure that the `Mnemonic`
    /// is properly deallocated to avoid memory leaks.
    #[no_mangle]
    pub unsafe extern "C" fn from_phrase(phrase: *const c_char) -> *mut Mnemonic {
        let c_str = CStr::from_ptr(phrase);
        let phrase_str = c_str.to_str().unwrap();

        let lang = bip39::Mnemonic::language_of(phrase_str).unwrap();

        let mnemonic = Mnemonic {
            phrase: phrase as *mut c_char,
            language: lang.into(),
        };
        Box::into_raw(Box::new(mnemonic))
    }

    /// # Safety
    ///
    /// This function is unsafe because it dereferences raw pointers.
    /// Callers must ensure that the Mnemonic struct is properly initialized
    /// and that the memory it points to remains allocated for the duration of the call.
    ///
    /// Additionally, the caller is responsible for managing the memory of the returned array,
    /// which is allocated on the heap.
    #[no_mangle]
    pub unsafe extern "C" fn derive_lightning_seed(
        &self,
        network: Network,
        hardened_child_index: *const u32,
    ) -> *mut c_char {
        let c_str = CStr::from_ptr(self.phrase);
        let phrase_str = c_str.to_str().unwrap();

        let mnemonic =
            bip39::Mnemonic::parse_in_normalized(self.language.into(), phrase_str).unwrap();
        let seed = mnemonic.to_seed_normalized("");
        let master_xprv = ExtendedPrivKey::new_master(network.into(), &seed).unwrap();
        let secp = Secp256k1::new();
        let hardened_child_index = if !hardened_child_index.is_null() {
            *hardened_child_index
        } else {
            535 // or some other default value
        };
        let xprv: ExtendedPrivKey = master_xprv
            .ckd_priv(
                &secp,
                ChildNumber::Hardened {
                    index: hardened_child_index,
                },
            )
            .unwrap();

        CString::new(hex::encode(xprv.private_key.secret_bytes()))
            .unwrap()
            .into_raw()
    }
}

/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
/// Callers must ensure that the pointer is valid.
///
/// This function should only be used to free memory that was allocated by the Rust code.
#[no_mangle]
pub unsafe extern "C" fn free_mnemonic(mnemonic: *mut Mnemonic) {
    if !mnemonic.is_null() {
        let m = Box::from_raw(mnemonic);
        // When you free the Mnemonic using free_mnemonic, you also need to free the CString allocated in the new method.
        let phrase = CString::from_raw(m.phrase);
        drop(phrase);
        drop(m);
    }
}

/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
/// Callers must ensure that the pointer is valid.
///
/// This function should only be used to free memory that was allocated by the Rust code.
#[no_mangle]
pub unsafe extern "C" fn free_lightning_seed(seed: *mut [u8; 32]) {
    if !seed.is_null() {
        drop(Box::from_raw(seed));
    }
}

#[cfg(test)]
mod tests {

    use std::ptr::null;

    use bdk::bitcoin::hashes::hex::FromHex;

    use super::*;

    #[test]
    fn generating_12_word_mnemonic_works() {
        let m = unsafe { Mnemonic::new(Language::English, WordCount::Words12) };
        let phrase = unsafe { CStr::from_ptr((*m).phrase) };
        assert_eq!(phrase.to_str().unwrap().split_whitespace().count(), 12);
        let parsed = bip39::Mnemonic::parse_in_normalized(
            Language::English.into(),
            phrase.to_str().unwrap(),
        );
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
        unsafe { free_mnemonic(m) };
    }

    #[test]
    fn generating_24_word_mnemonic_works() {
        let m = unsafe { Mnemonic::new(Language::English, WordCount::Words24) };
        let phrase = unsafe { CStr::from_ptr((*m).phrase) };
        assert_eq!(phrase.to_str().unwrap().split_whitespace().count(), 24);
        let parsed = bip39::Mnemonic::parse_in_normalized(
            Language::English.into(),
            phrase.to_str().unwrap(),
        );
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
        unsafe { free_mnemonic(m) };
    }

    #[test]
    fn generating_12_word_mnemonic_in_spanish_works() {
        let m = unsafe { Mnemonic::new(Language::Spanish, WordCount::Words12) };
        let phrase = unsafe { CStr::from_ptr((*m).phrase) };
        assert_eq!(phrase.to_str().unwrap().split_whitespace().count(), 12);
        let parsed = bip39::Mnemonic::parse_in_normalized(
            Language::Spanish.into(),
            phrase.to_str().unwrap(),
        );
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
        unsafe { free_mnemonic(m) };
    }

    #[test]
    fn generating_24_word_mnemonic_in_spanish_works() {
        let m = unsafe { Mnemonic::new(Language::Spanish, WordCount::Words24) };
        let phrase = unsafe { CStr::from_ptr((*m).phrase) };
        assert_eq!(phrase.to_str().unwrap().split_whitespace().count(), 24);
        let parsed = bip39::Mnemonic::parse_in_normalized(
            Language::Spanish.into(),
            phrase.to_str().unwrap(),
        );
        assert!(
            parsed.is_ok(),
            "Failed to parse mnemonic: {}",
            parsed.err().unwrap()
        );
        unsafe { free_mnemonic(m) };
    }

    #[test]
    fn recover_mnemonic_from_english_phrase_works() {
        let phrase =
            "panther apart spike furnace card shine ordinary visa machine tornado shallow art";
        let phrase_cstr = CString::new(phrase).unwrap();
        let m = unsafe { Mnemonic::from_phrase(phrase_cstr.as_ptr()) };
        assert_eq!(unsafe { (*m).language }, Language::English);
    }

    #[test]
    fn recover_mnemonic_from_spanish_phrase_works() {
        let phrase = "fiel acoger acoger pereza torero ábaco gimnasio certeza piso vampiro culpa pista bozal acoger topar triste óptica forro diez firma lástima apodo víspera filial";
        let phrase_cstr = CString::new(phrase).unwrap();
        let m = unsafe { Mnemonic::from_phrase(phrase_cstr.as_ptr()) };
        assert_eq!(unsafe { (*m).language }, Language::Spanish);
    }

    #[test]
    fn derive_lightning_seed_works() {
        let phrase: &str =
            "goat magnet speed sweet release pill tiny decline talent extra sunny diamond";
        let phrase_cstr = CString::new(phrase).unwrap();
        let mnemonic = unsafe { Mnemonic::from_phrase(phrase_cstr.as_ptr()) };
        let seed_hex = unsafe { (*mnemonic).derive_lightning_seed(Network::Bitcoin, null()) };
        // print the seed as a string
        println!("Seed Hex: {}", unsafe {
            CStr::from_ptr(seed_hex).to_str().unwrap()
        });

        let seed_bytes = Vec::from_hex(unsafe { CStr::from_ptr(seed_hex).to_str().unwrap() })
            .expect("Invalid hexadecimal string");
        let mut result = [0u8; 32];
        result.copy_from_slice(&seed_bytes[..32]);
        println!("Seed Bytes: {:?}", result);
        let xpriv_from_seed =
            ExtendedPrivKey::new_master(Network::Bitcoin.into(), &seed_bytes).unwrap();
        assert_eq!(xpriv_from_seed.to_string(), "xprv9s21ZrQH143K3YUjhWwtzDugFyCFYsmo4F4Ve8TDkdKiwrDHekHZoWZWc8XFGvC1iAiCf82fJBNVSGnkhyhWwAYSyuQoc9R4U6HSKS3YWgJ");
    }
}
