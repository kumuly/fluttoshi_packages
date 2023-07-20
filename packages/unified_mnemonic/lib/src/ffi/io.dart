import 'dart:ffi';

import 'package:unified_mnemonic/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

UnifiedMnemonic createWrapperImpl(ExternalLibrary dylib) =>
    UnifiedMnemonicImpl(dylib);
