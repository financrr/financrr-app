import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/models/store.dart';
import '../../../utils/platform_utils.dart';
import '../models/authentication.state.dart';

final StateNotifierProvider<AuthenticationNotifier, AuthenticationState> authProvider =
    StateNotifierProvider((_) => AuthenticationNotifier());

extension ConsumerStateAuthExtension on ConsumerState {
  Restrr get api => ref.read(authProvider).api!;
}

class AuthenticationNotifier extends StateNotifier<AuthenticationState> {
  final Logger _log = Logger('AuthenticationNotifier');

  AuthenticationNotifier() : super(const AuthenticationState.initial());

  /// Attempts to recover a previous session.
  /// If the session is not recoverable, the user is logged out.
  Future<AuthenticationState> attemptRecovery() async {
    if (state.isAuthenticated) {
      return state;
    }
    final String? token = await StoreKey.sessionToken.readAsync();
    final String? hostUrl = await StoreKey.hostUrl.readAsync();
    if (token == null || hostUrl == null) {
      return await _authFailure();
    }
    try {
      final Restrr api = await _getRestrrBuilder(Uri.parse(hostUrl)).refresh(sessionToken: token);
      return _authSuccess(api);
    } on RestrrException catch (_) {}
    return await _authFailure();
  }

  Future<AuthenticationState> register(String username, String password, Uri hostUrl) async {
    try {
      final RestrrBuilder builder = _getRestrrBuilder(hostUrl);
      return _authSuccess(await builder.register(username: username, password: password, sessionInfo: await getSessionInfo()));
    } catch (e) {
      _log.warning('Could not register: $e');
      return await _authFailure();
    }
  }

  Future<AuthenticationState> login(String username, String password, Uri hostUrl) async {
    try {
      final RestrrBuilder builder = _getRestrrBuilder(hostUrl);
      return _authSuccess(await builder.login(username: username, password: password, sessionInfo: await getSessionInfo()));
    } catch (e) {
      _log.warning('Could not log in: $e');
      return await _authFailure();
    }
  }

  Future<SessionInfo> getSessionInfo() async {
    return SessionInfo(
        name: await PlatformUtils.getPlatformName(),
        description: await PlatformUtils.getPlatformDescription(),
        platform: kIsWeb ? SessionPlatform.web : SessionPlatform.current());
  }

  Future<void> logout() async {
    if (!state.isAuthenticated) {
      throw StateError('User is not logged in!');
    }
    try {
      await state.api!.deleteCurrentSession();
    } catch (_) {
      _log.warning('AuthenticationProvider: Could not delete current session!');
    }
    await _authFailure();
  }

  AuthenticationState _authSuccess(Restrr api) {
    StoreKey.sessionToken.write(api.session.token);
    return state = state.copyWith(api: api, status: AuthenticationStatus.authenticated);
  }

  Future<AuthenticationState> _authFailure() async {
    await StoreKey.sessionToken.delete();
    return state = state.copyWith(api: null, status: AuthenticationStatus.unauthenticated);
  }

  RestrrBuilder _getRestrrBuilder(Uri uri) => RestrrBuilder(uri: uri)..options = (RestrrOptions()..isWeb = kIsWeb);
}
