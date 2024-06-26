import 'package:flutter/material.dart';

class TextUtils {
  const TextUtils._();

  static String? formatIBAN(String? iban) {
    if (iban == null || iban.length != 22) return null;
    return '${iban.substring(0, 4)} ${iban.substring(4, 8)} ${iban.substring(8, 12)} ${iban.substring(12, 16)} ${iban.substring(16, 20)} ${iban.substring(20)}';
  }

  /// Converts a template string to a list of [InlineSpan]s with the localized text and styles.
  /// The template string should contain placeholders in the form of `{key}` where `key` is the key of the localized string.
  static List<InlineSpan> richFormatL10nText(String template, String localized,
      {required Map<String, TextStyle Function(TextStyle)> namedStyles, TextStyle? style, Map<String, String>? namedArgs}) {
    if (namedStyles.isEmpty) {
      return [TextSpan(text: localized, style: style)];
    }
    String remaining = template;
    final List<(String, TextStyle?)> styles = [];

    for (MapEntry<String, TextStyle Function(TextStyle)> entry in namedStyles.entries) {
      String key = namedArgs?[entry.key] ?? entry.key;
      int start = remaining.indexOf('{${entry.key}}');
      int end = start + entry.key.length + 2;
      // add part before found key
      styles.add((remaining.substring(0, start), style));
      // add found key
      styles.add((key, entry.value.call(style ?? const TextStyle())));
      // shrink remaining string
      remaining = remaining.replaceRange(0, end, '');
    }
    if (remaining.isNotEmpty) {
      styles.add((remaining, style));
    }

    return [for ((String, TextStyle?) pair in styles) TextSpan(text: pair.$1, style: pair.$2)];
  }
}
