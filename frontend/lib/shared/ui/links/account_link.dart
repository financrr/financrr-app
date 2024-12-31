import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../modules/accounts/views/account_page.dart';
import '../../../modules/settings/providers/theme.provider.dart';

class AccountLink extends ConsumerWidget {
  final AccountId accountId;
  final String label;

  AccountLink({super.key, required Account account})
      : accountId = account.id,
        label = account.name;

  const AccountLink.fromData({super.key, required this.accountId, required this.label});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var theme = ref.watch(themeProvider);

    return GestureDetector(
      onTap: () => context.goPath(AccountPage.pagePath.build(params: {'accountId': accountId.value.toString()})),
      child: Text.rich(
          TextSpan(children: [
            TextSpan(text: label),
            WidgetSpan(
                child: Padding(
              padding: const EdgeInsets.only(left: 3),
              child: Icon(Icons.open_in_new_rounded, size: 16, color: theme.financrrExtension.primary),
            ))
          ]),
          style: theme.textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600, color: theme.financrrExtension.primary)),
    );
  }
}
