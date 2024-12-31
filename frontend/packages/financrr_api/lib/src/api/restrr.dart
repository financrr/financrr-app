import 'package:logging/logging.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

import '../../financrr_api.dart';

class RestrrOptions {
  bool isWeb = false;
  bool disableLogging = false;

  EntityCacheView<Currency, CurrencyId>? currencyCacheView;
  EntityCacheView<PartialSession, PartialSessionId>? sessionCacheView;
  EntityCacheView<Transaction, TransactionId>? transactionCacheView;
  EntityCacheView<TransactionTemplate, TransactionTemplateId>? transactionTemplateCacheView;
  EntityCacheView<ScheduledTransactionTemplate, ScheduledTransactionTemplateId>? scheduledTransactionTemplateCacheView;
  EntityCacheView<Account, AccountId>? accountCacheView;
  EntityCacheView<User, UserId>? userCacheView;
}

class RouteOptions {
  Uri? hostUri;
  int? apiVersion;
}

abstract class Restrr {
  static final Logger log = Logger('Restrr');

  RestrrOptions get options;
  RouteOptions get routeOptions;

  Session get session;

  /// The currently authenticated user.
  User get selfUser => session.user;

  /// Checks whether the specified URI is valid and points to a valid
  /// financrr API.
  static Future<ServerInfo> checkUri(Uri uri, {bool isWeb = false}) async {
    final RestResponse<ServerInfo> response = await RequestHandler.request(
      route: StatusRoutes.health.compile(),
      mapper: (json) => ServerInfo.fromJson(json),
      routeOptions: RouteOptions()..hostUri = uri,
    );
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }

  void on<T extends RestrrEvent>(Type type, void Function(T) func);

  /* Users */

  /// Retrieves the currently authenticated user.
  Future<User> retrieveSelf({bool forceRetrieve = false});

  /// Creates a new user.
  Future<User> createUser({required String username, required String password, String? displayName, String? email});

  /* Sessions */

  Future<PartialSession> retrieveCurrentSession({bool forceRetrieve = false});

  Future<PartialSession> retrieveSessionById(Id id, {bool forceRetrieve = false});

  Future<Paginated<PartialSession>> retrieveAllSessions({int page = 1, int limit = 25, bool forceRetrieve = false});

  /// Deletes the current session, effectively logging out the user.
  Future<bool> deleteCurrentSession();

  Future<bool> deleteAllSessions();

  /* Accounts */

  Future<Account> createAccount(
      {required String name,
      required UnformattedAmount originalBalance,
      required Id currencyId,
      String? description,
      String? iban});

  List<Account> getAccounts();

  Future<Account> retrieveAccountById(Id id, {bool forceRetrieve = false});

  Future<Paginated<Account>> retrieveAllAccounts({int page = 1, int limit = 25, bool forceRetrieve = false});

  /* Currencies */

  Future<Currency> createCurrency({required String name, required String symbol, required int decimalPlaces, String? isoCode});

  List<Currency> getCurrencies();

  Future<Currency> retrieveCurrencyById(Id id, {bool forceRetrieve = false});

  Future<Paginated<Currency>> retrieveAllCurrencies({int page = 1, int limit = 25, bool forceRetrieve = false});

  /* Transactions */

  Future<Transaction> createTransaction(
      {required UnformattedAmount amount,
      required Id currencyId,
      required DateTime executedAt,
      required String name,
      String? description,
      Id? sourceId,
      Id? destinationId,
      Id? budgetId});

  Future<Transaction> createTransactionFromTemplate({required Id templateId, required DateTime executedAt});

  Future<Transaction> retrieveTransactionById(Id id, {bool forceRetrieve = false});

  Future<Paginated<Transaction>> retrieveAllTransactions({int page = 1, int limit = 25, bool forceRetrieve = false});

  /* Transaction Templates */

  List<TransactionTemplate> getTransactionTemplates();

  Future<TransactionTemplate> createTransactionTemplate(
      {required UnformattedAmount amount,
      required Id currencyId,
      required String name,
      String? description,
      Id? sourceId,
      Id? destinationId,
      Id? budgetId});

  Future<TransactionTemplate> retrieveTransactionTemplateById(Id id, {bool forceRetrieve = false});

  Future<Paginated<TransactionTemplate>> retrieveAllTransactionTemplates(
      {int page = 1, int limit = 25, bool forceRetrieve = false});

  /* Scheduled Transaction Templates */

  List<ScheduledTransactionTemplate> getScheduledTransactionTemplates();

  Future<ScheduledTransactionTemplate> createScheduledTransactionTemplate(
      {required Id templateId, required ScheduleRule scheduleRule});

  Future<ScheduledTransactionTemplate> retrieveScheduledTransactionTemplateById(Id id, {bool forceRetrieve = false});

  Future<Paginated<ScheduledTransactionTemplate>> retrieveAllScheduledTransactionTemplates(
      {int page = 1, int limit = 25, bool forceRetrieve = false});
}
