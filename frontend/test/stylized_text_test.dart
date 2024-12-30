import 'package:financrr_frontend/utils/text_utils.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('This is a test', () {
    const String template = 'This is a test';
    const String localized = 'This is a test';
    final List<InlineSpan> spans = TextUtils.richFormatL10nText(template, localized, namedStyles: {});
    expect(spans.length, 1);
  });

  test('This is {a} test', () {
    const String template = 'This is {a} test';
    const String localized = 'This is b test';
    final List<InlineSpan> spans = TextUtils.richFormatL10nText(template, localized, namedStyles: {}, namedArgs: {'a': 'b'});
    expect(spans.length, 1);
    expect(spans[0].toPlainText(), 'This is b test');
  });

  test('This is {a} test (with namedStyles)', () {
    const String template = 'This is {a} test';
    const String localized = 'This is b test';
    final List<InlineSpan> spans = TextUtils.richFormatL10nText(template, localized,
        namedStyles: {'a': (style) => style.copyWith(color: Colors.red)}, namedArgs: {'a': 'b'});
    expect(spans.length, 3);
    expect(spans[0].toPlainText(), 'This is ');
    expect(spans[1].toPlainText(), 'b');
    expect(spans[2].toPlainText(), ' test');
    expect((spans[1] as TextSpan).style!.color, Colors.red);
  });

  test('This is {a} test (with namedStyles and style)', () {
    const String template = 'This is {a} test';
    const String localized = 'This is b test';
    final List<InlineSpan> spans = TextUtils.richFormatL10nText(template, localized,
        namedStyles: {'a': (style) => style.copyWith(color: Colors.red)},
        style: const TextStyle(color: Colors.blue),
        namedArgs: {'a': 'b'});
    expect(spans.length, 3);
    expect(spans[0].toPlainText(), 'This is ');
    expect(spans[1].toPlainText(), 'b');
    expect(spans[2].toPlainText(), ' test');
    expect((spans[0] as TextSpan).style!.color, Colors.blue);
    expect((spans[1] as TextSpan).style!.color, Colors.red);
    expect((spans[2] as TextSpan).style!.color, Colors.blue);
  });
}
