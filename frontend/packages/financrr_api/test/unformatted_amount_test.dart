import 'package:financrr_api/financrr_api.dart';
import 'package:test/expect.dart';
import 'package:test/scaffolding.dart';

void main() {
  test('0.00', () {
    const UnformattedAmount amount = UnformattedAmount(0);
    expect(amount.rawAmount, 0);
    expect(amount.format(2, '.', thousandsSeparator: ','), '0.00');
  });

  test('0.50', () {
    const UnformattedAmount amount = UnformattedAmount(50);
    expect(amount.rawAmount, 50);
    expect(amount.format(2, '.', thousandsSeparator: ','), '0.50');
  });

  test('1.23', () {
    const UnformattedAmount amount = UnformattedAmount(123);
    expect(amount.rawAmount, 123);
    expect(amount.format(2, '.', thousandsSeparator: ','), '1.23');
  });

  test('1,234,567.89', () {
    const UnformattedAmount amount = UnformattedAmount(123456789);
    expect(amount.rawAmount, 123456789);
    expect(amount.format(2, '.', thousandsSeparator: ','), '1,234,567.89');
  });

  test('1,234,567.89€', () {
    const UnformattedAmount amount = UnformattedAmount(123456789);
    expect(amount.rawAmount, 123456789);
    expect(amount.format(2, '.', thousandsSeparator: ',', currencySymbol: '€'), '1,234,567.89€');
  });

  test('1,234,567.89US\$', () {
    const UnformattedAmount amount = UnformattedAmount(123456789);
    expect(amount.rawAmount, 123456789);
    expect(amount.format(2, '.', thousandsSeparator: ',', currencySymbol: 'US\$'), '1,234,567.89US\$');
  });
}
