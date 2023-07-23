import 'package:unified_mnemonic/src/bridge_generated.dart';

/// Represents the external library for unified_mnemonic
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

/// Function that creates a platform-specific instance of the [UnifiedMnemonic]
/// class.
///
/// This is a stub implementation that throws an [UnimplementedError]. In a real
/// application, this function should be implemented in platform-specific code
/// (Android, iOS, web, etc.), which interacts with the native APIs to create
/// an instance of [UnifiedMnemonic].
///
/// [lib] is an [ExternalLibrary] instance representing the native library
/// containing the platform-specific implementations of the [UnifiedMnemonic]
/// methods.
///
/// This function returns a [UnifiedMnemonic] instance. In the stub
/// implementation, this function always throws an [UnimplementedError].
UnifiedMnemonic createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();
