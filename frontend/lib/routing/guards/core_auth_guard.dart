import 'dart:async';

import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';

import '../../modules/auth/providers/authentication.provider.dart';
import '../../modules/auth/views/server_config_page.dart';
import '../page_path.dart';
import 'guard.dart';

class CoreAuthGuard extends Guard {
  static final _log = Logger('CoreAuthGuard');

  @override
  FutureOr<PagePathBuilder?> redirect(ProviderRef<Object?> ref, GoRouterState state) async {
    if (!(await ref.read(authProvider.notifier).attemptRecovery()).isAuthenticated) {
      _log.info('CoreAuthGuard: User is not authenticated (anymore?), redirecting to ServerConfigPage');
      return ServerConfigPage.pagePath;
    }
    return null;
  }
}
