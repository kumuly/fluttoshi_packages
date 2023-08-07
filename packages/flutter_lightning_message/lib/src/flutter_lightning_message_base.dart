import 'package:flutter_lightning_message/src/ffi.dart';
import 'package:lightning_message/lightning_message.dart';

/// The bindings to the native functions in the library.
final _bindings = createLib();

/// Creates a new Signer instance from a given 64-byte seed.
///
/// The seed is used to derive the private key for the signer.
/// This method communicates with the underlying C library to create the signer.
///
/// [seed]: The 64-byte seed from which the secret key will be derived.
Future<Signer> newSignerFromSeed(U8Array64 seed) async {
  return _bindings.fromSeedStaticMethodSigner(seed: seed);
}

/// Creates a new Signer instance from a given 32-byte seed used by LDK.
///
/// The seed is used to derive the private key for the signer.
/// This method communicates with the underlying C library to create the signer.
///
/// [seed]: The 32-byte seed used by LDK from which the private key will be
/// derived.
Future<Signer> newSignerFromLdkSeed(U8Array32 seed) async {
  return _bindings.fromLdkSeedStaticMethodSigner(seed: seed);
}

/// Verifies the signature of a given message against a given public key.
///
/// The message is expected to be signed by the private key associated with the
/// provided public key.
///
/// [message]: The signed message.
/// [signature]: The signature of the message.
/// [publicKey]: The public key or node id of the signer
Future<bool> verify(String message, String signature, String publicKey) {
  return _bindings.verify(
    message: message,
    signature: signature,
    publicKey: publicKey,
  );
}

/// Recovers the node id (public key) from a given message and signature.
///
/// This is typically used in message verification, to recover the signer's node
///  id.
///
/// [message]: The signed message.
/// [signature]: The signature of the message.
Future<String> recoverNodeId(String message, String signature) {
  return _bindings.recoverNodeId(
    message: message,
    signature: signature,
  );
}
