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
          'abuse'
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
