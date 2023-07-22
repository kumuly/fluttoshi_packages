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

  const recoveryPhrase =
      'empty want equip quick that stuff motion floor oblige prize tower pigeon';
  final recoverdMnemonic = await Mnemonic.fromPhrase(
    bridge: unifiedMnemonic,
    phrase: recoveryPhrase,
  );

  final lightningSeed =
      await recoverdMnemonic.deriveLightningSeed(network: Network.Bitcoin);

  // Todo: create a utility function in the package to convert the seed to hex.
  final lightningSeedHex = lightningSeed
      .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
      .join();
  print('Seed for Lightning node: $lightningSeedHex');
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
