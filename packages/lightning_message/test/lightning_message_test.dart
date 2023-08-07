import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'package:convert/convert.dart';
import 'package:lightning_message/lightning_message.dart';
import 'package:test/test.dart';

void main() {
  final lightningMessage = createWrapper(useLibrary());
  group('Signer tests - ', () {
    const testMessage = 'Test message';
    late U8Array64 seedArray;

    setUp(() {
      const seed = 'f7eae7a9b7a67d641913956e1e8fc0694884bca82824b3a2f34239a36cc'
          '3bae9ef4668a9305ddc5f750366608c7356687fd0abe188a2055adf4dfd3128669ad'
          '7';
      final seedList = hex.decode(seed);
      seedArray = U8Array64(Uint8List.fromList(seedList));
    });

    test('Signer from seed', () async {
      final signer =
          await lightningMessage.fromSeedStaticMethodSigner(seed: seedArray);
      expect(
        signer.nodeId,
        '03407efa0f513c836c29590660b798ec0705d3e31be9bb3f993fac12331fa649f9',
      );
    });

    test('Sign from seed', () async {
      final signer =
          await lightningMessage.fromSeedStaticMethodSigner(seed: seedArray);
      final signature = await signer.sign(message: testMessage);
      expect(
          signature,
          'dh96kc1xs5iihe56wiz4qjtx8r8cau18cb3kmn3yheyjg3e9ooayai'
          '4baayuic8nxxs34pg567m9uob5pqpw7y43z39gi15o9e8dxyhw');
    });

    test('Verify', () async {
      const signature = 'dh96kc1xs5iihe56wiz4qjtx8r8cau18cb3kmn3yheyjg3e9ooayai'
          '4baayuic8nxxs34pg567m9uob5pqpw7y43z39gi15o9e8dxyhw';
      const nodeId =
          '03407efa0f513c836c29590660b798ec0705d3e31be9bb3f993fac12331fa649f9';
      final isValid = await lightningMessage.verify(
        message: testMessage,
        signature: signature,
        publicKey: nodeId,
      );
      expect(isValid, true);
    });

    test('Verify wrong message', () async {
      const signature = 'dh96kc1xs5iihe56wiz4qjtx8r8cau18cb3kmn3yheyjg3e9ooayai'
          '4baayuic8nxxs34pg567m9uob5pqpw7y43z39gi15o9e8dxyhw';
      const nodeId =
          '03407efa0f513c836c29590660b798ec0705d3e31be9bb3f993fac12331fa649f9';
      final isValid = await lightningMessage.verify(
        message: 'Test message 2',
        signature: signature,
        publicKey: nodeId,
      );
      expect(isValid, false);
    });

    test('Verify wrong node id', () async {
      const signature = 'dh96kc1xs5iihe56wiz4qjtx8r8cau18cb3kmn3yheyjg3e9ooayai'
          '4baayuic8nxxs34pg567m9uob5pqpw7y43z39gi15o9e8dxyhw';
      const nodeId =
          '031c0d8356994448e6859d7d767c792f8af4b86fe19d62e3431aba0488c0522931';
      final isValid = await lightningMessage.verify(
        message: testMessage,
        signature: signature,
        publicKey: nodeId,
      );
      expect(isValid, false);
    });

    test('Recover node id', () async {
      const signature =
          'dhjxcm549zsr57qunj73yuyzskmzqcksaumoiz3r6tg5jziyi54ggg48'
          'zmyhsjdp6ebyuzzomo5k54hxtigpn77pkyzjrunz7shsjz89';
      final nodeId = await lightningMessage.recoverNodeId(
        message: testMessage,
        signature: signature,
      );
      expect(
        nodeId,
        '031c0d8356994448e6859d7d767c792f8af4b86fe19d62e3431aba0488c0522931',
      );
    });
  });
}

DynamicLibrary useLibrary() {
  // If you are running these tests locally, you will need to run
  // `cargo build -r` to generate the needed dylib.
  const libName = 'lightning_message';
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
