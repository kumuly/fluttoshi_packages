import 'package:flutter/material.dart';
import 'dart:async';

import 'package:flutter_lightning_message/flutter_lightning_message.dart'
    as flutter_lightning_message;

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final message = 'Test message';
  final signature = 'dh96kc1xs5iihe56wiz4qjtx8r8cau18cb3kmn3yheyjg3e9ooayai'
      '4baayuic8nxxs34pg567m9uob5pqpw7y43z39gi15o9e8dxyhw';
  late String nodeId;

  @override
  Future<void> initState() async {
    super.initState();
    nodeId = await flutter_lightning_message.recoverNodeId(message, signature);
  }

  @override
  Widget build(BuildContext context) {
    const textStyle = TextStyle(fontSize: 25);
    const spacerSmall = SizedBox(height: 10);
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(10),
            child: Column(
              children: [
                const Text(
                  'This calls a native function through FFI that is shipped as source in the package. '
                  'The native code is built as part of the Flutter Runner build.',
                  style: textStyle,
                  textAlign: TextAlign.center,
                ),
                spacerSmall,
                Text(
                  'recovered node id = $nodeId',
                  style: textStyle,
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
