// lib/src/ffi.dart
import 'package:flutter_unified_mnemonic/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';
import 'package:unified_mnemonic/unified_mnemonic.dart';

/// Creates an instance of the UnifiedMnemonic class using an instance of
/// a library implementation created by the createLibraryImpl function.
/// This factory method serves as a bridge between the FFI and the Dart
/// object representing the mnemonic functionality.
///
/// Returns:
/// An instance of the UnifiedMnemonic class.
UnifiedMnemonic createLib() => createWrapper(createLibraryImpl());
