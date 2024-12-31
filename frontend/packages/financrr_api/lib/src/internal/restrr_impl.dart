import 'package:financrr_api/src/internal/entities/account_impl.dart';
import 'package:financrr_api/src/internal/entities/currency/currency_impl.dart';
import 'package:financrr_api/src/internal/entities/transaction/transaction_impl.dart';
import 'package:financrr_api/src/internal/entities/user_impl.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';
import 'package:financrr_api/src/internal/utils/request_utils.dart';

import '../../financrr_api.dart';
import '../api/events/event_handler.dart';
import 'cache/entity_cache_view_impl.dart';
import 'entities/session/partial_session_impl.dart';
import 'entities/transaction/scheduled_transaction_template_impl.dart';
import 'entities/transaction/transaction_template_impl.dart';
import 'entity_builder.dart';

class RestrrImpl implements Restrr {
  @override
  RestrrOptions options = RestrrOptions();
  @override
  RouteOptions routeOptions = RouteOptions();

  late final RestrrEventHandler eventHandler;
  late final RequestHandler requestHandler = RequestHandler(this);
  late final EntityBuilder entityBuilder = EntityBuilder(api: this);

  /* Caches */

  late final EntityCacheView<Currency, CurrencyId> currencyCache = options.currencyCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<PartialSession, PartialSessionId> sessionCache = options.sessionCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<Account, AccountId> accountCache = options.accountCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<Transaction, TransactionId> transactionCache =
      options.transactionCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<TransactionTemplate, TransactionTemplateId> transactionTemplateCache =
      options.transactionTemplateCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<ScheduledTransactionTemplate, ScheduledTransactionTemplateId> scheduledTransactionTemplateCache =
      options.scheduledTransactionTemplateCacheView ?? EntityCacheViewImpl();
  late final EntityCacheView<User, UserId> userCache = options.userCacheView ?? EntityCacheViewImpl();

  RestrrImpl({required Map<Type, Function> eventMap}) : eventHandler = RestrrEventHandler(eventMap);

  @override
  late final Session session;

  @override
  User get selfUser => session.user;

  @override
  void on<T extends RestrrEvent>(Type type, void Function(T) func) => eventHandler.on(type, func);

  /* Users */

