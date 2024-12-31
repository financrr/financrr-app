import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';

import '../../../../financrr_api.dart';
import '../../requests/responses/rest_response.dart';
import '../../utils/request_utils.dart';

class TransactionIdImpl extends IdImpl<Transaction> implements TransactionId {
  const TransactionIdImpl({required super.api, required super.value});

  @override
  Transaction? get() => api.transactionCache.get(this);

  @override
  Future<Transaction> retrieve({forceRetrieve = false}) => RequestUtils.getOrRetrieveSingle(
      api: api,
      key: this,
      cacheView: api.transactionCache,
      compiledRoute: TransactionRoutes.getById.compile(params: [value]),
      mapper: (json) => api.entityBuilder.buildTransaction(json),
      forceRetrieve: forceRetrieve);
}

class TransactionImpl extends RestrrEntityImpl<Transaction, TransactionId> implements Transaction {
  @override
  final AccountId? sourceId;
  @override
  final AccountId? destinationId;
  @override
  final UnformattedAmount amount;
  @override
  final CurrencyId currencyId;
  @override
  final String name;
  @override
  final String? description;
  @override
  final EntityId? budgetId;
  @override
  final DateTime createdAt;
  @override
  final DateTime executedAt;

  const TransactionImpl({
    required super.api,
    required super.id,
    required this.sourceId,
    required this.destinationId,
    required this.amount,
    required this.currencyId,
    required this.name,
    required this.description,
    required this.budgetId,
    required this.createdAt,
    required this.executedAt,
  }) : assert(sourceId != null || destinationId != null);

  @override
  TransactionType getType(Account current) {
    if (sourceId != null && destinationId != null) {
      if (sourceId!.value == current.id.value) {
        return TransactionType.transferOut;
      }
      if (destinationId!.value == current.id.value) {
        return TransactionType.transferIn;
      }
    }
    if (sourceId != null) {
      return TransactionType.withdrawal;
    }
    return TransactionType.deposit;
  }

  @override
  UnformattedAmount getDisplayAmount(Account current) {
    final TransactionType type = getType(current);
    if (type == TransactionType.deposit || type == TransactionType.transferIn) {
      return amount;
    } else {
      return amount * -1;
    }
  }

  @override
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: TransactionRoutes.deleteById.compile(params: [id.value]),
      api: api,
      key: id,
      cacheView: api.transactionCache);

  @override
  Future<Transaction> update(
      {Id? sourceId,
      Id? destinationId,
      UnformattedAmount? amount,
      Id? currencyId,
      String? name,
      String? description,
      Id? budgetId,
      DateTime? executedAt}) async {
    if (sourceId == null &&
        destinationId == null &&
        amount == null &&
        currencyId == null &&
        name == null &&
        description == null &&
        budgetId == null &&
        executedAt == null) {
      throw ArgumentError('At least one field must be set');
    }
    final RestResponse<Transaction> response = await api.requestHandler.apiRequest(
        route: TransactionRoutes.patchById.compile(params: [id.value]),
        mapper: (json) => api.entityBuilder.buildTransaction(json),
        body: {
          if (sourceId != null) 'source_id': sourceId,
          if (destinationId != null) 'destination_id': destinationId,
          if (amount != null) 'amount': amount.rawAmount,
          if (currencyId != null) 'currency_id': currencyId,
          if (name != null) 'name': name,
          if (description != null) 'description': description,
          if (budgetId != null) 'budget_id': budgetId,
          if (executedAt != null) 'executed_at': executedAt.toUtc().toIso8601String(),
        });
    return response.data!;
  }

  @override
  Future<TransactionTemplate> createTemplate() async {
    return api.createTransactionTemplate(
        amount: amount,
        currencyId: currencyId.value,
        name: name,
        description: description,
        sourceId: sourceId?.value,
        destinationId: destinationId?.value,
        budgetId: budgetId?.value);
  }
}
