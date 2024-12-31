import '../../../../financrr_api.dart';

abstract class CurrencyId extends EntityId<Currency> {}

abstract class Currency extends RestrrEntity<Currency, CurrencyId> {
  @override
  CurrencyId get id;

  String get name;
  String get symbol;
  int get decimalPlaces;
  String? get isoCode;
}
