import 'package:unified_mnemonic/src/bridge_generated.dart';
import 'package:unified_mnemonic/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

UnifiedMnemonic? _wrapper;

/// Creates a wrapper for the `UnifiedMnemonic` functionality using an instance
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
/// An instance of the `UnifiedMnemonic` class.
UnifiedMnemonic createWrapper(ExternalLibrary lib) {
  _wrapper ??= createWrapperImpl(lib);
  return _wrapper!;
}
