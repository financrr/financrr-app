import 'package:financrr_api/src/internal/entities/session/partial_session_impl.dart';

import '../../../../financrr_api.dart';

class SessionImpl extends PartialSessionImpl implements Session {
  @override
  final String token;

  const SessionImpl(
      {required super.api,
      required super.id,
      required super.name,
      required super.description,
      required super.platform,
      required super.createdAt,
      required super.expiresAt,
      required super.user,
      required this.token});
}
