import '../../../../financrr_api.dart';

abstract class TransactionTemplateId extends EntityId<TransactionTemplate> {}

abstract class TransactionTemplate extends RestrrEntity<TransactionTemplate, TransactionTemplateId> {
  @override
  TransactionTemplateId get id;

  AccountId? get sourceId;
  AccountId? get destinationId;
  UnformattedAmount get amount;
  CurrencyId get currencyId;
  String get name;
  String? get description;
  EntityId? get budgetId;
  DateTime get createdAt;

  TransactionType getType(Account current);
  Future<bool> delete();

  Future<TransactionTemplate> update({
    Id? sourceId,
    Id? destinationId,
    UnformattedAmount? amount,
    Id? currencyId,
    String? name,
    String? description,
    Id? budgetId,
  });

  Future<ScheduledTransactionTemplate> schedule(ScheduleRule rule);

  Future<Transaction> createTransaction({required DateTime executedAt});
}
