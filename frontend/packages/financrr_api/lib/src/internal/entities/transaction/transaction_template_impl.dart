import 'package:financrr_api/financrr_api.dart';
import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';

import '../../requests/responses/rest_response.dart';
import '../../utils/request_utils.dart';

class TransactionTemplateIdImpl extends IdImpl<TransactionTemplate> implements TransactionTemplateId {
  const TransactionTemplateIdImpl({required super.api, required super.value});

  @override
  TransactionTemplate? get() => api.transactionTemplateCache.get(this);

  @override
  Future<TransactionTemplate> retrieve({forceRetrieve = false}) => RequestUtils.getOrRetrieveSingle(
      api: api,
      key: this,
      cacheView: api.transactionTemplateCache,
      compiledRoute: TransactionTemplateRoutes.getById.compile(params: [value]),
      mapper: (json) => api.entityBuilder.buildTransactionTemplate(json),
      forceRetrieve: forceRetrieve);
}

class TransactionTemplateImpl extends RestrrEntityImpl<TransactionTemplate, TransactionTemplateId>
    implements TransactionTemplate {
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

  const TransactionTemplateImpl({
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
  });

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
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: TransactionTemplateRoutes.deleteById.compile(params: [id.value]),
      api: api,
      key: id,
      cacheView: api.transactionTemplateCache);

  @override
  Future<TransactionTemplate> update(
      {Id? sourceId,
      Id? destinationId,
      UnformattedAmount? amount,
      Id? currencyId,
      String? name,
      String? description,
      Id? budgetId}) async {
    if (sourceId == null &&
        destinationId == null &&
        amount == null &&
        currencyId == null &&
        name == null &&
        description == null &&
        budgetId == null) {
      throw ArgumentError('At least one field must be set');
    }
    final RestResponse<TransactionTemplate> response = await api.requestHandler.apiRequest(
        route: TransactionTemplateRoutes.patchById.compile(params: [id.value]),
        mapper: (json) => api.entityBuilder.buildTransactionTemplate(json),
        body: {
          if (sourceId != null) 'source_id': sourceId,
          if (destinationId != null) 'destination_id': destinationId,
          if (amount != null) 'amount': amount.rawAmount,
          if (currencyId != null) 'currency_id': currencyId,
          if (name != null) 'name': name,
          if (description != null) 'description': description,
          if (budgetId != null) 'budget_id': budgetId,
        });
    return response.data!;
  }

  @override
  Future<ScheduledTransactionTemplate> schedule(ScheduleRule scheduleRule) async {
    return api.createScheduledTransactionTemplate(templateId: id.value, scheduleRule: scheduleRule);
  }

  @override
  Future<Transaction> createTransaction({required DateTime executedAt}) async {
    return api.createTransactionFromTemplate(templateId: id.value, executedAt: executedAt);
  }
}