  @override
  Future<User> retrieveSelf({bool forceRetrieve = false}) async {
    return UserIdImpl(api: this, value: session.user.id.value).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<User> createUser({required String username, required String password, String? displayName, String? email}) async {
    final RestResponse<User> response = await requestHandler
        .apiRequest(route: UserRoutes.create.compile(), mapper: (json) => entityBuilder.buildUser(json), noAuth: true, body: {
      'username': username,
      'password': password,
      if (displayName != null) 'display_name': displayName,
      if (email != null) 'email': email,
    });
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }

  /* Sessions */

  @override
  Future<PartialSession> retrieveCurrentSession({bool forceRetrieve = false}) {
    return RequestUtils.getOrRetrieveSingle(
        api: this,
        key: session.id,
        cacheView: sessionCache,
        compiledRoute: SessionRoutes.getCurrent.compile(),
        mapper: (json) => entityBuilder.buildPartialSession(json),
        forceRetrieve: forceRetrieve);
  }

  @override
  Future<PartialSession> retrieveSessionById(Id id, {bool forceRetrieve = false}) {
    return PartialSessionIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<PartialSession>> retrieveAllSessions({int page = 1, int limit = 25, bool forceRetrieve = false}) {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: SessionRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildPartialSession(json),
        forceRetrieve: forceRetrieve);
  }

  @override
  Future<bool> deleteCurrentSession() async {
    final RestResponse<bool> response = await requestHandler.noResponseApiRequest(route: SessionRoutes.deleteCurrent.compile());
    if (response.hasData && response.data!) {
      eventHandler.fire(SessionDeleteEvent(api: this));
      return true;
    }
    return false;
  }

  @override
  Future<bool> deleteAllSessions() async {
    final RestResponse<bool> response = await requestHandler.noResponseApiRequest(route: SessionRoutes.deleteAll.compile());
    return response.hasData && response.data!;
  }

  /* Accounts */

  @override
  Future<Account> createAccount(
      {required String name,
      required UnformattedAmount originalBalance,
      required Id currencyId,
      String? description,
      String? iban}) async {
    final RestResponse<Account> response = await requestHandler
        .apiRequest(route: AccountRoutes.create.compile(), mapper: (json) => entityBuilder.buildAccount(json), body: {
      'name': name,
      'original_balance': originalBalance.rawAmount,
      'currency_id': currencyId,
      if (description != null) 'description': description,
      if (iban != null) 'iban': iban
    });
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }

  @override
  List<Account> getAccounts() => accountCache.getAll();

  @override
  Future<Account> retrieveAccountById(Id id, {bool forceRetrieve = false}) async {
    return AccountIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<Account>> retrieveAllAccounts({int page = 1, int limit = 25, bool forceRetrieve = false}) async {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: AccountRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildAccount(json),
        forceRetrieve: forceRetrieve);
  }

  /* Currencies */

  @override
  Future<Currency> createCurrency(
      {required String name, required String symbol, required int decimalPlaces, String? isoCode}) async {
    final RestResponse<Currency> response = await requestHandler.apiRequest(
        route: CurrencyRoutes.create.compile(),
        mapper: (json) => entityBuilder.buildCurrency(json),
        body: {'name': name, 'symbol': symbol, 'decimal_places': decimalPlaces, if (isoCode != null) 'iso_code': isoCode});
    if (response.hasError) {
      throw response.error!;
    }
    return response.data!;
  }

  @override
  List<Currency> getCurrencies() => currencyCache.getAll();

  @override
  Future<Currency> retrieveCurrencyById(Id id, {bool forceRetrieve = false}) async {
    return CurrencyIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<Currency>> retrieveAllCurrencies({int page = 1, int limit = 25, bool forceRetrieve = false}) async {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: CurrencyRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildCurrency(json),
        forceRetrieve: forceRetrieve);
  }

  /* Transactions */

  @override
  Future<Transaction> createTransaction(
      {required UnformattedAmount amount,
      required Id currencyId,
      required DateTime executedAt,
      required String name,
      String? description,
      Id? sourceId,
      Id? destinationId,
      Id? budgetId}) async {
    if (sourceId == null && destinationId == null) {
      throw ArgumentError('Either source or destination must be set!');
    }
    final RestResponse<Transaction> response = await requestHandler
        .apiRequest(route: TransactionRoutes.create.compile(), mapper: (json) => entityBuilder.buildTransaction(json), body: {
      'amount': amount.rawAmount,
      'currency_id': currencyId,
      'executed_at': executedAt.toUtc().toIso8601String(),
      'name': name,
      if (description != null) 'description': description,
      if (sourceId != null) 'source_id': sourceId,
      if (destinationId != null) 'destination_id': destinationId,
      if (budgetId != null) 'budget_id': budgetId
    });
    if (response.hasError) {
      throw response.error!;
    }
    // invalidate cache
    transactionCache.clear();
    return response.data!;
  }

  @override
  Future<Transaction> createTransactionFromTemplate(
      {required Id templateId, required DateTime executedAt}) async {
    final RestResponse<Transaction> response = await requestHandler.apiRequest(
        route: TransactionRoutes.createFromTemplate.compile(),
        mapper: (json) => entityBuilder.buildTransaction(json),
        body: {
          'template_id': templateId,
          'executed_at': executedAt.toUtc().toIso8601String(),
        });
    if (response.hasError) {
      throw response.error!;
    }
    // invalidate cache
    transactionCache.clear();
    return response.data!;
  }

  @override
  Future<Transaction> retrieveTransactionById(Id id, {bool forceRetrieve = false}) async {
    return TransactionIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<Transaction>> retrieveAllTransactions({int page = 1, int limit = 25, bool forceRetrieve = false}) async {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: TransactionRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildTransaction(json),
        forceRetrieve: forceRetrieve);
  }

  /* Transaction Templates */

  @override
  List<TransactionTemplate> getTransactionTemplates() => transactionTemplateCache.getAll();

  @override
  Future<TransactionTemplate> createTransactionTemplate(
      {required UnformattedAmount amount,
      required Id currencyId,
      required String name,
      String? description,
      Id? sourceId,
      Id? destinationId,
      Id? budgetId}) async {
    if (sourceId == null && destinationId == null) {
      throw ArgumentError('Either source or destination must be set!');
    }
    final RestResponse<TransactionTemplate> response = await requestHandler.apiRequest(
        route: TransactionTemplateRoutes.create.compile(),
        mapper: (json) => entityBuilder.buildTransactionTemplate(json),
        body: {
          'amount': amount.rawAmount,
          'currency_id': currencyId,
          'name': name,
          if (description != null) 'description': description,
          if (sourceId != null) 'source_id': sourceId,
          if (destinationId != null) 'destination_id': destinationId,
          if (budgetId != null) 'budget_id': budgetId
        });
    if (response.hasError) {
      throw response.error!;
    }
    // invalidate cache
    transactionTemplateCache.clear();
    return response.data!;
  }

  @override
  Future<TransactionTemplate> retrieveTransactionTemplateById(Id id, {bool forceRetrieve = false}) async {
    return TransactionTemplateIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<TransactionTemplate>> retrieveAllTransactionTemplates(
      {int page = 1, int limit = 25, bool forceRetrieve = false}) async {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: TransactionTemplateRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildTransactionTemplate(json),
        forceRetrieve: forceRetrieve);
  }

  /* Scheduled Transaction Templates */

  @override
  List<ScheduledTransactionTemplate> getScheduledTransactionTemplates() => scheduledTransactionTemplateCache.getAll();

  @override
  Future<ScheduledTransactionTemplate> createScheduledTransactionTemplate(
      {required Id templateId, required ScheduleRule scheduleRule}) async {
    final RestResponse<ScheduledTransactionTemplate> response = await requestHandler.apiRequest(
        route: ScheduledTransactionTemplateRoutes.create.compile(),
        mapper: (json) => entityBuilder.buildScheduledTransactionTemplate(json),
        body: {
          'template_id': templateId,
          'recurrence_rule': scheduleRule.toJson(),
        });
    if (response.hasError) {
      throw response.error!;
    }
    // invalidate cache
    scheduledTransactionTemplateCache.clear();
    return response.data!;
  }

  @override
  Future<ScheduledTransactionTemplate> retrieveScheduledTransactionTemplateById(Id id, {bool forceRetrieve = false}) async {
    return ScheduledTransactionTemplateIdImpl(api: this, value: id).retrieve(forceRetrieve: forceRetrieve);
  }

  @override
  Future<Paginated<ScheduledTransactionTemplate>> retrieveAllScheduledTransactionTemplates(
      {int page = 1, int limit = 25, bool forceRetrieve = false}) async {
    return RequestUtils.getOrRetrievePage(
        api: this,
        compiledRoute: ScheduledTransactionTemplateRoutes.getAll.compile(),
        page: page,
        limit: limit,
        mapper: (json) => entityBuilder.buildScheduledTransactionTemplate(json),
        forceRetrieve: forceRetrieve);
  }
}
