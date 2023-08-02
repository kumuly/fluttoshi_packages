import 'package:bip39/src/bridge_generated.dart';

/// Represents the external library for bip39
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

/// Function that creates a platform-specific instance of the [Bip39]
/// class.
///
/// This is a stub implementation that throws an [UnimplementedError]. In a real
/// application, this function should be implemented in platform-specific code
/// (Android, iOS, web, etc.), which interacts with the native APIs to create
/// an instance of [Bip39].
///
/// [lib] is an [ExternalLibrary] instance representing the native library
/// containing the platform-specific implementations of the [Bip39]
/// methods.
///
/// This function returns a [Bip39] instance. In the stub
/// implementation, this function always throws an [UnimplementedError].
Bip39 createWrapperImpl(ExternalLibrary lib) => throw UnimplementedError();
