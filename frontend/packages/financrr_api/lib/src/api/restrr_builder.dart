import '../../financrr_api.dart';
import '../internal/requests/responses/rest_response.dart';
import '../internal/restrr_impl.dart';
import '../internal/utils/request_utils.dart';

class SessionInfo {
  final String name;
  final String? description;
  final SessionPlatform platform;

  const SessionInfo({required this.name, this.description, required this.platform});

  Map<String, dynamic> toJson() {
    return {
      'name': name,
      if (description != null) 'description': description,
      'platform': platform.name,
    };
  }
}

/// A builder for creating a new [Restrr] instance.
/// The [Restrr] instance is created by calling [create].
class RestrrBuilder {
  final Map<Type, Function> _eventMap = {};

  final Uri uri;

  RestrrOptions options = RestrrOptions();

  RestrrBuilder({required this.uri});

  RestrrBuilder on<T extends RestrrEvent>(Type type, void Function(T) func) {
    _eventMap[type] = func;
    return this;
  }

  Future<Restrr> login({required String username, required String password, required SessionInfo sessionInfo}) async {
    return _handleAuthProcess(authFunction: (apiImpl) {
      return apiImpl.requestHandler.apiRequest(
          route: SessionRoutes.create.compile(),
          body: {'username': username, 'password': password, ...sessionInfo.toJson()},
          noAuth: true,
          mapper: (json) => apiImpl.entityBuilder.buildPartialSession(json));
    });
  }

  Future<Restrr> register(
      {required String username,
      required String password,
      required SessionInfo sessionInfo,
      String? displayName,
      String? email}) async {
    return _handleAuthProcess(authFunction: (apiImpl) async {
      // register user first
      final User user = await apiImpl.createUser(username: username, password: password, email: email);
      return apiImpl.requestHandler.apiRequest(
          route: SessionRoutes.create.compile(),
          body: {'username': user.username, 'password': password, ...sessionInfo.toJson()},
          noAuth: true,
          mapper: (json) => apiImpl.entityBuilder.buildPartialSession(json));
    });
  }

  Future<Restrr> refresh({required String sessionToken}) async {
    return _handleAuthProcess(authFunction: (apiImpl) {
      return apiImpl.requestHandler.apiRequest(
          route: SessionRoutes.refresh.compile(),
          bearerTokenOverride: sessionToken,
          mapper: (json) => apiImpl.entityBuilder.buildPartialSession(json));
    });
  }

  Future<Restrr> _handleAuthProcess({required Future<RestResponse<PartialSession>> Function(RestrrImpl) authFunction}) async {
    // check if the URI is valid and the API is healthy
    final ServerInfo statusResponse = await Restrr.checkUri(uri, isWeb: options.isWeb);
    Restrr.log.config('Host: $uri, API v${statusResponse.apiVersion}');
    // build api instance
    final RestrrImpl apiImpl = RestrrImpl(eventMap: _eventMap)
      ..options = options
      ..routeOptions = (RouteOptions()
        ..hostUri = uri
        ..apiVersion = statusResponse.apiVersion);
    // call auth function
    final RestResponse<PartialSession> response = await authFunction(apiImpl);
    if (response.hasError) {
      throw response.error!;
    }
    if (response.data is! Session) {
      throw ArgumentError('The response data is not a session');
    }
    apiImpl.session = response.data! as Session;

    // Retrieve all accounts, currencies, templates & scheduled templates to make them available in the cache
    _cacheEntities(apiImpl, (api) => api.retrieveAllAccounts(limit: 50), 'accounts');
    _cacheEntities(apiImpl, (api) => api.retrieveAllCurrencies(limit: 50), 'currencies');
    _cacheEntities(apiImpl, (api) => api.retrieveAllTransactionTemplates(limit: 50), 'transaction templates');
    _cacheEntities(
        apiImpl, (api) => api.retrieveAllScheduledTransactionTemplates(limit: 50), 'scheduled transaction templates');

    apiImpl.eventHandler.fire(ReadyEvent(api: apiImpl));
    return apiImpl;
  }

  Future _cacheEntities<E extends RestrrEntity<E, ID>, ID extends EntityId<E>>(
      RestrrImpl apiImpl, Future<Paginated<E>> Function(RestrrImpl) cacheFunction, String term) async {
    try {
      final List<E> currencies = await RequestUtils.fetchAllPaginated<E, ID>(apiImpl, await cacheFunction.call(apiImpl));
      Restrr.log.info('Cached ${currencies.length} $term');
    } catch (e) {
      Restrr.log.warning('Failed to cache $term: $e');
    }
  }
}
