import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/async_wrapper.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:restrr/restrr.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/custom_replacements/custom_text_button.dart';
import '../../../shared/ui/notice_card.dart';
import '../../../shared/ui/paginated_wrapper.dart';
import '../../../shared/ui/cards/transaction_card.dart';
import '../../settings/providers/l10n.provider.dart';
import '../../transactions/views/transaction_create_page.dart';
import 'account_edit_page.dart';
import 'accounts_overview_page.dart';

class AccountPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: AccountsOverviewPage.pagePath, path: ':accountId');

  final String? accountId;

  const AccountPage({super.key, required this.accountId});

  @override
  ConsumerState<AccountPage> createState() => _AccountPageState();
}

class _AccountPageState extends ConsumerState<AccountPage> {
  final StreamController<Account> _accountStreamController = StreamController.broadcast();
  final GlobalKey<PaginatedWrapperState> _transactionPaginatedKey = GlobalKey();
  late final Restrr _api = api;

  Future<Account?> _fetchAccount({bool forceRetrieve = false}) async {
    return _accountStreamController.fetchData(
        widget.accountId, (id) => _api.retrieveAccountById(id, forceRetrieve: forceRetrieve));
  }

  @override
  void initState() {
    super.initState();
    _fetchAccount();
  }

  @override
  void dispose() {
    _accountStreamController.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);
    var l10n = ref.watch(l10nProvider);

    buildTransactionSection(Account account) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          L10nKey.dashboardTransactions.toText(style: theme.textTheme.titleMedium),
          Padding(
            padding: const EdgeInsets.only(top: 10),
            child: PaginatedWrapper(
              key: _transactionPaginatedKey,
              initialPageFunction: (forceRetrieve) => account.retrieveAllTransactions(forceRetrieve: forceRetrieve),
              onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
              onSuccess: (context, snap) {
                final List<Transaction> transactions = snap.data!.items;
                if (transactions.isEmpty) {
                  return Center(
                      child: NoticeCard(
                    title: L10nKey.transactionNoneFoundTitle.toString(),
                    description: L10nKey.transactionNoneFoundBody.toString(),
                    onTap: () => context
                        .goPath(TransactionCreatePage.pagePath.build(params: {'accountId': account.id.value.toString()})),
                  ));
                }
                return Column(
                  children: [
                    for (var transaction in transactions)
                      Padding(
                        padding: const EdgeInsets.only(bottom: 10),
                        child: TransactionCard(account: account, transaction: transaction),
                      )
                  ],
                );
              },
            ),
          )
        ],
      );
    }

    buildVerticalLayout(Account account, Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Align(
          alignment: Alignment.topCenter,
          child: SizedBox(
            width: size.width / 1.1,
            child: RefreshIndicator(
              onRefresh: () async {
                await _fetchAccount(forceRetrieve: true);
                await _transactionPaginatedKey.currentState?.reset();
              },
              child: ListView(
                children: [
                  Column(
                    children: [
                      Text(
                          account.balance.formatWithCurrency(account.currencyId.get()!, l10n.decimalSeparator,
                              thousandsSeparator: l10n.thousandSeparator),
                          style: theme.textTheme.titleLarge?.copyWith(
                              color: account.balance.rawAmount < 0
                                  ? theme.financrrExtension.error
                                  : theme.financrrExtension.primary)),
                      Text(account.name),
                    ],
                  ),
                  const SizedBox(height: 20),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      FinancrrTextButton(
                          onPressed: () => context
                              .goPath(TransactionCreatePage.pagePath.build(params: {'accountId': account.id.value.toString()})),
                          icon: const Icon(Icons.add, size: 17),
                          label: L10nKey.transactionCreate.toText()),
                      const Spacer(),
                      IconButton(
                          tooltip: L10nKey.accountDelete.toString(),
                          onPressed: () => _deleteAccount(account),
                          icon: const Icon(Icons.delete_rounded, size: 17)),
                      IconButton(
                          tooltip: L10nKey.accountEdit.toString(),
                          onPressed: () => context
                              .goPath(AccountEditPage.pagePath.build(params: {'accountId': account.id.value.toString()})),
                          icon: const Icon(Icons.create, size: 17))
                    ],
                  ),
                  Padding(
                    padding: const EdgeInsets.only(top: 10),
                    child: buildTransactionSection(account),
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }

    handleAccountStream(Size size) {
      return StreamWrapper(
          stream: _accountStreamController.stream,
          onSuccess: (_, snap) => buildVerticalLayout(snap.data!, size),
          onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
          onError: (_, __) => L10nKey.accountNotFound.toText());
    }

    return AdaptiveScaffold(verticalBuilder: (_, __, size) => handleAccountStream(size));
  }

  void _deleteAccount(Account account) async {
    try {
      await account.delete();
      if (!mounted) return;
      L10nKey.commonDeleteObjectSuccess.showSnack(context, namedArgs: {'object': account.name});
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
