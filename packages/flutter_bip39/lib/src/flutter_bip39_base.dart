import 'dart:async';

import 'package:bip39/bip39.dart';
import 'package:flutter_bip39/src/ffi.dart';

/// The bindings to the native functions in the library.
final Bip39 _bindings = createLib();

/// Returns all available languages.
///
/// This function fetches and returns all languages that are available in the
/// Bip39 bindings.
Future<List<Language>> allLanguages() async {
  return _bindings.allStaticMethodLanguage();
}

/// Returns word list for a specific language.
///
/// Given a [language], this function fetches and returns all words in the
/// bip39 dictionary of the specified language.
Future<List<String>> wordList(Language language) async {
  return _bindings.wordListMethodLanguage(that: language);
}

/// Returns words in the bip39 dictionary for a specific language that start
/// with the prefix.
///
/// Given a [language] and a [prefix], this function fetches and returns all
/// words in the language that start with the prefix.
Future<List<String>> wordsByPrefix(Language language, String prefix) async {
  return _bindings.wordsByPrefixMethodLanguage(that: language, prefix: prefix);
}

/// Returns the index of a word in a specific language's bip39 dictionary.
///
/// Given a [language] and a [word], this function returns the index of the word
/// in the specific language's word list.
/// If the word is not found, the function returns -1.
Future<int> findWord(Language language, String word) async {
  return _bindings.findWordMethodLanguage(that: language, word: word);
}

/// Generates a new mnemonic in the specified language and word count.
///
/// Given a [language] and a [wordCount], this function generates a new
/// mnemonic phrase.
/// The [language] is used to determine the word list used for the phrase,
/// and the [wordCount] determines the number of words in the mnemonic.
Future<Mnemonic> generateIn(Language language, WordCount wordCount) async {
  return _bindings.generateInStaticMethodMnemonic(
    language: language,
    wordCount: wordCount,
  );
}

/// Parses a list of words into a mnemonic.
///
/// Given a [List<String>] of words, this function attempts to parse it into a
/// [Mnemonic]. If the words form a valid mnemonic, it is returned. Otherwise,
/// an error is thrown.
Future<Mnemonic> parse(List<String> words) async {
  return _bindings.parseStaticMethodMnemonic(words: words);
}

/// Parses a list of words in a specific language into a mnemonic.
///
/// Given a [language] and a [List<String>] of words, this function attempts to
/// parse the words into a [Mnemonic] in the specific language.
/// If the words form a valid mnemonic in the specified language, it is
/// returned. Otherwise, an error is thrown.
Future<Mnemonic> parseIn(Language language, List<String> words) async {
  return _bindings.parseInStaticMethodMnemonic(
    language: language,
    words: words,
  );
}
