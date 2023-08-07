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
// Generated by `flutter_rust_bridge`@ 1.80.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_verify_impl(
    port_: MessagePort,
    message: impl Wire2Api<String> + UnwindSafe,
    signature: impl Wire2Api<String> + UnwindSafe,
    public_key: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, bool>(
        WrapInfo {
            debug_name: "verify",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_message = message.wire2api();
            let api_signature = signature.wire2api();
            let api_public_key = public_key.wire2api();
            move |task_callback| Ok(verify(api_message, api_signature, api_public_key))
        },
    )
}
fn wire_recover_node_id_impl(
    port_: MessagePort,
    message: impl Wire2Api<String> + UnwindSafe,
    signature: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "recover_node_id",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_message = message.wire2api();
            let api_signature = signature.wire2api();
            move |task_callback| Ok(recover_node_id(api_message, api_signature))
        },
    )
}
fn wire_from_seed__static_method__Signer_impl(
    port_: MessagePort,
    seed: impl Wire2Api<[u8; 64]> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Signer>(
        WrapInfo {
            debug_name: "from_seed__static_method__Signer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_seed = seed.wire2api();
            move |task_callback| Ok(Signer::from_seed(api_seed))
        },
    )
}
fn wire_from_ldk_seed__static_method__Signer_impl(
    port_: MessagePort,
    seed: impl Wire2Api<[u8; 32]> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Signer>(
        WrapInfo {
            debug_name: "from_ldk_seed__static_method__Signer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_seed = seed.wire2api();
            move |task_callback| Ok(Signer::from_ldk_seed(api_seed))
        },
    )
}
fn wire_sign__method__Signer_impl(
    port_: MessagePort,
    that: impl Wire2Api<Signer> + UnwindSafe,
    message: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "sign__method__Signer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_message = message.wire2api();
            move |task_callback| Ok(Signer::sign(&api_that, api_message))
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

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Signer {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.secret_key_bytes.into_into_dart().into_dart(),
            self.node_id.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Signer {}
impl rust2dart::IntoIntoDart<Signer> for Signer {
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
    pub fn wire_verify(port_: MessagePort, message: String, signature: String, public_key: String) {
        wire_verify_impl(port_, message, signature, public_key)
    }

    #[wasm_bindgen]
    pub fn wire_recover_node_id(port_: MessagePort, message: String, signature: String) {
        wire_recover_node_id_impl(port_, message, signature)
    }

    #[wasm_bindgen]
    pub fn wire_from_seed__static_method__Signer(port_: MessagePort, seed: Box<[u8]>) {
        wire_from_seed__static_method__Signer_impl(port_, seed)
    }

    #[wasm_bindgen]
    pub fn wire_from_ldk_seed__static_method__Signer(port_: MessagePort, seed: Box<[u8]>) {
        wire_from_ldk_seed__static_method__Signer_impl(port_, seed)
    }

    #[wasm_bindgen]
    pub fn wire_sign__method__Signer(port_: MessagePort, that: JsValue, message: String) {
        wire_sign__method__Signer_impl(port_, that, message)
    }

    // Section: allocate functions

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for String {
        fn wire2api(self) -> String {
            self
        }
    }

    impl Wire2Api<Signer> for JsValue {
        fn wire2api(self) -> Signer {
            let self_ = self.dyn_into::<JsArray>().unwrap();
            assert_eq!(
                self_.length(),
                2,
                "Expected 2 elements, got {}",
                self_.length()
            );
            Signer {
                secret_key_bytes: self_.get(0).wire2api(),
                node_id: self_.get(1).wire2api(),
            }
        }
    }

    impl Wire2Api<[u8; 32]> for Box<[u8]> {
        fn wire2api(self) -> [u8; 32] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
        }
    }
    impl Wire2Api<[u8; 64]> for Box<[u8]> {
        fn wire2api(self) -> [u8; 64] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
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
    impl Wire2Api<u8> for JsValue {
        fn wire2api(self) -> u8 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<[u8; 32]> for JsValue {
        fn wire2api(self) -> [u8; 32] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
        }
    }
    impl Wire2Api<[u8; 64]> for JsValue {
        fn wire2api(self) -> [u8; 64] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
        }
    }
    impl Wire2Api<Vec<u8>> for JsValue {
        fn wire2api(self) -> Vec<u8> {
            self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
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
    pub extern "C" fn wire_verify(
        port_: i64,
        message: *mut wire_uint_8_list,
        signature: *mut wire_uint_8_list,
        public_key: *mut wire_uint_8_list,
    ) {
        wire_verify_impl(port_, message, signature, public_key)
    }

    #[no_mangle]
    pub extern "C" fn wire_recover_node_id(
        port_: i64,
        message: *mut wire_uint_8_list,
        signature: *mut wire_uint_8_list,
    ) {
        wire_recover_node_id_impl(port_, message, signature)
    }

    #[no_mangle]
    pub extern "C" fn wire_from_seed__static_method__Signer(
        port_: i64,
        seed: *mut wire_uint_8_list,
    ) {
        wire_from_seed__static_method__Signer_impl(port_, seed)
    }

    #[no_mangle]
    pub extern "C" fn wire_from_ldk_seed__static_method__Signer(
        port_: i64,
        seed: *mut wire_uint_8_list,
    ) {
        wire_from_ldk_seed__static_method__Signer_impl(port_, seed)
    }

    #[no_mangle]
    pub extern "C" fn wire_sign__method__Signer(
        port_: i64,
        that: *mut wire_Signer,
        message: *mut wire_uint_8_list,
    ) {
        wire_sign__method__Signer_impl(port_, that, message)
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_signer_0() -> *mut wire_Signer {
        support::new_leak_box_ptr(wire_Signer::new_with_null_ptr())
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
    impl Wire2Api<Signer> for *mut wire_Signer {
        fn wire2api(self) -> Signer {
            let wrap = unsafe { support::box_from_leak_ptr(self) };
            Wire2Api::<Signer>::wire2api(*wrap).into()
        }
    }
    impl Wire2Api<Signer> for wire_Signer {
        fn wire2api(self) -> Signer {
            Signer {
                secret_key_bytes: self.secret_key_bytes.wire2api(),
                node_id: self.node_id.wire2api(),
            }
        }
    }

    impl Wire2Api<[u8; 32]> for *mut wire_uint_8_list {
        fn wire2api(self) -> [u8; 32] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
        }
    }
    impl Wire2Api<[u8; 64]> for *mut wire_uint_8_list {
        fn wire2api(self) -> [u8; 64] {
            let vec: Vec<u8> = self.wire2api();
            support::from_vec_to_array(vec)
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
    pub struct wire_Signer {
        secret_key_bytes: *mut wire_uint_8_list,
        node_id: *mut wire_uint_8_list,
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

    impl NewWithNullPtr for wire_Signer {
        fn new_with_null_ptr() -> Self {
            Self {
                secret_key_bytes: core::ptr::null_mut(),
                node_id: core::ptr::null_mut(),
            }
        }
    }

    impl Default for wire_Signer {
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
