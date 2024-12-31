import 'package:financrr_api/financrr_api.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

import '../../utils/request_utils.dart';
import 'currency_impl.dart';

class CustomCurrencyImpl extends CurrencyImpl implements CustomCurrency {
  @override
  final UserId? userId;

  const CustomCurrencyImpl({
    required super.api,
    required super.id,
    required super.name,
    required super.symbol,
    required super.isoCode,
    required super.decimalPlaces,
    required this.userId,
  });

  @override
  bool isCreatedBy(User user) => userId?.value == user.id.value;

  @override
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: CurrencyRoutes.deleteById.compile(params: [id.value]), api: api, key: id, cacheView: api.currencyCache);

  @override
  Future<Currency> update({String? name, String? symbol, String? isoCode, int? decimalPlaces}) async {
    if (name == null && symbol == null && isoCode == null && decimalPlaces == null) {
      throw ArgumentError('At least one field must be set');
    }
    final RestResponse<Currency> response = await api.requestHandler.apiRequest(
        route: CurrencyRoutes.patchById.compile(params: [id.value]),
        mapper: (json) => api.entityBuilder.buildCurrency(json),
        body: {
          if (name != null) 'name': name,
          if (symbol != null) 'symbol': symbol,
          if (isoCode != null) 'iso_code': isoCode,
          if (decimalPlaces != null) 'decimal_places': decimalPlaces,
        });
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }
}
