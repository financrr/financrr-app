import 'package:financrr_api/financrr_api.dart';

enum AuthenticationStatus { unknown, authenticated, unauthenticated }

class AuthenticationState {
  final Restrr? api;
  final AuthenticationStatus status;

  const AuthenticationState({
    required this.api,
    required this.status,
  });

  const AuthenticationState.initial()
      : api = null,
        status = AuthenticationStatus.unknown;

  bool get isAuthenticated => status == AuthenticationStatus.authenticated && api != null;
  bool get isAdmin => api?.selfUser.isAdmin ?? false;

  AuthenticationState copyWith({
    Restrr? api,
    User? user,
    AuthenticationStatus? status,
  }) {
    return AuthenticationState(
      api: api ?? this.api,
      status: status ?? this.status,
    );
  }
}
