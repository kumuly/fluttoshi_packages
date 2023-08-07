import 'dart:ffi';
import 'dart:io';

import 'package:bip39/bip39.dart';

Future<void> main() async {
  final bip39 = createWrapper(useLibrary());
  final mnemonic = await bip39.generateInStaticMethodMnemonic(
    language: Language.English,
    wordCount: WordCount.Words12,
  );

  assert(mnemonic.words.length == 12, 'Mnemonic should have 12 words');
}

DynamicLibrary useLibrary() {
  // If you are running these tests locally, you will need to run
  // `cargo build -r` to generate the needed dylib.
  const libName = 'bip39';
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
