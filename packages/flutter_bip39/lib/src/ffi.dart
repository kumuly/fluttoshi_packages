// lib/src/ffi.dart
import 'package:bip39/bip39.dart';
import 'package:flutter_bip39/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

Bip39 createLib() => createWrapper(createLibraryImpl());
