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

  assert(
    mnemonic.phrase.split(' ').length == 12,
    'Phrase should have 12 words',
  );

  const recoveryPhrase = 'goat magnet speed sweet release pill '
      'tiny decline talent extra sunny diamond';
  final recoverdMnemonic = await Mnemonic.fromPhrase(
    bridge: unifiedMnemonic,
    phrase: recoveryPhrase,
  );

  final lightningSeed =
      await recoverdMnemonic.deriveLightningSeed(network: Network.Bitcoin);
  assert(
    lightningSeed.length == 32,
    'Lightning seed should be 32 bytes',
  );

  final lightningSeedHex = lightningSeed
      .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
      .join();
  assert(
    lightningSeedHex ==
        '426540629d356f207fd792c0215e787ded943a1c405a4353f7174926bb6fe129',
    'LightningSeedHex should be the same as the one derived from the mnemonic',
  );
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
