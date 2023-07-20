import 'package:unified_mnemonic/src/bridge_generated.dart';

/// Represents the external library for unified_mnemonic
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

UnifiedMnemonic createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();
