import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';

import '../../../financrr_api.dart';
import '../requests/responses/rest_response.dart';
import '../utils/request_utils.dart';

class AccountIdImpl extends IdImpl<Account> implements AccountId {
  const AccountIdImpl({required super.api, required super.value});

  @override
  Account? get() => api.accountCache.get(this);

  @override
  Future<Account> retrieve({forceRetrieve = false}) {
    return RequestUtils.getOrRetrieveSingle(
        api: api,
        key: this,
        cacheView: api.accountCache,
        compiledRoute: AccountRoutes.getById.compile(params: [value]),
        mapper: (json) => api.entityBuilder.buildAccount(json),
        forceRetrieve: forceRetrieve);
  }
}

class AccountImpl extends RestrrEntityImpl<Account, AccountId> implements Account {
  @override
  final String name;
  @override
  final String? description;
  @override
  final String? iban;
  @override
  final UnformattedAmount balance;
  @override
  final UnformattedAmount originalBalance;
  @override
  final CurrencyId currencyId;
  @override
  final DateTime createdAt;

  const AccountImpl({
    required super.api,
    required super.id,
    required this.name,
    required this.description,
    required this.iban,
    required this.balance,
    required this.originalBalance,
    required this.currencyId,
    required this.createdAt,
  });

  @override
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: AccountRoutes.deleteById.compile(params: [id.value]), api: api, key: id, cacheView: api.accountCache);

  @override
  Future<Account> update(
      {String? name, String? description, String? iban, UnformattedAmount? originalBalance, Id? currencyId}) async {
    if (name == null && description == null && iban == null && originalBalance == null && currencyId == null) {
      throw ArgumentError('At least one field must be set');
    }
    final RestResponse<Account> response = await api.requestHandler.apiRequest(
        route: AccountRoutes.patchById.compile(params: [id.value]),
        mapper: (json) => api.entityBuilder.buildAccount(json),
        body: {
          if (name != null) 'name': name,
          if (description != null) 'description': description,
          if (iban != null) 'iban': iban,
          if (originalBalance != null) 'original_balance': originalBalance.rawAmount,
          if (currencyId != null) 'currency_id': currencyId,
        });
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }

  @override
  Future<Paginated<Transaction>> retrieveAllTransactions({int page = 1, int limit = 25, bool forceRetrieve = false}) {
    return RequestUtils.getOrRetrievePage(
        api: api,
        compiledRoute: AccountRoutes.getTransactions.compile(params: [id.value]),
        page: page,
        limit: limit,
        mapper: (json) => api.entityBuilder.buildTransaction(json),
        forceRetrieve: forceRetrieve);
  }
}
