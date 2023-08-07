// lib/src/ffi.dart
import 'package:bip39/bip39.dart';
import 'package:flutter_bip39/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

/// Creates an instance of the Bip39 class using an instance of
/// a library implementation created by the createLibraryImpl function.
/// This factory method serves as a bridge between the FFI and the Dart
/// object representing the mnemonic functionality.
///
/// Returns:
/// An instance of the Bip39 class.
Bip39 createLib() => createWrapper(createLibraryImpl());
