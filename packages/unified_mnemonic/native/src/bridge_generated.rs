#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.79.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_new__static_method__Mnemonic_impl(
    port_: MessagePort,
    language: impl Wire2Api<Language> + UnwindSafe,
    word_count: impl Wire2Api<WordCount> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Mnemonic>(
        WrapInfo {
            debug_name: "new__static_method__Mnemonic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_language = language.wire2api();
            let api_word_count = word_count.wire2api();
            move |task_callback| Ok(Mnemonic::new(api_language, api_word_count))
        },
    )
}
fn wire_from_phrase__static_method__Mnemonic_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Mnemonic>(
        WrapInfo {
            debug_name: "from_phrase__static_method__Mnemonic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            move |task_callback| Ok(Mnemonic::from_phrase(api_phrase))
        },
    )
}
fn wire_derive_seed__method__Mnemonic_impl(
    port_: MessagePort,
    that: impl Wire2Api<Mnemonic> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, [u8; 64]>(
        WrapInfo {
            debug_name: "derive_seed__method__Mnemonic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Mnemonic::derive_seed(&api_that))
        },
    )
}
fn wire_derive_lightning_seed__method__Mnemonic_impl(
    port_: MessagePort,
    that: impl Wire2Api<Mnemonic> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    hardened_child_index: impl Wire2Api<Option<u32>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, [u8; 32]>(
        WrapInfo {
            debug_name: "derive_lightning_seed__method__Mnemonic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_network = network.wire2api();
            let api_hardened_child_index = hardened_child_index.wire2api();
            move |task_callback| {
                Ok(Mnemonic::derive_lightning_seed(
                    &api_that,
                    api_network,
                    api_hardened_child_index,
                ))
            }
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<Language> for i32 {
    fn wire2api(self) -> Language {
        match self {
            0 => Language::English,
            1 => Language::Spanish,
            _ => unreachable!("Invalid variant for Language: {}", self),
        }
    }
}

impl Wire2Api<Network> for i32 {
    fn wire2api(self) -> Network {
        match self {
            0 => Network::Bitcoin,
            1 => Network::Testnet,
            2 => Network::Signet,
            3 => Network::Regtest,
            _ => unreachable!("Invalid variant for Network: {}", self),
        }
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<WordCount> for i32 {
    fn wire2api(self) -> WordCount {
        match self {
            0 => WordCount::Words12,
            1 => WordCount::Words15,
            2 => WordCount::Words18,
            3 => WordCount::Words21,
            4 => WordCount::Words24,
            _ => unreachable!("Invalid variant for WordCount: {}", self),
        }
    }
}
// Section: impl IntoDart

impl support::IntoDart for Language {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::English => 0,
            Self::Spanish => 1,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Language {}
impl rust2dart::IntoIntoDart<Language> for Language {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Mnemonic {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.phrase.into_into_dart().into_dart(),
            self.language.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Mnemonic {}
impl rust2dart::IntoIntoDart<Mnemonic> for Mnemonic {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web {
    use super::*;
    // Section: wire functions

    #[wasm_bindgen]
    pub fn wire_new__static_method__Mnemonic(port_: MessagePort, language: i32, word_count: i32) {
        wire_new__static_method__Mnemonic_impl(port_, language, word_count)
    }

    #[wasm_bindgen]
    pub fn wire_from_phrase__static_method__Mnemonic(port_: MessagePort, phrase: String) {
        wire_from_phrase__static_method__Mnemonic_impl(port_, phrase)
    }

    #[wasm_bindgen]
    pub fn wire_derive_seed__method__Mnemonic(port_: MessagePort, that: JsValue) {
        wire_derive_seed__method__Mnemonic_impl(port_, that)
    }

    #[wasm_bindgen]
    pub fn wire_derive_lightning_seed__method__Mnemonic(
        port_: MessagePort,
        that: JsValue,
        network: i32,
        hardened_child_index: JsValue,
    ) {
        wire_derive_lightning_seed__method__Mnemonic_impl(
            port_,
            that,
            network,
            hardened_child_index,
        )
    }

    // Section: allocate functions

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for String {
        fn wire2api(self) -> String {
            self
        }
    }

    impl Wire2Api<Mnemonic> for JsValue {
        fn wire2api(self) -> Mnemonic {
            let self_ = self.dyn_into::<JsArray>().unwrap();
            assert_eq!(
                self_.length(),
                2,
                "Expected 2 elements, got {}",
                self_.length()
            );
            Mnemonic {
                phrase: self_.get(0).wire2api(),
                language: self_.get(1).wire2api(),
            }
        }
    }

    impl Wire2Api<Vec<u8>> for Box<[u8]> {
        fn wire2api(self) -> Vec<u8> {
            self.into_vec()
        }
    }

    // Section: impl Wire2Api for JsValue

    impl Wire2Api<String> for JsValue {
        fn wire2api(self) -> String {
            self.as_string().expect("non-UTF-8 string, or not a string")
        }
    }
    impl Wire2Api<i32> for JsValue {
        fn wire2api(self) -> i32 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<Language> for JsValue {
        fn wire2api(self) -> Language {
            (self.unchecked_into_f64() as i32).wire2api()
        }
    }
    impl Wire2Api<Network> for JsValue {
        fn wire2api(self) -> Network {
            (self.unchecked_into_f64() as i32).wire2api()
        }
    }
    impl Wire2Api<Option<u32>> for JsValue {
        fn wire2api(self) -> Option<u32> {
            (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
        }
    }
    impl Wire2Api<u32> for JsValue {
        fn wire2api(self) -> u32 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<u8> for JsValue {
        fn wire2api(self) -> u8 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<Vec<u8>> for JsValue {
        fn wire2api(self) -> Vec<u8> {
            self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
        }
    }
    impl Wire2Api<WordCount> for JsValue {
        fn wire2api(self) -> WordCount {
            (self.unchecked_into_f64() as i32).wire2api()
        }
    }
}
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_new__static_method__Mnemonic(
        port_: i64,
        language: i32,
        word_count: i32,
    ) {
        wire_new__static_method__Mnemonic_impl(port_, language, word_count)
    }

    #[no_mangle]
    pub extern "C" fn wire_from_phrase__static_method__Mnemonic(
        port_: i64,
        phrase: *mut wire_uint_8_list,
    ) {
        wire_from_phrase__static_method__Mnemonic_impl(port_, phrase)
    }

    #[no_mangle]
    pub extern "C" fn wire_derive_seed__method__Mnemonic(port_: i64, that: *mut wire_Mnemonic) {
        wire_derive_seed__method__Mnemonic_impl(port_, that)
    }

    #[no_mangle]
    pub extern "C" fn wire_derive_lightning_seed__method__Mnemonic(
        port_: i64,
        that: *mut wire_Mnemonic,
        network: i32,
        hardened_child_index: *mut u32,
    ) {
        wire_derive_lightning_seed__method__Mnemonic_impl(
            port_,
            that,
            network,
            hardened_child_index,
        )
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_mnemonic_0() -> *mut wire_Mnemonic {
        support::new_leak_box_ptr(wire_Mnemonic::new_with_null_ptr())
    }

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
        support::new_leak_box_ptr(value)
    }

    #[no_mangle]
    pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
        let ans = wire_uint_8_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for *mut wire_uint_8_list {
        fn wire2api(self) -> String {
            let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()
        }
    }
    impl Wire2Api<Mnemonic> for *mut wire_Mnemonic {
        fn wire2api(self) -> Mnemonic {
            let wrap = unsafe { support::box_from_leak_ptr(self) };
            Wire2Api::<Mnemonic>::wire2api(*wrap).into()
        }
    }
    impl Wire2Api<u32> for *mut u32 {
        fn wire2api(self) -> u32 {
            unsafe { *support::box_from_leak_ptr(self) }
        }
    }

    impl Wire2Api<Mnemonic> for wire_Mnemonic {
        fn wire2api(self) -> Mnemonic {
            Mnemonic {
                phrase: self.phrase.wire2api(),
                language: self.language.wire2api(),
            }
        }
    }

    impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
        fn wire2api(self) -> Vec<u8> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }

    // Section: wire structs

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_Mnemonic {
        phrase: *mut wire_uint_8_list,
        language: i32,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_8_list {
        ptr: *mut u8,
        len: i32,
    }

    // Section: impl NewWithNullPtr

    pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }

    impl NewWithNullPtr for wire_Mnemonic {
        fn new_with_null_ptr() -> Self {
            Self {
                phrase: core::ptr::null_mut(),
                language: Default::default(),
            }
        }
    }

    impl Default for wire_Mnemonic {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    // Section: sync execution mode utility

    #[no_mangle]
    pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
        unsafe {
            let _ = support::box_from_leak_ptr(ptr);
        };
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
