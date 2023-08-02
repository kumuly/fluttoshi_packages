import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:bip39/src/bridge_generated.dart';

/// Represents the external library for bip39 when running in a web
/// environment.
typedef ExternalLibrary = WasmModule;

/// Function that creates a web-specific instance of the [Bip39]
/// class.
///
/// This function is specifically used when the application is running in a
/// web environment. It leverages WebAssembly to handle the native APIs for
/// creating an instance of [Bip39].
///
/// [module] is a [WasmModule] instance representing the WebAssembly module
/// containing the web-specific implementations of the [Bip39]
/// methods.
///
/// This function returns a [Bip39] instance, created using the
/// provided WebAssembly module.
Bip39 createWrapperImpl(ExternalLibrary module) => Bip39Impl.wasm(module);
