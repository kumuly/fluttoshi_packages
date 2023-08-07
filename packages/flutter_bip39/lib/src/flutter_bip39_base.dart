import 'dart:async';

import 'package:bip39/bip39.dart';
import 'package:flutter_bip39/src/ffi.dart';

/// The bindings to the native functions in the library.
final Bip39 _bindings = createLib();

Future<List<Language>> allLanguages() async {
  return _bindings.allStaticMethodLanguage();
}

Future<List<String>> wordList(Language language) async {
  return _bindings.wordListMethodLanguage(that: language);
}

Future<List<String>> wordsByPrefix(Language language, String prefix) async {
  return _bindings.wordsByPrefixMethodLanguage(that: language, prefix: prefix);
}

Future<int> findWord(Language language, String word) async {
  return _bindings.findWordMethodLanguage(that: language, word: word);
}

Future<Mnemonic> generateIn(Language language, WordCount wordCount) async {
  return _bindings.generateInStaticMethodMnemonic(
    language: language,
    wordCount: wordCount,
  );
}

Future<Mnemonic> parse(List<String> words) async {
  return _bindings.parseStaticMethodMnemonic(words: words);
}

Future<Mnemonic> parseIn(Language language, List<String> words) async {
  return _bindings.parseInStaticMethodMnemonic(
    language: language,
    words: words,
  );
}
