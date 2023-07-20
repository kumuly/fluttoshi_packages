// lib/src/ffi.dart
import 'package:unified_mnemonic/unified_mnemonic.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

UnifiedMnemonic createLib() => createWrapper(createLibraryImpl());
