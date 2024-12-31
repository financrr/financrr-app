import 'dart:convert';

import 'package:financrr_api/financrr_api.dart';
import 'package:financrr_api/src/internal/restrr_impl.dart';
import 'package:test/expect.dart';
import 'package:test/scaffolding.dart';

final Uri _validUri = Uri.parse('https://api-stage.financrr.app');

const String userJson = '''
{
  "id": 1,
  "username": "admin",
  "email": null,
  "created_at": "+002024-02-17T20:48:43.391176000Z",
  "is_admin": true
}
''';

const String currencyJson = '''
{
  "id": 1,
  "name": "US Dollar",
  "symbol": "\$",
  "iso_code": "USD",
  "decimal_places": 2,
  "user": 1
}
''';

const String sessionJson = '''
{
  "id": 1,
  "token": "abc",
  "name": "Test Session",
  "description": null,
  "platform": "web",
  "user": $userJson,
  "created_at": "+002024-02-17T20:48:43.391176000Z",
  "expires_at": "+002024-02-17T20:48:43.391176000Z"
}
''';

const String accountJson = '''
{
  "id": 1,
  "name": "Cash",
  "description": "Cash in hand",
  "iban": null,
  "balance": 0,
  "original_balance": 0,
  "currency_id": 1,
  "created_at": "+002024-02-17T20:48:43.391176000Z"
}
''';

const String transactionJson = '''
{
  "id": 1,
  "source_id": 1,
  "destination_id": null,
  "amount": 100,
  "currency_id": 1,
  "name": "Initial balance",
  "description": "Initial balance",
  "budget_id": null,
  "created_at": "+002024-02-17T20:48:43.391176000Z",
  "executed_at": "+002024-02-17T20:48:43.391176000Z"
}
''';

void main() {
  late RestrrImpl api;

  // TODO: mock the api instance instead of actually logging in

  setUp(() async {
    // log in, get api instance
    api = (await RestrrBuilder(uri: _validUri).login(
        username: 'admin',
        password: 'Financrr123',
        sessionInfo: SessionInfo(name: 'Test Session', platform: SessionPlatform.web))) as RestrrImpl;
  });

  group('[EntityBuilder] ', () {
    test('.buildCurrency', () {
      final Currency currency = api.entityBuilder.buildCurrency(jsonDecode(currencyJson));
      expect(currency.id.value, 1);
      expect(currency.name, 'US Dollar');
      expect(currency.symbol, '\$');
      expect(currency.isoCode, 'USD');
      expect(currency.decimalPlaces, 2);
    });

    test('.buildPartialSession', () {
      final Session session = api.entityBuilder.buildPartialSession(jsonDecode(sessionJson)) as Session;
      expect(session.id.value, 1);
      expect(session.token, 'abc');
      expect(session.name, 'Test Session');
      expect(session.description, null);
      expect(session.platform, SessionPlatform.web);
      expect(session.user.id.value, 1);
      expect(session.createdAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
      expect(session.expiresAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
    });

    test('.buildAccount', () {
      final Account account = api.entityBuilder.buildAccount(jsonDecode(accountJson));
      expect(account.id.value, 1);
      expect(account.name, 'Cash');
      expect(account.description, 'Cash in hand');
      expect(account.iban, null);
      expect(account.balance, 0);
      expect(account.originalBalance, 0);
      expect(account.currencyId.value, 1);
      expect(account.createdAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
    });

    test('.buildTransaction', () {
      final Transaction transaction = api.entityBuilder.buildTransaction(jsonDecode(transactionJson));
      expect(transaction.id.value, 1);
      expect(transaction.sourceId?.value, 1);
      expect(transaction.destinationId?.value, null);
      expect(transaction.amount, 100);
      expect(transaction.currencyId.value, 1);
      expect(transaction.name, 'Initial balance');
      expect(transaction.description, 'Initial balance');
      expect(transaction.budgetId, null);
      expect(transaction.createdAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
      expect(transaction.executedAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
    });

    test('.buildUser', () {
      final User user = api.entityBuilder.buildUser(jsonDecode(userJson));
      expect(user.id.value, 1);
      expect(user.username, 'admin');
      expect(user.email, null);
      expect(user.createdAt, DateTime.parse('+002024-02-17T20:48:43.391176000Z'));
      expect(user.isAdmin, true);
    });
  });
}
