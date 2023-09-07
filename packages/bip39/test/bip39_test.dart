import 'dart:ffi';
import 'dart:io';

import 'package:bip39/bip39.dart';
import 'package:flutter_rust_bridge/src/ffi/io.dart';
import 'package:test/test.dart';

void main() {
  final bip39 = createWrapper(useLibrary());
  group('Language tests - ', () {
    setUp(() {
      // Additional setup goes here.
    });

    test('Support English and Spanish', () async {
      final languages = await bip39.allStaticMethodLanguage();
      expect(languages, [Language.English, Language.Spanish]);
    });

    test('Get words by prefix', () async {
      const prefix = 'ab';
      final words = await bip39.wordsByPrefixMethodLanguage(
        that: Language.English,
        prefix: prefix,
      );
      expect(
        words,
        [
          'abandon',
          'ability',
          'able',
          'about',
          'above',
          'absent',
          'absorb',
          'abstract',
          'absurd',
          'abuse',
        ],
      );
    });

    test('Find words', () async {
      const abandon = 'abandon';
      final abandonIndex = await bip39.findWordMethodLanguage(
        that: Language.English,
        word: abandon,
      );
      expect(abandonIndex, 0);

      const about = 'about';
      final abountIndex = await bip39.findWordMethodLanguage(
        that: Language.English,
        word: about,
      );
      expect(abountIndex, 3);

      const notFound = 'notFound';
      final notFoundIndex = await bip39.findWordMethodLanguage(
        that: Language.English,
        word: notFound,
      );
      expect(notFoundIndex, -1);
    });
  });

  group('Mnemonic tests - ', () {
    final words = [
      'mind',
      'boil',
      'voice',
      'table',
      'general',
      'gym',
      'lunar',
      'exhaust',
      'shoulder',
      'usage',
      'hero',
      'father',
    ];
    test('Generate mnemonic', () async {
      final mnemonic = await bip39.generateInStaticMethodMnemonic(
        language: Language.English,
        wordCount: WordCount.Words12,
      );
      expect(mnemonic.words.length, 12);
    });

    test('Recover mnemonic', () async {
      final mnemonic = await bip39.parseStaticMethodMnemonic(words: words);
      expect(mnemonic.words, words);
      expect(mnemonic.language, Language.English);
    });

    test('Derive seed', () async {
      final mnemonic = await bip39.parseStaticMethodMnemonic(words: words);
      final seed = await mnemonic.toSeed(passphrase: '');
      final seedHex =
          seed.map((byte) => byte.toRadixString(16).padLeft(2, '0')).join();
      expect(
        seedHex,
        'f7eae7a9b7a67d641913956e1e8fc0694884bca82824b3a2f34239a36cc3b'
        'ae9ef4668a9305ddc5f750366608c7356687fd0abe188a2055adf4dfd3128669ad7',
      );
    });
  });
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
