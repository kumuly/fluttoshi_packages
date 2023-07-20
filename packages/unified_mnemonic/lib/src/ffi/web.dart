import 'package:unified_mnemonic/src/bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

typedef ExternalLibrary = WasmModule;

UnifiedMnemonic createWrapperImpl(ExternalLibrary module) =>
    UnifiedMnemonicImpl.wasm(module);
