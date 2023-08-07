import 'package:lightning_message/lightning_message.dart';

/// This function is expected to provide a [WasmModule] for web platform.
///
/// Currently, this function does not support web and throws an
/// [UnsupportedError] upon invocation. The implementation for web support will
/// be added in future updates.
///
/// For reference on how to add web support, see:
/// [Flutter Rust Bridge example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/lib/ffi.web.dart)
///
/// Throws:
///   - [UnsupportedError]: when called, indicating that the web support is not
/// provided yet.
WasmModule createLibraryImpl() {
  // Add web support. See:
  // https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/lib/ffi.web.dart
  throw UnsupportedError('Web support is not provided yet.');
}
