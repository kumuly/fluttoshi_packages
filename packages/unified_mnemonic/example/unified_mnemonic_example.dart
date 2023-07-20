import 'dart:ffi';
import 'dart:io';

import 'package:unified_mnemonic/unified_mnemonic.dart';

void main() async {
  final unifiedMnemonic = createWrapper(getLibrary());
  final mnemonic = await Mnemonic.newMnemonic(
    bridge: unifiedMnemonic,
    language: Language.English,
    wordCount: WordCount.Words12,
  );

  print('Mnemonic phrase: ${mnemonic.phrase}');
}

DynamicLibrary getLibrary() {
  // If you are running this example locally, you will need to run
  // `cargo build -r` to generate the needed dylib.
  const libName = 'unified_mnemonic';
  final libPrefix = {
    Platform.isWindows: '',
    Platform.isMacOS: 'lib',
    Platform.isLinux: 'lib',
  }[true]!;
  final libSuffix = {
    Platform.isWindows: 'dll',
    Platform.isMacOS: 'dylib',
    Platform.isLinux: 'so',
  }[true]!;
  final dylibPath = '../../target/release/$libPrefix$libName.$libSuffix';
  return DynamicLibrary.open(dylibPath);
}
