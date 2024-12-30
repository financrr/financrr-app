import 'dart:async';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/formatter/money_input_formatter.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:restrr/restrr.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/async_wrapper.dart';
import '../../../shared/ui/cards/account_card.dart';
import '../../../shared/ui/custom_replacements/custom_button.dart';
import '../../../utils/form_fields.dart';
import '../../settings/providers/l10n.provider.dart';
import 'account_page.dart';

class AccountEditPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: AccountPage.pagePath, path: 'edit');

  final String? accountId;

  const AccountEditPage({super.key, required this.accountId});

  @override
  ConsumerState<AccountEditPage> createState() => AccountEditPageState();
}

class AccountEditPageState extends ConsumerState<AccountEditPage> {
  final StreamController<Account> _accountStreamController = StreamController.broadcast();
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = api;

  late final TextEditingController _nameController;
  late final TextEditingController _descriptionController;
  late final TextEditingController _ibanController;
  late final TextEditingController _originalBalanceController;

  UnformattedAmount _originalBalance = UnformattedAmount.zero;
  Currency? _currency;

  bool _isValid = false;

  Future<Account?> _fetchAccount({bool forceRetrieve = false}) async {
    return _accountStreamController.fetchData(
        widget.accountId, (id) => _api.retrieveAccountById(id, forceRetrieve: forceRetrieve));
  }

  @override
  void initState() {
    super.initState();
    _fetchAccount().then((account) {
      if (account != null) {
        _nameController = TextEditingController(text: account.name);
        _descriptionController = TextEditingController(text: account.description);
        _ibanController = TextEditingController(text: account.iban);
        _originalBalanceController = TextEditingController();
        _currency = account.currencyId.get()!;
      }
    });

    _isValid = _formKey.currentState?.validate() ?? false;
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    _ibanController.dispose();
    _originalBalanceController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var l10n = ref.watch(l10nProvider);

    buildVerticalLayout(Account account, Size size) {
      final MoneyInputFormatter moneyFormatter = MoneyInputFormatter.fromCurrency(
        currency: account.currencyId.get() ?? _api.getCurrencies().first,
        decimalSeparator: l10n.decimalSeparator,
        thousandSeparator: l10n.thousandSeparator,
      );
      if (_originalBalanceController.text.isEmpty) {
        _originalBalanceController.text = moneyFormatter
            .formatEditUpdate(const TextEditingValue(text: ''), TextEditingValue(text: account.originalBalance.toString()))
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
                  AccountCard.fromData(
                    id: 0,
                    name: _nameController.text,
                    iban: _ibanController.text,
                    description: _descriptionController.text,
                    balance: _originalBalance,
                    currency: _currency ?? _api.getCurrencies().first,
                    interactive: false,
                  ),
                  const SizedBox(height: 20),
                  ...FormFields.account(ref,
                      api: _api,
                      nameController: _nameController,
                      descriptionController: _descriptionController,
                      ibanController: _ibanController,
                      originalBalanceController: _originalBalanceController,
                      selectedCurrency: _currency ?? _api.getCurrencies().first,
                      moneyInputFormatter: moneyFormatter,
                      onOriginalBalanceChanged: (balance) => setState(() => _originalBalance = balance),
                      onCurrencyChanged: (currency) => setState(() => _currency = currency!)),
                  const SizedBox(height: 20),
                  FinancrrButton(
                    onPressed: _isValid ? () => _editAccount(account) : null,
                    text: _nameController.text.isEmpty
                        ? L10nKey.accountEdit.toString()
                        : L10nKey.commonEditObject.toString(namedArgs: {'object': _nameController.text}),
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
          onError: (_, __) => L10nKey.accountNotFound.toText());
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => handleAccountStream(size),
    );
  }

  Future<void> _editAccount(Account account) async {
    if (!_isValid || _currency == null) return;
    try {
      await account.update(
        name: _nameController.text,
        description: _descriptionController.text.isEmpty ? null : _descriptionController.text,
        iban: _ibanController.text.isEmpty ? null : _ibanController.text,
        originalBalance: _originalBalance,
        currencyId: _currency!.id.value,
      );
      if (!mounted) return;
      L10nKey.commonEditObjectSuccess.showSnack(context, namedArgs: {'object': _nameController.text});
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
