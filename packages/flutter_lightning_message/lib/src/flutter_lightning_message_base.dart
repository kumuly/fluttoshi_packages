import 'package:flutter_lightning_message/src/ffi.dart';
import 'package:lightning_message/lightning_message.dart';

/// The bindings to the native functions in the library.
final _bindings = createLib();

Future<Signer> newSignerFromSeed(U8Array64 seed) async {
  return _bindings.fromSeedStaticMethodSigner(seed: seed);
}

Future<Signer> newSignerFromLdkSeed(U8Array32 seed) async {
  return _bindings.fromLdkSeedStaticMethodSigner(seed: seed);
}

Future<bool> verify(String message, String signature, String publicKey) {
  return _bindings.verify(
    message: message,
    signature: signature,
    publicKey: publicKey,
  );
}

Future<String> recoverNodeId(String message, String signature) {
  return _bindings.recoverNodeId(
    message: message,
    signature: signature,
  );
}
