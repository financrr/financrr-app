import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_button.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/paginated_wrapper.dart';
import '../../../shared/ui/cards/session_card.dart';
import '../../../utils/common_actions.dart';
import 'settings_page.dart';

class SessionSettingsPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: SettingsPage.pagePath, path: 'session');

  const SessionSettingsPage({super.key});

  @override
  ConsumerState<SessionSettingsPage> createState() => _SessionSettingsPageState();
}

class _SessionSettingsPageState extends ConsumerState<SessionSettingsPage> {
  final GlobalKey<PaginatedWrapperState<PartialSession>> _paginatedSessionKey = GlobalKey();
  late final Restrr _api = api;

  final ValueNotifier<int> _amount = ValueNotifier(0);

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Center(
          child: SizedBox(
            width: size.width / 1.1,
            child: RefreshIndicator(
              onRefresh: () async => _paginatedSessionKey.currentState?.reset(),
              child: ListView(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      FinancrrTextButton(
                        onPressed: () => _deleteAllSessions(),
                        label: L10nKey.commonDeleteAll.toText(),
                        icon: const Icon(Icons.delete_sweep_rounded),
                      ),
                      ValueListenableBuilder(
                          valueListenable: _amount,
                          builder: (context, value, child) {
                            // TODO: add plurals
                            return Text('${_amount.value} session(s)');
                          })
                    ],
                  ),
                  const SizedBox(height: 20),
                  PaginatedWrapper(
                    key: _paginatedSessionKey,
                    initialPageFunction: (forceRetrieve) => _api.retrieveAllSessions(limit: 10, forceRetrieve: forceRetrieve),
                    onError: (context, snap) {
                      if (snap.error is ServerException) {
                        CommonActions.logOut(this, ref);
                      }
                      return Text(snap.error.toString());
                    },
                    onSuccess: (context, snap) {
                      final PaginatedDataResult<PartialSession> sessions = snap.data!;
                      WidgetsBinding.instance.addPostFrameCallback((_) {
                        _amount.value = sessions.total;
                      });
                      return Column(
                        children: [
                          for (PartialSession s in sessions.items)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 10),
                              child: SessionCard(
                                  session: s,
                                  onDelete: s.id.value == _api.session.id.value
                                      ? () => CommonActions.logOut(this, ref)
                                      : () => _deleteSession(s)),
                            ),
                          if (sessions.nextPage != null)
                            FinancrrTextButton(
                              onPressed: () => sessions.nextPage!(_api),
                              label: L10nKey.commonLoadMore.toText(),
                            ),
                        ],
                      );
                    },
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }

  void _deleteSession(PartialSession session) async {
    try {
      await session.delete();
      if (!mounted) return;
      L10nKey.commonDeleteObjectSuccess.showSnack(context, namedArgs: {'object': session.name});
      if (session.id.value == _api.session.id.value) {
        CommonActions.logOut(this, ref);
      }
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }

  void _deleteAllSessions() async {
    try {
      await _api.deleteAllSessions();
      if (!mounted) return;
      L10nKey.sessionDeleteAllSuccess.showSnack(context);
      CommonActions.logOut(this, ref);
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
