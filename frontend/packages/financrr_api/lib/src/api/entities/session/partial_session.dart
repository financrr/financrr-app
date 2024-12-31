import '../../../../financrr_api.dart';

abstract class PartialSessionId extends EntityId<PartialSession> {}

abstract class PartialSession extends RestrrEntity<PartialSession, PartialSessionId> {
  @override
  PartialSessionId get id;

  String get name;
  String? get description;
  SessionPlatform get platform;
  DateTime get createdAt;
  DateTime get expiresAt;
  User get user;

  Future<bool> delete();
}
