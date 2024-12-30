import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/settings/models/themes/app_theme_extension.model.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/modules/settings/ui/settings_category.dart';
import 'package:financrr_frontend/modules/settings/views/local_storage_settings_page.dart';
import 'package:financrr_frontend/modules/settings/views/l10n_settings_page.dart';
import 'package:financrr_frontend/modules/settings/views/session_settings_page.dart';
import 'package:financrr_frontend/modules/settings/views/template_overview_settings_page.dart';
import 'package:financrr_frontend/modules/settings/views/theme_settings_page.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/async_wrapper.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:restrr/restrr.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/custom_replacements/custom_circle_avatar.dart';
import '../../../utils/common_actions.dart';
import 'currency_settings_page.dart';
import 'log_settings_page.dart';

class SettingsPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder('/@me/settings');

  const SettingsPage({super.key});

  @override
  ConsumerState<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends ConsumerState<SettingsPage> {
  late final Restrr _api = api;

  List<SettingsCategory> _getCategories() => [
        SettingsCategory(title: L10nKey.settingsCategoryAccount, items: [
          SettingsItem(
            title: L10nKey.settingsItemCurrencies,
            iconData: Icons.currency_exchange_rounded,
            onTap: () => context.goPath(CurrencySettingsPage.pagePath.build()),
          ),
          SettingsItem(
            title: L10nKey.settingsItemTransactionTemplates,
            iconData: Icons.insert_page_break_outlined,
            onTap: () => context.goPath(TemplateOverviewSettingsPage.pagePath.build()),
          ),
          SettingsItem(
            title: L10nKey.settingsItemSessions,
            iconData: Icons.devices_rounded,
            onTap: () => context.goPath(SessionSettingsPage.pagePath.build()),
          ),
        ]),
        SettingsCategory(title: L10nKey.settingsCategoryApp, items: [
          SettingsItem(
            title: L10nKey.settingsItemAppearance,
            iconData: Icons.palette_outlined,
            onTap: () => context.goPath(ThemeSettingsPage.pagePath.build()),
          ),
          SettingsItem(
            title: L10nKey.settingsItemLanguage,
            iconData: Icons.language_rounded,
            onTap: () => context.goPath(L10nSettingsPage.pagePath.build()),
          ),
        ]),
        SettingsCategory(title: L10nKey.settingsCategoryDeveloper, items: [
          SettingsItem(
            title: L10nKey.settingsItemLocalStorage,
            iconData: Icons.sd_storage_outlined,
            onTap: () => context.goPath(LocalStorageSettingsPage.pagePath.build()),
          ),
          SettingsItem(
            title: L10nKey.settingsItemLogs,
            iconData: Icons.format_align_left_rounded,
            onTap: () => context.goPath(LogSettingsPage.pagePath.build()),
          ),
        ]),
        SettingsCategory(items: [
          SettingsItem(
            title: L10nKey.commonLogout,
            iconData: Icons.logout,
            onTap: () => CommonActions.logOut(this, ref),
          ),
        ]),
      ];

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider).getCurrent();

    buildVersionInfo() {
      return FutureWrapper(
          future: PackageInfo.fromPlatform(),
          onSuccess: (_, snap) {
            final PackageInfo info = snap.data!;
            return SizedBox(
              width: double.infinity,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  L10nKey.settingsFooter.toText(),
                  L10nKey.commonVersion.toText(namedArgs: {'version': '${info.version}+${info.buildNumber}'})
                ],
              ),
            );
          },
          onError: (_, __) => const SizedBox(),
          onLoading: (_, __) => const SizedBox());
    }

    buildDummyAccountItem() {
      return Container(
        padding: const EdgeInsets.all(10),
        decoration: BoxDecoration(
          border: Border.all(width: 3, color: theme.financrrExtension.surfaceVariant1),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Row(
          children: [
            FinancrrCircleAvatar.text(text: _api.selfUser.effectiveDisplayName, radius: 25),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(_api.selfUser.effectiveDisplayName, style: theme.themeData.textTheme.titleSmall),
                  const Text('placeholder@financrr.app'),
                ],
              ),
            ),
          ],
        ),
      );
    }

    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10),
        child: Center(
          child: SizedBox(
            width: size.width / 1.1,
            child: Scaffold(
                body: ListView(
              children: [
                buildDummyAccountItem(),
                for (SettingsCategory category in _getCategories()) category,
                Padding(
                  padding: const EdgeInsets.only(top: 20),
                  child: buildVersionInfo(),
                )
              ],
            )),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }
}
