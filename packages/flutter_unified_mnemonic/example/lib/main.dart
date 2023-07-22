import 'package:flutter/material.dart';

import 'package:flutter_unified_mnemonic/flutter_unified_mnemonic.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late Mnemonic newMnemonic;
  late Mnemonic recoveredMnemonic;
  late String newMnemonicLightningSeedHex;
  late String recoveredMnemonicLightningSeedHex;

  @override
  void initState() {
    super.initState();
    initAsyncState();
  }

  Future<void> initAsyncState() async {
    newMnemonic = await generateNewMnemonic(
        language: Language.Spanish, wordCount: WordCount.Words24);
    final newMnemonicLightningSeed =
        await newMnemonic.deriveLightningSeed(network: Network.Bitcoin);
    newMnemonicLightningSeedHex = newMnemonicLightningSeed
        .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
        .join();

    const recoveryPhrase =
        'goat magnet speed sweet release pill tiny decline talent extra sunny diamond';
    recoveredMnemonic = await recoverMnemonicFromPhrase(phrase: recoveryPhrase);
    final recoveredMnemonicLightningSeed =
        await recoveredMnemonic.deriveLightningSeed(network: Network.Bitcoin);
    recoveredMnemonicLightningSeedHex = recoveredMnemonicLightningSeed
        .map((byte) => byte.toRadixString(16).padLeft(2, '0'))
        .join();
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
        body: FutureBuilder(
          future: initAsyncState(),
          builder: (context, snapshot) {
            if (snapshot.connectionState == ConnectionState.waiting) {
              return const CircularProgressIndicator();
            } else if (snapshot.hasError) {
              return Text('Error: ${snapshot.error}');
            } else {
              return SingleChildScrollView(
                child: Container(
                  padding: const EdgeInsets.all(10),
                  child: Column(
                    children: [
                      const Text(
                        'This calls a native function through FFI that is build for the platform it is running on.',
                        style: textStyle,
                        textAlign: TextAlign.center,
                      ),
                      spacerSmall,
                      Text(
                        'new mnemonic = ${newMnemonic.phrase}',
                        style: textStyle,
                        textAlign: TextAlign.center,
                      ),
                      spacerSmall,
                      Text(
                        'new mnemonic lightning seed = $newMnemonicLightningSeedHex',
                        style: textStyle,
                        textAlign: TextAlign.center,
                      ),
                      spacerSmall,
                      Text(
                        'recovered mnemonic lightning seed = $recoveredMnemonicLightningSeedHex',
                        style: textStyle,
                        textAlign: TextAlign.center,
                      ),
                    ],
                  ),
                ),
              );
            }
          },
        ),
      ),
    );
  }
}
