import 'dart:ffi';
import 'dart:io';

import 'package:lightning_message/lightning_message.dart';

void main() async {
  final lightningMessage = createWrapper(useLibrary());
  const signature = 'dhjxcm549zsr57qunj73yuyzskmzqcksaumoiz3r6tg5jziyi54ggg48'
      'zmyhsjdp6ebyuzzomo5k54hxtigpn77pkyzjrunz7shsjz89';
  final nodeId = await lightningMessage.recoverNodeId(
    message: 'Test message',
    signature: signature,
  );
  assert(
    nodeId ==
        '031c0d8356994448e6859d7d767c792f8af4b86fe19d62e3431aba0488c0522931',
    'Node id: $nodeId',
  );
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
