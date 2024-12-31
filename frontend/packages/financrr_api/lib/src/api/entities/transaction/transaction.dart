import 'package:financrr_api/financrr_api.dart';

abstract class TransactionId extends EntityId<Transaction> {}

abstract class Transaction extends RestrrEntity<Transaction, TransactionId> {
  @override
  TransactionId get id;

  AccountId? get sourceId;
  AccountId? get destinationId;
  UnformattedAmount get amount;
  CurrencyId get currencyId;
  String get name;
  String? get description;
  EntityId? get budgetId;
  DateTime get createdAt;
  DateTime get executedAt;

  TransactionType getType(Account current);
  UnformattedAmount getDisplayAmount(Account current);

  Future<bool> delete();

  Future<Transaction> update({
    Id? sourceId,
    Id? destinationId,
    UnformattedAmount? amount,
    Id? currencyId,
    String? name,
    String? description,
    Id? budgetId,
    DateTime? executedAt,
  });

  Future<TransactionTemplate> createTemplate();
}
