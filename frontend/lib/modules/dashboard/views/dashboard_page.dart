import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/async_wrapper.dart';
import 'package:financrr_frontend/shared/ui/cards/account_card.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/cards/transaction_card.dart';
import '../../../shared/ui/notice_card.dart';
import '../../accounts/views/account_create_page.dart';
import '../../accounts/views/accounts_overview_page.dart';

class DashboardQuickAction {
  final L10nKey title;
  final IconData iconData;
  final Function() onTap;

  const DashboardQuickAction({required this.title, required this.iconData, required this.onTap});
}

class DashboardPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder('/@me/dashboard');

  const DashboardPage({super.key});

  @override
  ConsumerState<DashboardPage> createState() => _DashboardPageState();
}

class _DashboardPageState extends ConsumerState<DashboardPage> {
  late final Restrr _api = api;

  final StreamController<Paginated<Transaction>> _transactionStreamController = StreamController.broadcast();

  Future<Paginated<Transaction>> _fetchLatestTransactions({bool forceRetrieve = false}) async {
    final Paginated<Transaction> transactions = await _api.retrieveAllTransactions(limit: 10, forceRetrieve: forceRetrieve);
    _transactionStreamController.add(transactions);
    return transactions;
  }

  List<DashboardQuickAction> _getQuickActions() => [
        DashboardQuickAction(title: L10nKey.transactionCreate, iconData: Icons.add, onTap: () {}),
        DashboardQuickAction(title: L10nKey.transactionCreate, iconData: Icons.add, onTap: () {}),
        DashboardQuickAction(title: L10nKey.transactionCreate, iconData: Icons.add, onTap: () {}),
        DashboardQuickAction(title: L10nKey.transactionCreate, iconData: Icons.add, onTap: () {})
      ];

  @override
  void initState() {
    super.initState();
    _fetchLatestTransactions();
  }

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);

    buildQuickActionBar() {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 10),
            child: L10nKey.dashboardQuickActions.toText(style: theme.textTheme.titleMedium),
          ),
          SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: Row(mainAxisAlignment: MainAxisAlignment.end, children: [
              for (var action in _getQuickActions())
                SizedBox(
                  width: 100,
                  child: Column(
                    children: [
                      GestureDetector(
                        onTap: action.onTap,
                        child: Container(
                          padding: const EdgeInsets.all(10),
                          decoration: BoxDecoration(
                            border: Border.all(width: 3, color: theme.financrrExtension.surfaceVariant1),
                            shape: BoxShape.circle,
                          ),
                          child: Icon(action.iconData),
                        ),
                      ),
                      const SizedBox(height: 5),
                      action.title.toText(style: theme.textTheme.labelSmall, textAlign: TextAlign.center)
                    ],
                  ),
                )
            ]),
          ),
        ],
      );
    }

    buildTotalDummySection(Size size) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          L10nKey.dashboardTotal.toText(style: theme.textTheme.titleMedium),
          Text('0,00€', style: theme.textTheme.displaySmall?.copyWith(color: theme.financrrExtension.primary)),
          Container(
            width: size.width / 4,
            height: 5,
            decoration: BoxDecoration(
              color: theme.financrrExtension.surfaceVariant1,
              borderRadius: BorderRadius.circular(10),
            ),
          )
        ],
      );
    }

    buildTransactionSection() {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 10),
            child: L10nKey.dashboardTransactions.toText(style: theme.textTheme.titleMedium),
          ),
          StreamWrapper(
              stream: _transactionStreamController.stream,
              onError: (_, __) => const Text('Error'),
              onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
              onSuccess: (context, snap) {
                final List<Transaction> transactions = snap.data!.items;
                if (transactions.isEmpty) {
                  return Center(
                      child: NoticeCard(
                          title: L10nKey.transactionNoneFoundTitle.toString(),
                          description: L10nKey.transactionNoneFoundBody.toString()));
                }
                return Column(
                  children: [
                    for (Transaction t in transactions)
                      SizedBox(
                          width: double.infinity,
                          child: Padding(
                            padding: const EdgeInsets.only(bottom: 10),
                            // we can't determine an account here, so we pass null
                            child: TransactionCard(account: null, transaction: t),
                          ))
                  ],
                );
              })
        ],
      );
    }

    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10),
        child: Center(
          child: SizedBox(
              width: size.width / 1.1,
              child: RefreshIndicator(
                onRefresh: () => _fetchLatestTransactions(forceRetrieve: true),
                child: ListView(children: [
                  buildTotalDummySection(size),
                  const SizedBox(height: 20),
                  buildQuickActionBar(),
                  const SizedBox(height: 20),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      L10nKey.dashboardAccounts.toText(style: theme.textTheme.titleMedium),
                      PopupMenuButton(
                          icon: const Icon(Icons.more_horiz),
                          itemBuilder: (context) {
                            return [
                              PopupMenuItem(
                                  child: ListTile(
                                title: L10nKey.accountListManage.toText(),
                                leading: const Icon(Icons.manage_accounts_rounded),
                                onTap: () => context.goPath(AccountsOverviewPage.pagePath.build()),
                              )),
                              PopupMenuItem(
                                  child: ListTile(
                                title: L10nKey.accountCreate.toText(),
                                leading: const Icon(Icons.add),
                                onTap: () => context.goPath(AccountCreatePage.pagePath.build()),
                              ))
                            ];
                          })
                    ],
                  ),
                  for (Account a in _api.getAccounts())
                    Padding(
                      padding: const EdgeInsets.only(bottom: 10),
                      child: AccountCard(account: a),
                    ),
                  if (_api.getAccounts().isEmpty)
                    Center(
                        child: NoticeCard(
                            title: L10nKey.accountNoneFoundTitle.toString(),
                            description: L10nKey.accountNoneFoundBody.toString(),
                            onTap: () => context.goPath(AccountCreatePage.pagePath.build()))),
                  if (_api.getAccounts().isNotEmpty) buildTransactionSection()
                ]),
              )),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }
}
