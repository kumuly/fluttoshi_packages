import 'package:bip39/src/bridge_generated.dart';
import 'package:bip39/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

Bip39? _wrapper;

/// Creates a wrapper for the `Bip39` functionality using an instance
/// of an `ExternalLibrary` that interfaces with the native code. This wrapper
/// provides a Dart-friendly API for interacting with the unified mnemonic
/// functionality in native code.
///
/// The created wrapper is stored and returned for future calls, allowing the
/// native library to be loaded only once.
///
/// [lib]: An instance of `ExternalLibrary` that serves as the bridge between
/// Dart and the native code.
///
/// Returns:
/// An instance of the `Bip39` class.
Bip39 createWrapper(ExternalLibrary lib) {
  _wrapper ??= createWrapperImpl(lib);
  return _wrapper!;
}
