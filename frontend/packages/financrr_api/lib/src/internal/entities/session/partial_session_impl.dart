import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';

import '../../../../financrr_api.dart';
import '../../utils/request_utils.dart';

class PartialSessionIdImpl extends IdImpl<PartialSession> implements PartialSessionId {
  const PartialSessionIdImpl({required super.api, required super.value});

  @override
  PartialSession? get() => api.sessionCache.get(this);

  @override
  Future<PartialSession> retrieve({forceRetrieve = false}) => RequestUtils.getOrRetrieveSingle(
      api: api,
      key: this,
      cacheView: api.sessionCache,
      compiledRoute: SessionRoutes.getById.compile(params: [value]),
      mapper: (json) => api.entityBuilder.buildPartialSession(json),
      forceRetrieve: forceRetrieve);
}

class PartialSessionImpl extends RestrrEntityImpl<PartialSession, PartialSessionId> implements PartialSession {
  @override
  final String name;
  @override
  final String? description;
  @override
  final SessionPlatform platform;
  @override
  final DateTime createdAt;
  @override
  final DateTime expiresAt;
  @override
  final User user;

  const PartialSessionImpl({
    required super.api,
    required super.id,
    required this.name,
    required this.description,
    required this.platform,
    required this.createdAt,
    required this.expiresAt,
    required this.user,
  });

  @override
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: SessionRoutes.deleteById.compile(params: [id.value]), api: api, key: id, cacheView: api.sessionCache);
}
