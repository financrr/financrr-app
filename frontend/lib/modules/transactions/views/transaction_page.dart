import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/modules/transactions/views/transaction_edit_page.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/links/account_link.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:restrr/restrr.dart';

import '../../../shared/models/store.dart';
import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../../../routing/page_path.dart';
import '../../../shared/ui/async_wrapper.dart';
import '../../accounts/views/account_page.dart';
import '../../settings/providers/l10n.provider.dart';

class TransactionPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath =
      PagePathBuilder.child(parent: AccountPage.pagePath, path: 'transactions/:transactionId');

  final String? accountId;
  final String? transactionId;

  const TransactionPage({super.key, required this.accountId, required this.transactionId});

  @override
  ConsumerState<TransactionPage> createState() => TransactionPageState();
}

class TransactionPageState extends ConsumerState<TransactionPage> {
  final StreamController<Account> _accountStreamController = StreamController.broadcast();
  final StreamController<Transaction> _transactionStreamController = StreamController.broadcast();

  late final Restrr _api = api;

  Future<Account?> _fetchAccount({bool forceRetrieve = false}) async {
    return _accountStreamController.fetchData(
        widget.accountId, (id) => _api.retrieveAccountById(id, forceRetrieve: forceRetrieve));
  }

  Future<Transaction?> _fetchTransaction({bool forceRetrieve = false}) async {
    return _transactionStreamController.fetchData(
        widget.transactionId, (id) => _api.retrieveTransactionById(id, forceRetrieve: forceRetrieve));
  }

  @override
  void initState() {
    super.initState();
    _fetchAccount().then((_) {
      Future.delayed(const Duration(milliseconds: 100), () => _fetchTransaction());
    });
  }

  @override
  void dispose() {
    _accountStreamController.close();
    _transactionStreamController.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);
    var l10n = ref.watch(l10nProvider);

    buildTableRow(L10nKey label, dynamic value) {
      return TableRow(children: [
        Padding(
          padding: const EdgeInsets.all(10),
          child: label.toText(
            style: theme.textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600),
          ),
        ),
        Padding(
          padding: const EdgeInsets.all(10),
          child: value is Account ? AccountLink(account: value) : Text(value),
        ),
      ]);
    }

    buildVerticalLayout(Account account, Transaction transaction, Size size) {
      final UnformattedAmount displayAmount = transaction.getDisplayAmount(account);
      final String amountStr = displayAmount.formatWithCurrency(account.currencyId.get()!, l10n.decimalSeparator,
          thousandsSeparator: l10n.thousandSeparator);
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Align(
          alignment: Alignment.topCenter,
          child: SizedBox(
            width: size.width / 1.1,
            child: RefreshIndicator(
                onRefresh: () async {
                  await _fetchAccount(forceRetrieve: true);
                  await _fetchTransaction(forceRetrieve: true);
                },
                child: ListView(
                  children: [
                    Column(
                      children: [
                        Text(amountStr,
                            style: theme.textTheme.titleLarge?.copyWith(
                                color: displayAmount.rawAmount > 0
                                    ? theme.themeData.primaryColor
                                    : theme.themeData.colorScheme.error)),
                        Text(transaction.description ?? StoreKey.dateTimeFormat.readSync()!.format(transaction.executedAt)),
                      ],
                    ),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        IconButton(
                            tooltip: 'Delete Transaction',
                            onPressed: () => _deleteTransaction(transaction),
                            icon: const Icon(Icons.delete_rounded, size: 17)),
                        IconButton(
                            tooltip: 'Edit Transaction',
                            onPressed: () => context.goPath(TransactionEditPage.pagePath.build(params: {
                                  'accountId': account.id.value.toString(),
                                  'transactionId': transaction.id.value.toString()
                                })),
                            icon: const Icon(Icons.create_rounded, size: 17))
                      ],
                    ),
                    Padding(
                      padding: const EdgeInsets.only(top: 10),
                      child: Table(
                        border: TableBorder.all(
                            borderRadius: BorderRadius.circular(10), color: theme.financrrExtension.surfaceVariant1, width: 3),
                        children: [
                          buildTableRow(L10nKey.transactionPropertiesType, transaction.getType(account).name),
                          buildTableRow(L10nKey.transactionPropertiesAmount, amountStr),
                          buildTableRow(L10nKey.transactionPropertiesName, transaction.name),
                          buildTableRow(L10nKey.transactionPropertiesDescription,
                              transaction.description ?? L10nKey.commonNotAvailable.toString()),
                          buildTableRow(L10nKey.transactionPropertiesFrom,
                              transaction.sourceId?.get() ?? L10nKey.commonNotAvailable.toString()),
                          buildTableRow(L10nKey.transactionPropertiesTo,
                              transaction.destinationId?.get() ?? L10nKey.commonNotAvailable.toString()),
                          buildTableRow(L10nKey.transactionPropertiesExecutedAt,
                              StoreKey.dateTimeFormat.readSync()!.format(transaction.executedAt)),
                          buildTableRow(L10nKey.transactionPropertiesCreatedAt,
                              StoreKey.dateTimeFormat.readSync()!.format(transaction.createdAt)),
                        ],
                      ),
                    )
                  ],
                )),
          ),
        ),
      );
    }

    handleTransactionStream(Account account, Size size) {
      return StreamWrapper(
          stream: _transactionStreamController.stream,
          onSuccess: (_, snap) => buildVerticalLayout(account, snap.data!, size),
          onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
          onError: (_, __) => L10nKey.transactionNotFound.toText());
    }

    handleAccountStream(Size size) {
      return StreamWrapper(
          stream: _accountStreamController.stream,
          onSuccess: (_, snap) => handleTransactionStream(snap.data!, size),
          onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
          onError: (_, __) => L10nKey.accountNotFound.toText());
    }

    return AdaptiveScaffold(verticalBuilder: (_, __, size) => handleAccountStream(size));
  }

  void _deleteTransaction(Transaction transaction) async {
    try {
      await transaction.delete();
      if (!mounted) return;
      context.pop();
      L10nKey.commonDeleteObjectSuccess.showSnack(context, namedArgs: {'object': transaction.description ?? 'transaction'});
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
