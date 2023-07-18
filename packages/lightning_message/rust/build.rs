use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        braces: cbindgen::Braces::SameLine,
        cpp_compat: true,
        style: cbindgen::Style::Both,
        ..cbindgen::Config::default()
    };

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("librust_lightning_message_ffi.h");
}
