import 'package:financrr_api/src/api/requests/route.dart';

class StatusRoutes {
  const StatusRoutes._();

  static final Route health = Route.get('/status/health', isVersioned: false);
  static final Route coffee = Route.get('/status/coffee', isVersioned: false);
}

class SessionRoutes {
  const SessionRoutes._();

  static final Route getCurrent = Route.get('/session/current');
  static final Route deleteCurrent = Route.delete('/session/current');
  static final Route getById = Route.get('/session/{sessionId}');
  static final Route deleteById = Route.delete('/session/{sessionId}');
  static final Route getAll = Route.get('/session');
  static final Route deleteAll = Route.delete('/session');
  static final Route create = Route.post('/session');
  static final Route refresh = Route.patch('/session/refresh');
}

class UserRoutes {
  const UserRoutes._();

  static final Route getSelf = Route.get('/user/@me');
  static final Route create = Route.post('/user/register');
}

class AccountRoutes {
  const AccountRoutes._();

  static final Route getAll = Route.get('/account');
  static final Route getById = Route.get('/account/{accountId}');
  static final Route deleteById = Route.delete('/account/{accountId}');
  static final Route patchById = Route.patch('/account/{accountId}');
  static final Route getTransactions = Route.get('/account/{accountId}/transactions');
  static final Route create = Route.post('/account');
}

class CurrencyRoutes {
  const CurrencyRoutes._();

  static final Route getAll = Route.get('/currency');
  static final Route create = Route.post('/currency');
  static final Route getById = Route.get('/currency/{currencyId}');
  static final Route deleteById = Route.delete('/currency/{currencyId}');
  static final Route patchById = Route.patch('/currency/{currencyId}');
}

class TransactionRoutes {
  const TransactionRoutes._();

  static final Route getAll = Route.get('/transaction');
  static final Route getById = Route.get('/transaction/{transactionId}');
  static final Route deleteById = Route.delete('/transaction/{transactionId}');
  static final Route patchById = Route.patch('/transaction/{transactionId}');
  static final Route create = Route.post('/transaction');
  static final Route createFromTemplate = Route.post('/transaction/from-template');
}

class TransactionTemplateRoutes {
  const TransactionTemplateRoutes._();

  static final Route getAll = Route.get('/transaction/template');
  static final Route getById = Route.get('/transaction/template/{templateId}');
  static final Route deleteById = Route.delete('/transaction/template/{templateId}');
  static final Route patchById = Route.patch('/transaction/template/{templateId}');
  static final Route create = Route.post('/transaction/template');
}

class ScheduledTransactionTemplateRoutes {
  const ScheduledTransactionTemplateRoutes._();

  static final Route getAll = Route.get('/transaction/recurring'); // TODO: rename once backend is updated
  static final Route getById = Route.get('/transaction/recurring/{recurringId}'); // TODO: rename once backend is updated
  static final Route deleteById = Route.delete('/transaction/recurring/{recurringId}'); // TODO: rename once backend is updated
  static final Route patchById = Route.patch('/transaction/recurring/{recurringId}'); // TODO: rename once backend is updated
  static final Route create = Route.post('/transaction/recurring'); // TODO: rename once backend is updated
}
