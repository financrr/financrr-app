import '../../../financrr_api.dart';

abstract class AccountId extends EntityId<Account> {}

abstract class Account extends RestrrEntity<Account, AccountId> {
  @override
  AccountId get id;

  String get name;
  String? get description;
  String? get iban;
  UnformattedAmount get balance;
  UnformattedAmount get originalBalance;
  CurrencyId get currencyId;
  DateTime get createdAt;

  Future<bool> delete();

  Future<Account> update({String? name, String? description, String? iban, UnformattedAmount? originalBalance, Id? currencyId});

  Future<Paginated<Transaction>> retrieveAllTransactions({int page = 1, int limit = 25, bool forceRetrieve = false});
}
