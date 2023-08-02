[![Build & Test](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml/badge.svg)](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml)

# Fluttoshi packages

We think that all apps that need any kind of global payment mechanism are the best of with Bitcoin on the Lightning Network. We envision a near future where all apps will have a lightweight, non-custodial Lightning wallet embedded, even if they are not necesarily 'Bitcoin' or payment apps at their core.

We want to make it as easy as possible for this to happen. Therefore the packages in this project are designed to be as modular as possible, so that they can be used as building blocks in any Flutter app, regardless of the app's purpose. Also apps that do want to make more full-fledged Bitcoin and Lightning apps might find these packages useful. A monorepo is used instead of a repo per package to guarantee all packages and their dependency versions are compatible with each other on the same versions.

We also believe that Flutter is the best framework to build cross-platform apps and that Rust is the best language to build the core functionality of these apps. Therefore we are building these packages in Rust and Flutter.

We mainly use the [Lightning Development Kit](https://www.lightningdevkit.org) and [Bitcoin Development Kit](https://www.bitcoindevkit.org) to have trusted, well-tested and well-maintained Rust libraries to build on.

## Packages

- [x] `unified_mnemonic`: package to generate and/or restore mnemonics and derive other keys and seeds from them. This makes it possible to have a unified wallet, which means having just one mnemonic for different wallets. For example, have a Bitcoin on-chain wallet, a Bitcoin on-chain wallet with passphrase and a Lightning node wallet, all derived from one 'unified' mnemonic.
- [ ] `lightning_message`: package to sign and verify Lightning messages. Useful for proving ownership of a node, which in turn is useful for login mechanisms to get rid of username and passwords in apps, amongst other things.
- [ ] `ldk`: package to set up a Lightning node, based on LDK, in a Flutter app.
- [ ] `ldk_bloc`: package that uses the ldk package and provides a Bloc to use in Flutter apps.
- [ ] `lsp_client`: package to connect to a Lightning Service Provider (LSP) and use it to get liquidity, open channels, receive payments offline, etc.
- [ ] `payjoin`: package to integrate payjoin and its different applications, like payjoin for on-chain transactions and payjoin for opening Lightning channels.

## Mono-repo

The idea of managing a monorepo with multiple packages is to have a single place to manage all the packages and their dependencies to make sure that all packages on the same version are compatible with each other.
This means that we need to be careful when adding a new package to the monorepo and make sure they all use the same versions of the dependencies so they can be used together in Flutter apps.

### Adding a new dependency

When adding a new dependency to a package, make sure to check if the dependency is already used in another package and if so, use the same version.

### Adding a new package

#### Both Dart-only & Flutter packages

Of every package we manage a Dart package and a Flutter package. The Dart package contains the Rust code (in case the package uses Rust or a Rust library) and the Flutter package is a wrapper around the Dart package to make it usable in Flutter apps. This is done so the Dart packages can be used in non-Flutter apps as well, like a CLI or server app.

The naming of the Dart package is `<package_name>`. The naming of the Flutter package is `flutter_<package_name>`. Each has its own folder in the `packages` folder. When adding a new package, make sure to add both the Dart and Flutter package.

Always start with the Dart package and test that it is working, then add the Flutter package that uses and wraps the Dart package.

#### Starting with a Dart package

To create a dart package in the `packages` folder, just run the following command from the root of the repository:

```bash
dart create --template=package packages/<package_name>
```

Adapt the version of the package to the same version of the other packages, add a clear description, homepage and repository and check that the Dart SDK version is up-to-date in the `pubspec.yaml` file that got created.

Remove the `analysis_options.yaml` file from the package, as we use the one in the root of the repository: `rm packages/<package_name>/analysis_options.yaml`.

##### Native code

If the package uses Rust, add the dependencies to create bindings. Currently we use `flutter_rust_bridge` to facilitate this. These instructions are also based on their documentation on [creating a Dart/Flutter library with Rust bindings](https://cjycode.com/flutter_rust_bridge/library.html).
In the future, to have less dependencies, it is possible we will eliminate `flutter_rust_bridge` and create bindings more manually or in another way. But for now, just go on and add `flutter_rust_bridge`, `ffi` and `ffigen` to your (dev-)dependencies in the package:

```bash
cd packages/<package_name>
dart pub add flutter_rust_bridge ffi && dart pub add ffigen --dev
```

Make sure again the versions of the package and the dependencies are the same as the other packages in the monorepo.

Now create a file `ffi.dart` in `lib/src` and a folder named `ffi` with the following files: `io.dart`, `web.dart` and `stub.dart`:

```bash
touch lib/src/ffi.dart
mkdir -p lib/src/ffi && touch lib/src/ffi/io.dart lib/src/ffi/web.dart lib/src/ffi/stub.dart
```

These files handle communication between Dart and native code. Depending on whether the application runs on a desktop/mobile (io.dart), in a web environment (web.dart), or lacks a specific implementation (stub.dart), a corresponding file will be selected. The ffi.dart file leverages conditional imports to use the correct platform-specific implementation, and it provides a function to create and store an instance of the api of the bindings for native interfacing.

Copy the content of these files from other packages that use Rust and adapt them to the new package by changing the names of the return values of the wrappers and making sure the imports also have the path to the new package.

It is normal that you see errors in the files, as we have not yet generated the bindings. That will happen automatically from the moment we have the Rust code in the package and the build process set up.

Now add the following to the `<package_name>.dart` in the `lib` folder:

```dart
/// Document the library here.
library;

// Export any libraries intended for clients of this package.
export 'package:flutter_rust_bridge/flutter_rust_bridge.dart' show WasmModule;

export 'src/bridge_generated.dart';
export 'src/ffi.dart';
```

This will export everything that is needed to use the bindings to interface with the native code in the package.

If you do not need any extra code in Dart and only need to expose the native code through the bindings as they are generated, you can remove the `lib/src/<package_name>_base.dart` file.

Now we can add the Rust project and code itself. Create a folder `native` in the package and add a `Cargo.toml` file like this:

```toml
[package]
name = "<package_name>"
version = "<package_version>"
edition = "2021"

[lib]
crate-type = ["staticlib", "cdylib"]

[build-dependencies]
flutter_rust_bridge_codegen = "<frb_version>"

[dependencies]
flutter_rust_bridge = "<frb_version>"
```

This has the same name as the package and the same version. The `crate-type` is needed to make sure the package is compiled as both a static library as a C-compatible dynamic library so they can be linked correctly depending on the platform of the application.
Flutter Rust Bridge is used to generate the bindings and the codegen is used to generate the bindings. Make sure they have the same version as the other packages in the monorepo.

Copy the following content to a `build.rs` file in the `native` folder and replace the <package_name> for the correct name of the package:

```rust
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

const RUST_INPUT: &str = "src/api.rs";
const DART_OUTPUT: &str = "../lib/src/bridge_generated.dart";

const IOS_C_OUTPUT: &str = "../../flutter_<package_name>/ios/Classes/frb.h";
const MACOS_C_OUTPUT_DIR: &str = "../../flutter_<package_name>/macos/Classes/";

fn main() {
    // Tell Cargo that if the input Rust code changes, rerun this build script
    println!("cargo:rerun-if-changed={}", RUST_INPUT);

    // Options for frb_codegen
    let raw_opts = RawOpts {
        rust_input: vec![RUST_INPUT.to_string()],
        dart_output: vec![DART_OUTPUT.to_string()],
        c_output: Some(vec![IOS_C_OUTPUT.to_string()]),
        extra_c_output_path: Some(vec![MACOS_C_OUTPUT_DIR.to_string()]),
        inline_rust: true,
        wasm: true,
        ..Default::default()
    };

    // Generate Rust & Dart ffi bridges
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    // Format the generated Dart code
    _ = std::process::Command::new("flutter")
        .arg("format")
        .arg("..")
        .spawn();
}
```

Also add a `.gitignore` file to the `native` folder with the following content:

```gitignore
# Rust library related
Cargo.lock
target
```

Now we can add the Rust code itself. Create a folder `src` in the `native` folder and add a `api.rs` file. Here you can implement the Rust code that you want to expose to Dart.

Expose the api as a module by adding the following `lib.rs` file to the `native/src` folder:

```rust
mod api;
```

Now add the new native package as a member in the workspace of the project, you can do this in the `Cargo.toml` file that is in the root of the repository, so not the one in the `native` folder, but in the root of the repository. You can do it by adding the following to the `[workspace]` member's array, like this:

```toml
[workspace]
members = [
    ..., // other packages
    "packages/<package_name>/native",
]
```

Now run `cargo build` to check if everything compiles correctly. If it does, you can go on to implement the Rust code in the `api.rs` file as such.

Make sure your Rust code has tests, the Dart code too and an example in Dart works. Also add tests and an example for the Dart code to know the bindings work correctly.

Now run `melos run build` to generate the bindings and the Dart code. If everything works correctly, you can go on to the next step.

#### Wrapping in a Flutter package

## Contributing

We welcome contributions to this project. Please read the [contributing guidelines](CONTRIBUTING.md) for more information.

```

```

```

```

```

```
