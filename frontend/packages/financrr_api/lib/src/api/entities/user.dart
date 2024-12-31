import '../../../financrr_api.dart';

abstract class UserId extends EntityId<User> {}

abstract class User extends RestrrEntity<User, UserId> {
  String get username;
  String? get email;
  String? get displayName;
  DateTime get createdAt;
  bool get isAdmin;

  /// Returns the effective display name for the user.
  /// If the display name is not set, the username is returned instead.
  String get effectiveDisplayName;
}
