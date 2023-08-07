import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:lightning_message/src/bridge_generated.dart';

/// Represents the external library for unified_mnemonic when running in a web
/// environment.
typedef ExternalLibrary = WasmModule;

/// Function that creates a web-specific instance of the [LightningMessage]
/// class.
///
/// This function is specifically used when the application is running in a
/// web environment. It leverages WebAssembly to handle the native APIs for
/// creating an instance of [LightningMessage].
///
/// [module] is a [WasmModule] instance representing the WebAssembly module
/// containing the web-specific implementations of the [LightningMessage]
/// methods.
///
/// This function returns a [LightningMessage] instance, created using the
/// provided WebAssembly module.
LightningMessage createWrapperImpl(ExternalLibrary module) =>
    LightningMessageImpl.wasm(module);
