import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/transactions/views/transaction_page.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_button.dart';
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
import '../../../shared/ui/cards/transaction_card.dart';
import '../../../utils/form_fields.dart';
import '../../../utils/formatter/money_input_formatter.dart';
import '../../settings/providers/l10n.provider.dart';

class TransactionEditPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: TransactionPage.pagePath, path: 'edit');

  final String? accountId;
  final String? transactionId;

  const TransactionEditPage({super.key, required this.accountId, required this.transactionId});

  @override
  ConsumerState<TransactionEditPage> createState() => TransactionEditPageState();
}

class TransactionEditPageState extends ConsumerState<TransactionEditPage> {
  final StreamController<Account> _accountStreamController = StreamController.broadcast();
  final StreamController<Transaction> _transactionStreamController = StreamController.broadcast();
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = api;

  late final TextEditingController _nameController;
  late final TextEditingController _amountController;
  late final TextEditingController _descriptionController;
  late final TextEditingController _executedAtController;

  bool _isValid = false;
  UnformattedAmount _amount = UnformattedAmount.zero;
  TransactionType _type = TransactionType.deposit;
  DateTime _executedAt = DateTime.now();

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
    _fetchAccount().then((account) {
      Future.delayed(
          const Duration(milliseconds: 100),
          () => _fetchTransaction().then((transaction) {
                if (transaction != null) {
                  _nameController = TextEditingController(text: transaction.name);
                  _amountController = TextEditingController();
                  _descriptionController = TextEditingController(text: transaction.description);
                  _executedAtController =
                      TextEditingController(text: StoreKey.dateTimeFormat.readSync()!.format(transaction.executedAt));
                  _isValid = _formKey.currentState?.validate() ?? false;
                  _type = transaction.getType(account!);
                  _executedAt = transaction.executedAt;
                }
              }));
    });
  }

  @override
  void dispose() {
    _accountStreamController.close();
    _transactionStreamController.close();

    _nameController.dispose();
    _amountController.dispose();
    _descriptionController.dispose();
    _executedAtController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var l10n = ref.watch(l10nProvider);

    buildVerticalLayout(Account account, Transaction transaction, Size size) {
      final MoneyInputFormatter moneyFormatter = MoneyInputFormatter.fromCurrency(
        currency: account.currencyId.get() ?? _api.getCurrencies().first,
        decimalSeparator: l10n.decimalSeparator,
        thousandSeparator: l10n.thousandSeparator,
      );
      if (_amountController.text.isEmpty) {
        _amountController.text = moneyFormatter
            .formatEditUpdate(const TextEditingValue(text: ''), TextEditingValue(text: transaction.amount.toString()))
            .text;
      }

      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Align(
          alignment: Alignment.topCenter,
          child: SizedBox(
            width: size.width / 1.1,
            child: SingleChildScrollView(
                child: Form(
              key: _formKey,
              onChanged: () => setState(() => _isValid = _formKey.currentState?.validate() ?? false),
              child: Column(
                children: [
                  TransactionCard.fromData(
                    id: 0,
                    amount: _amount,
                    account: account,
                    name: _nameController.text,
                    description: _descriptionController.text.isEmpty ? null : _descriptionController.text,
                    type: _type,
                    createdAt: DateTime.now(),
                    executedAt: _executedAt,
                    interactive: false,
                  ),
                  const SizedBox(height: 20),
                  ...FormFields.transaction(
                    this,
                    currentAccount: account,
                    nameController: _nameController,
                    amountController: _amountController,
                    descriptionController: _descriptionController,
                    executedAtController: _executedAtController,
                    selectedType: _type,
                    executedAt: _executedAt,
                    moneyInputFormatter: moneyFormatter,
                    onAmountChanged: (amount) => setState(() => _amount = amount),
                    onSelectionChanged: (types) => setState(() => _type = types.first),
                    onExecutedAtChanged: (date) => setState(() => _executedAt = date),
                  ),
                  const SizedBox(height: 20),
                  FinancrrButton(
                    onPressed: _isValid ? () => _editTransaction(account, transaction, _type) : null,
                    text: L10nKey.transactionEdit.toString(),
                  ),
                ],
              ),
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

  Future<void> _editTransaction(Account account, Transaction transaction, TransactionType type, {Account? secondary}) async {
    if (!_isValid) return;
    final (Id?, Id?) sourceAndDest = switch (_type) {
      TransactionType.deposit => (null, account.id.value),
      TransactionType.withdrawal => (account.id.value, null),
      TransactionType.transferOut => (account.id.value, secondary!.id.value),
      TransactionType.transferIn => (secondary!.id.value, account.id.value),
    };
    try {
      await transaction.update(
          sourceId: sourceAndDest.$1,
          destinationId: sourceAndDest.$2,
          amount: _amount,
          name: _nameController.text,
          description: _descriptionController.text.isEmpty ? null : _descriptionController.text,
          executedAt: _executedAt,
          currencyId: account.currencyId.value);
      if (!mounted) return;
      L10nKey.commonEditObjectSuccess.showSnack(context, namedArgs: {'object': transaction.name});
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
