import 'package:flutter/services.dart';
import 'package:restrr/restrr.dart';

class MoneyInputFormatter extends TextInputFormatter {
  final String symbol;
  final int decimalPlaces;
  final String decimalSeparator;
  final String thousandSeparator;

  MoneyInputFormatter(
      {required this.symbol, required this.decimalPlaces, required this.decimalSeparator, required this.thousandSeparator});

  MoneyInputFormatter.fromCurrency(
      {required Currency currency, required this.decimalSeparator, required this.thousandSeparator})
      : symbol = currency.symbol,
        decimalPlaces = currency.decimalPlaces;

  UnformattedAmount amount = UnformattedAmount.zero;

  @override
  TextEditingValue formatEditUpdate(TextEditingValue oldValue, TextEditingValue newValue) {
    amount = UnformattedAmount.fromString(newValue.text);
    final String formatted = amount.format(decimalPlaces, decimalSeparator, thousandsSeparator: thousandSeparator);
    return newValue.copyWith(
      text: '$formatted$symbol',
      selection: TextSelection.collapsed(offset: formatted.length),
    );
  }
}
