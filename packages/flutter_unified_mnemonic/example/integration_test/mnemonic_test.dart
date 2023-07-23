import 'package:flutter/material.dart';
import 'package:flutter_unified_mnemonic_example/main.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Mnemonic is generated', (tester) async {
    await tester.pumpWidget(const MyApp());
    await tester.tap(find.byKey(const Key('generate_mnemonic')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('generate_mnemonic')), findsOneWidget);
  });
}
