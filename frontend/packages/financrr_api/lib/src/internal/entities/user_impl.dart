import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';

import '../../../financrr_api.dart';
import '../utils/request_utils.dart';

class UserIdImpl extends IdImpl<User> implements UserId {
  const UserIdImpl({required super.api, required super.value});

  @override
  User? get() => api.userCache.get(this);

  @override
  Future<User> retrieve({forceRetrieve = false}) {
    return RequestUtils.getOrRetrieveSingle(
        api: api,
        key: this,
        cacheView: api.userCache,
        compiledRoute: UserRoutes.getSelf.compile(),
        mapper: (json) => api.entityBuilder.buildUser(json),
        forceRetrieve: forceRetrieve);
  }
}

class UserImpl extends RestrrEntityImpl<User, UserId> implements User {
  @override
  final String username;
  @override
  final String? email;
  @override
  final String? displayName;
  @override
  final DateTime createdAt;
  @override
  final bool isAdmin;

  const UserImpl({
    required super.api,
    required super.id,
    required this.username,
    required this.email,
    required this.displayName,
    required this.createdAt,
    required this.isAdmin,
  });

  @override
  String get effectiveDisplayName => displayName ?? username;
}
