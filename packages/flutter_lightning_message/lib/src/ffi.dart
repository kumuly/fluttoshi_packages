import 'package:flutter_lightning_message/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';
import 'package:lightning_message/lightning_message.dart';

/// Creates an instance of the LightningMessage class using an instance of
/// a library implementation created by the createLibraryImpl function.
/// This factory method serves as a bridge between the FFI and the Dart
/// object representing the mnemonic functionality.
///
/// Returns:
/// An instance of the LightningMessage class.
LightningMessage createLib() => createWrapper(createLibraryImpl());
