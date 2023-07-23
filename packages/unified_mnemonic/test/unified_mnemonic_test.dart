import 'dart:ffi';
import 'dart:io';

import 'package:test/test.dart';
import 'package:unified_mnemonic/unified_mnemonic.dart';

void main() {
  final unifiedMnemonic = createWrapper(useLibrary());
  group('An English Mnemonic Generation', () {
    setUp(() {
      // Additional setup goes here.
    });

    test('with 12 Words', () async {
      final mnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      expect(mnemonic.language, Language.English);
      expect(mnemonic.phrase.split(' ').length, 12);
    });

    test('with 24 Words', () async {
      final mnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words24,
      );
      expect(mnemonic.language, Language.English);
      expect(mnemonic.phrase.split(' ').length, 24);
    });

    test('has different phrases with every generation', () async {
      final mnemonic1 = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      final mnemonic2 = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      expect(mnemonic1.phrase, isNot(mnemonic2.phrase));
    });
  });

  group('A Spanish Mnemonic Generation', () {
    test('with 12 Words', () async {
      final mnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.Spanish,
        wordCount: WordCount.Words12,
      );
      expect(mnemonic.language, Language.Spanish);
      expect(mnemonic.phrase.split(' ').length, 12);
    });

    test('with 24 Words', () async {
      final mnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.Spanish,
        wordCount: WordCount.Words24,
      );
      expect(mnemonic.language, Language.Spanish);
      expect(mnemonic.phrase.split(' ').length, 24);
    });
  });

  group('A Mnemonic Recovery', () {
    test('with 12 English Words', () async {
      final lostMnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      final recoveredMnemonic = await Mnemonic.fromPhrase(
        bridge: unifiedMnemonic,
        phrase: lostMnemonic.phrase,
      );
      expect(recoveredMnemonic.language, Language.English);
      expect(recoveredMnemonic.phrase.split(' ').length, 12);
      expect(recoveredMnemonic.phrase, lostMnemonic.phrase);
    });

    test('with 24 English Words', () async {
      final lostMnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words24,
      );
      final recoveredMnemonic = await Mnemonic.fromPhrase(
        bridge: unifiedMnemonic,
        phrase: lostMnemonic.phrase,
      );
      expect(recoveredMnemonic.language, Language.English);
      expect(recoveredMnemonic.phrase.split(' ').length, 24);
      expect(recoveredMnemonic.phrase, lostMnemonic.phrase);
    });

    test('with 12 Spanish Words', () async {
      final lostMnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.Spanish,
        wordCount: WordCount.Words12,
      );
      final recoveredMnemonic = await Mnemonic.fromPhrase(
        bridge: unifiedMnemonic,
        phrase: lostMnemonic.phrase,
      );
      expect(recoveredMnemonic.language, Language.Spanish);
      expect(recoveredMnemonic.phrase.split(' ').length, 12);
      expect(recoveredMnemonic.phrase, lostMnemonic.phrase);
    });

    test('with 24 Spanish Words', () async {
      final lostMnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.Spanish,
        wordCount: WordCount.Words24,
      );
      final recoveredMnemonic = await Mnemonic.fromPhrase(
        bridge: unifiedMnemonic,
        phrase: lostMnemonic.phrase,
      );
      expect(recoveredMnemonic.language, Language.Spanish);
      expect(recoveredMnemonic.phrase.split(' ').length, 24);
      expect(recoveredMnemonic.phrase, lostMnemonic.phrase);
    });
  });

  group('A Lightning Seed Derivation', () {
    test('should be of 32 Bytes', () async {
      final mnemonic = await Mnemonic.newMnemonic(
        bridge: unifiedMnemonic,
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      final seed = await mnemonic.deriveLightningSeed(
        network: Network.Testnet,
      );
      expect(seed.length, 32);
    });

    test('should be consistent with BIP32 and BIP39 on Bitcoin Network',
        () async {
      const mnemonicPhrase = 'goat magnet speed sweet release pill '
          'tiny decline talent extra sunny diamond';
      final mnemonic = await Mnemonic.fromPhrase(
        bridge: unifiedMnemonic,
        phrase: mnemonicPhrase,
      );
      final lightningSeed = await mnemonic.deriveLightningSeed(
        network: Network.Bitcoin,
      );
      final lightningSeedHex = lightningSeed
          .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
          .join();
      expect(
        lightningSeedHex,
        '426540629d356f207fd792c0215e787ded943a1c405a4353f7174926bb6fe129',
      );
    });
  });
}

DynamicLibrary useLibrary() {
  // If you are running these tests locally, you will need to run
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
