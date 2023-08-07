[![Build & Test](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml/badge.svg)](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml)

# Fluttoshi packages

We think that all apps that need any kind of global payment mechanism are the best of with Bitcoin on the Lightning Network. We envision a near future where all apps will have a lightweight, non-custodial Lightning wallet embedded, even if they are not necesarily 'Bitcoin' or payment apps at their core.

We want to make it as easy as possible for this to happen. Therefore the packages in this project are designed to be as modular as possible, so that they can be used as building blocks in any Flutter app, regardless of the app's purpose. Also apps that do want to make more full-fledged Bitcoin and Lightning apps might find these packages useful. A monorepo is used instead of a repo per package to guarantee all packages and their dependency versions are compatible with each other on the same versions.

We also believe that Flutter is the best framework to build cross-platform apps and that Rust is the best language to build the core functionality of these apps. Therefore we are building these packages in Rust and Flutter.

We mainly use the [Lightning Development Kit](https://www.lightningdevkit.org) and [Bitcoin Development Kit](https://www.bitcoindevkit.org) to have trusted, well-tested and well-maintained Rust libraries to build on.

## Packages

- [x] `bip39`: package to generate and/or restore mnemonics and derive other keys and seeds from them.
- [x] `lightning_message`: package to sign and verify Lightning messages. Useful for proving ownership of a node, which in turn is useful for login mechanisms to get rid of username and passwords in apps, amongst other things.
- [ ] `unified_wallet`: package to derive different seeds from the same mnemonic for different wallet types. For example, have a Bitcoin on-chain wallet, a Bitcoin on-chain wallet with passphrase and a Lightning node wallet, all derived from and unified by just one mnemonic to safely store.
- [ ] `breez_sdk`: package to set up a Lightning node, based on Breez SDK, that is compatible with the other packages in this repo.
- [ ] `ldk`: package to set up a Lightning node, based on LDK, in a Flutter app.
- [ ] `ldk_bloc`: package that uses the ldk package and provides a Bloc to use in Flutter apps.
- [ ] `ldk_riverpod`: package that uses the ldk package and provides a Riverpod provider to use in Flutter apps.
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

Now we can wrap the Dart code with the native package in a Flutter package.

Let's use the plugin_ffi template for it. Change `<package_name>` for the correct name and run the following command in the root of the repository:

```bash
flutter create --template=plugin_ffi --platforms=android,ios,macos,linux,windows --org=dev.kumuly packages/flutter_<package_name>
```

Add a description, repository and homepage to the package `pubspec.yaml` file and set the correct version in accordance with the other packages in the monorepo.

Delete the `analysis_options.yaml` file in the `packages/flutter_<package_name>` folder as you also did for the other packages since we use the one in the root of the repository.ß

Now we can add the native package as a dependency to the Flutter package. Use the version syntax, e.g. ^1.0.0. Melos will take care of the dependency resolution for us.

```yaml
dependencies:
  <package_name>: <version>
```

To export your Dart-only package to Flutter and some possible extra wrappers you can replace the code in the `packages/flutter_<package_name>/lib/flutter_<package_name>.dart` file with the following:

```dart
/// Todo: add a description.
///
/// More dartdocs go here.
library;

// Export any libraries intended for clients of this package.
export 'package:<package_name>/<package_name>.dart';

export 'src/flutter_<package_name>_base.dart';
```

From the `packages/flutter_<package_name>` folder, remove the `packages/lib/flutter_<package_name>_bindings_generated.dart` file, add a `packages/lib/src/flutter_<package_name>_base.dart`.

```bash
cd packages/flutter_<package_name>
rm lib/flutter_<package_name>_bindings_generated.dart
mkdir lib/src
touch lib/src/flutter_<package_name>_base.dart
```

The base file can export easier to use wrappers around the bindings.

Now finally, we need to write some code to be able to handle the FFI in Flutter.
Create the following files and add the respective code replacing the `<package_name>` for the correct name:

- `lib/src/ffi/stub.dart`

  ```dart
  Object createLibraryImpl() => throw UnimplementedError();
  ```

- `lib/src/ffi/io.dart`

  ```dart
  import 'dart:ffi';
  import 'dart:io';

  DynamicLibrary createLibraryImpl() {
  const base = '<package_name>';

    if (Platform.isIOS || Platform.isMacOS) {
        return DynamicLibrary.executable();
    } else if (Platform.isWindows) {
        return DynamicLibrary.open('$base.dll');
    } else {
        return DynamicLibrary.open('lib$base.so');
    }
  }
  ```

- `lib/src/ffi/web.dart`

  ```dart
  import 'package:<package_name>/<package_name>.dart';

  WasmModule createLibraryImpl() {
  // TODO add web support. See:
  // https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/lib/ffi.web.dart
  throw UnsupportedError('Web support is not provided yet.');
  }
  ```

- `lib/src/ffi.dart`

  ```dart
  import 'package:<package_name>/<package_name>.dart';
  import 'ffi/stub.dart'
      if (dart.library.io) 'ffi/io.dart'
      if (dart.library.html) 'ffi/web.dart';

  <PackageName> createLib() =>
      createWrapper(createLibraryImpl());
  ```

Finally, run `melos bs` to resolve the dependencies inside the monorepo.

Now in the `lib/src/flutter_<package_name>_base.dart` file you can add the wrappers around the bindings by obtaining the bindings with `createLib()` and then using the bindings to create the wrappers. Like this:

```dart
import 'dart:async';

import 'package:<package_name>/<package_name>.dart';
import 'package:flutter_<package_name>/src/ffi.dart';

/// The bindings to the native functions in the library.
final _bindings = createLib();

// Add wrapper methods around the bindings' static and global methods.
```

It can be useful to create these wrappers for static methods generated by the bindings.

##### Bundling the native libraries

Now we need to bundle the native libraries with the Flutter package with the correct binaries for every platform.

###### Linux and Windows

Windows and Linux both use a CMake build system, but the CMakeLists.txt files are different. So we need to adapt the CMakeLists.txt file for both platforms.
Replace the `/packages/flutter_<package_name>/linux/CMakeLists.txt` with the following, replacing the `<package_name>` and `<version>` for the correct name and version:

```cmake
set(LibraryVersion "<package_name>-v<version>") # generated; do not edit

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "flutter_<package_name>")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}")
set(ArchivePath "${LibRoot}.tar.gz")
if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/kumuly/fluttoshi_packages/releases/download/${LibraryVersion}/other.tar.gz"
    ${ArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LibRoot})
file(MAKE_DIRECTORY ${LibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${ArchivePath}
  WORKING_DIRECTORY ${LibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(flutter_<package_name>_bundled_libraries
  "${LibRoot}/${FLUTTER_TARGET_PLATFORM}/lib<package_name>.so"
  PARENT_SCOPE
)
```

Replace the `/packages/flutter_<package_name>/windows/CMakeLists.txt` with the following, replacing the `<package_name>` and `<version>` for the correct name and version:

```cmake
set(LibraryVersion "<package_name>-v<version>") # generated; do not edit

# TODO Remove this workaround once Flutter supports Windows ARM.
# https://github.com/flutter/flutter/issues/116196
set(FLUTTER_TARGET_PLATFORM windows-x64)

# The Flutter tooling requires that developers have a version of Visual Studio
# installed that includes CMake 3.14 or later. You should not increase this
# version, as doing so will cause the plugin to fail to compile for some
# customers of the plugin.
cmake_minimum_required(VERSION 3.14)

# Project-level configuration.
set(PROJECT_NAME "flutter_<package_name>")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}")
set(ArchivePath "${LibRoot}.tar.gz")
if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/kumuly/fluttoshi_packages/releases/download/${LibraryVersion}/other.tar.gz"
    ${ArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LibRoot})
file(MAKE_DIRECTORY ${LibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${ArchivePath}
  WORKING_DIRECTORY ${LibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(flutter_<package_name>_bundled_libraries
  "${LibRoot}/${FLUTTER_TARGET_PLATFORM}/<package_name>.dll"
  PARENT_SCOPE
)
```

Add the following `.gitignore` to both the linux and windows folders:

```gitignore
# Set up as allowlist
*

# Allowed files
!.gitignore
!CMakeLists.txt
```

Add <package_name> to the [build-other script](/scripts/build-other.sh) and [copy-other script](/scripts/copy-other.sh).

###### iOS and macOS

Create `ios/Classes/EnforceBundling.swift` and `macos/Classes/EnforceBundling.swift` in the Flutter package with the following code in both:

```swift
public func dummyMethodToEnforceBundling() -> Int64 {
  return dummy_method_to_enforce_bundling()
}
let dummyVar = dummyMethodToEnforceBundling();
```

Add an empty `.gitkeep` file at `ios/Frameworks/.gitkeep` and `macos/Frameworks/.gitkeep`.
Add a `.gitignore` file at `ios/.gitignore` with following content:

```gitignore
Flutter/
Runner/
Frameworks/*
!Frameworks/.gitkeep
```

And `macos/.gitignore` with the following content:

```gitignore
Flutter/
Frameworks/*
!Frameworks/.gitkeep
```

Now add the following to `ios/flutter_<package_name>.podspec` and `macos/flutter_<package_name>.podspec` and replace `<package_name>`, `<version>` and `<PackageName>` with the correct values:

```podspec
release_tag_name = '<package_name>-v<version>' # generated; do not edit

# We cannot distribute the XCFramework alongside the library directly,
# so we have to fetch the correct version here.
framework_name = '<PackageName>.xcframework'
remote_zip_name = "#{framework_name}.zip"
url = "https://github.com/kumuly/fluttoshi_packages/releases/download/#{release_tag_name}/#{remote_zip_name}"
local_zip_name = "#{release_tag_name}.zip"
`
cd Frameworks
rm -rf #{framework_name}

if [ ! -f #{local_zip_name} ]
then
  curl -L #{url} -o #{local_zip_name}
fi

unzip #{local_zip_name}
cd -
`

Pod::Spec.new do |spec|
  spec.name          = 'flutter_<package_name>'
  spec.version       = '0.0.1'
  spec.license       = { :file => '../LICENSE' }
  spec.homepage      = 'https://github.com/kumuly/fluttoshi_packages'
  spec.authors       = { 'Your Name' => 'your-email@example.com' }
  spec.summary       = 'iOS/macOS Flutter bindings for <package_name>'

  spec.source              = { :path => '.' }
  spec.source_files        = 'Classes/**/*'
  spec.public_header_files = 'Classes/**/*.h'
  spec.vendored_frameworks = "Frameworks/#{framework_name}"

  spec.ios.deployment_target = '11.0'
  spec.osx.deployment_target = '10.14'
end
```

Now add the package_name to the [build-apple script](/scripts/build-apple.sh) and [copy-apple script](/scripts/copy-apple.sh).

###### Android

Add a `CMakeLists.txt` file to `packages/flutter_<package_name>/android/` folder with the following content, replacing `<package_name>` and `<version>` with the correct values:

```cmake
set(LibraryVersion "<package_name>-v<version>") # generated; do not edit

# Unlike the Windows & Linux CMakeLists.txt, this Android equivalent is just here
# to download the Android binaries into src/main/jniLibs/ and does not build anything.
# The binary download/extraction is difficult to do concisely in Groovy/Gradle,
# at least across host platforms, so we are just reusing our Linux/Windows logic.

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")
set(ArchivePath "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}.tar.gz")

if(NOT EXISTS ${ArchivePath})
    file(DOWNLOAD
        "https://github.com/kumuly/fluttoshi_packages/releases/download/${LibraryVersion}/android.tar.gz"
        ${ArchivePath}
        TLS_VERIFY ON
    )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LibRoot})
file(MAKE_DIRECTORY ${LibRoot})
execute_process(
    COMMAND ${CMAKE_COMMAND} -E tar xzf ${ArchivePath}
    WORKING_DIRECTORY ${LibRoot}
)
```

Replace the android {...} section at the bottom of build.gradle with the following:

```gradle
android {
    compileSdkVersion 31

    defaultConfig {
        minSdkVersion 16
    }

    // Trigger the binary download/update over in CMakeLists.txt
    externalNativeBuild {
        cmake {
            path "CMakeLists.txt"
        }
    }
}
```

Add the following to the `.gitignore` file in the android folder of the Flutter package:

```gitignore
# Ignore Rust binaries
src/main/jniLibs/
*.tar.gz
```

Now add the package_name to the [build-android script](/scripts/build-android.sh) and [copy-android script](/scripts/copy-android.sh).

##### Melos version script

Add the package to the [version script](/scripts/version.sh) so melos can bump the version automatically.

## Contributing

We welcome contributions to this project. Please read the [contributing guidelines](CONTRIBUTING.md) for more information.
