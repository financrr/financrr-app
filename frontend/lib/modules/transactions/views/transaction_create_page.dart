import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_button.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/formatter/money_input_formatter.dart';
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
import '../../accounts/views/account_page.dart';
import '../../settings/providers/l10n.provider.dart';

class TransactionCreatePage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: AccountPage.pagePath, path: 'transactions/create');

  final String? accountId;
  final TransactionTemplate? template;

  const TransactionCreatePage({super.key, required this.accountId, this.template});

  @override
  ConsumerState<TransactionCreatePage> createState() => _TransactionCreatePageState();
}

class _TransactionCreatePageState extends ConsumerState<TransactionCreatePage> {
  final StreamController<Account> _accountStreamController = StreamController.broadcast();
  final GlobalKey<FormState> _formKey = GlobalKey();
  late final Restrr _api = api;

  late final TextEditingController _nameController = TextEditingController(text: 'Transaction');
  final TextEditingController _amountController = TextEditingController();
  final TextEditingController _descriptionController = TextEditingController();
  late final TextEditingController _executedAtController;

  bool _isValid = false;
  UnformattedAmount _amount = UnformattedAmount.zero;
  TransactionType _type = TransactionType.deposit;
  DateTime _executedAt = DateTime.now();
  Account? _secondary;

  Future<Account?> _fetchAccount({bool forceRetrieve = false}) async {
    return _accountStreamController.fetchData(
        widget.accountId, (id) => _api.retrieveAccountById(id, forceRetrieve: forceRetrieve));
  }

  @override
  void initState() {
    super.initState();
    _executedAtController = TextEditingController(text: StoreKey.dateTimeFormat.readSync()!.format(_executedAt));
    _fetchAccount().then((account) {
      if (widget.template != null) {
        _nameController.text = widget.template!.name;
        _descriptionController.text = widget.template!.description ?? '';
        _amount = widget.template!.amount;
        _type = widget.template!.getType(account!);
      }
    });
    _isValid = _formKey.currentState?.validate() ?? false;
  }

  @override
  void dispose() {
    _nameController.dispose();
    _amountController.dispose();
    _descriptionController.dispose();
    _executedAtController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var l10n = ref.watch(l10nProvider);

    buildVerticalLayout(Account account, Size size) {
      final MoneyInputFormatter moneyFormatter = MoneyInputFormatter.fromCurrency(
        currency: account.currencyId.get()!,
        decimalSeparator: l10n.decimalSeparator,
        thousandSeparator: l10n.thousandSeparator,
      );
      if (_amountController.text.isEmpty) {
        _amountController.text = moneyFormatter
            .formatEditUpdate(const TextEditingValue(text: ''), TextEditingValue(text: _amount.rawAmount.toString()))
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
                    moneyInputFormatter: moneyFormatter,
                    onAmountChanged: (amount) => setState(() => _amount = amount),
                    onSelectionChanged: (types) => setState(() => _type = types.first),
                    onSecondaryChanged: (account) => setState(() => _secondary = account),
                    onExecutedAtChanged: (date) => setState(() => _executedAt = date),
                  ),
                  const SizedBox(height: 20),
                  FinancrrButton(
                    onPressed: _isValid ? () => _createTransaction(account, _type, secondary: _secondary) : null,
                    text: L10nKey.transactionCreate.toString(),
                  ),
                ],
              ),
            )),
          ),
        ),
      );
    }

    handleAccountStream(Size size) {
      return StreamWrapper(
        stream: _accountStreamController.stream,
        onSuccess: (_, snap) => buildVerticalLayout(snap.data!, size),
        onLoading: (_, __) => const Center(child: CircularProgressIndicator()),
        onError: (ctx, snap) => L10nKey.accountNotFound.toText(),
      );
    }

    return AdaptiveScaffold(verticalBuilder: (_, __, size) => handleAccountStream(size));
  }

  Future<void> _createTransaction(Account account, TransactionType type, {Account? secondary}) async {
    if (!_isValid) return;
    final (Id?, Id?) sourceAndDest = switch (_type) {
      //                                  from                  to
      TransactionType.deposit => (null, account.id.value),
      TransactionType.withdrawal => (account.id.value, null),
      TransactionType.transferIn => (secondary!.id.value, account.id.value),
      TransactionType.transferOut => (account.id.value, secondary!.id.value),
    };
    try {
      Transaction transaction = await _api.createTransaction(
          sourceId: sourceAndDest.$1,
          destinationId: sourceAndDest.$2,
          amount: _amount,
          name: _nameController.text,
          description: _descriptionController.text.isEmpty ? null : _descriptionController.text,
          executedAt: _executedAt,
          currencyId: account.currencyId.value);
      if (!mounted) return;
      L10nKey.commonCreateObjectSuccess.showSnack(context, namedArgs: {'object': transaction.name});
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
