import 'package:flutter_lightning_message/src/ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';
import 'package:lightning_message/lightning_message.dart';

LightningMessage createLib() => createWrapper(createLibraryImpl());
