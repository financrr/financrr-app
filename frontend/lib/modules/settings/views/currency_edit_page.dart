import 'dart:async';
import 'dart:math';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/modules/settings/views/currency_create_page.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../../routing/page_path.dart';
import '../../../shared/ui/async_wrapper.dart';
import '../../../shared/ui/custom_replacements/custom_button.dart';
import '../../../utils/form_fields.dart';
import 'currency_settings_page.dart';

class CurrencyEditPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath =
      PagePathBuilder.child(parent: CurrencySettingsPage.pagePath, path: ':currencyId/edit');

  final String? currencyId;

  const CurrencyEditPage({super.key, required this.currencyId});

  @override
  ConsumerState<CurrencyEditPage> createState() => _CurrencyEditPageState();
}

class _CurrencyEditPageState extends ConsumerState<CurrencyEditPage> {
  final StreamController<Currency> _currencyStreamController = StreamController.broadcast();
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = api;

  late final TextEditingController _symbolController;
  late final TextEditingController _nameController;
  late final TextEditingController _isoCodeController;
  late final TextEditingController _decimalPlacesController;

  late final double _randomNumber;

  bool _isValid = false;
  bool _isCustom = false;

  Future<Currency?> _fetchCurrency({bool forceRetrieve = false}) async {
    return _currencyStreamController.fetchData(
        widget.currencyId, (id) => _api.retrieveCurrencyById(id, forceRetrieve: forceRetrieve));
  }

  @override
  void initState() {
    super.initState();
    _fetchCurrency().then((currency) {
      if (currency != null) {
        _symbolController = TextEditingController(text: currency.symbol);
        _nameController = TextEditingController(text: currency.name);
        _isoCodeController = TextEditingController(text: currency.isoCode);
        _decimalPlacesController = TextEditingController(text: currency.decimalPlaces.toString());
        _isValid = _formKey.currentState?.validate() ?? false;
        _isCustom = currency is CustomCurrency;
      }
    });
    final Random random = Random();
    _randomNumber = random.nextDouble() + (random.nextInt(128) + 128);
  }

  @override
  void dispose() {
    _currencyStreamController.close();
    _symbolController.dispose();
    _nameController.dispose();
    _isoCodeController.dispose();
    _decimalPlacesController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);

    buildVerticalLayout(Currency currency, Size size) {
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
                  ...CurrencyCreatePage.buildCurrencyPreview(
                      size: size,
                      symbol: _symbolController.text,
                      name: _nameController.text,
                      isoCode: _isoCodeController.text,
                      decimalPlaces: _decimalPlacesController.text,
                      previewAmount: _randomNumber),
                  const SizedBox(height: 20),
                  ...FormFields.currency(
                      nameController: _nameController,
                      symbolController: _symbolController,
                      isoCodeController: _isoCodeController,
                      decimalPlacesController: _decimalPlacesController,
                      readOnly: !_isCustom),
                  const SizedBox(height: 20),
                  FinancrrButton(
                    onPressed: _isValid && _isCustom ? () => _editCurrency(currency as CustomCurrency) : null,
                    text: _nameController.text.isEmpty
                        ? L10nKey.currencyEdit.toString()
                        : L10nKey.commonEditObject.toString(namedArgs: {'object': _nameController.text}),
                  ),
                  if (!_isCustom)
                    Padding(
                      padding: const EdgeInsets.only(top: 10),
                      child: SizedBox(
                        width: size.width,
                        child: Row(
                          mainAxisSize: MainAxisSize.max,
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            const Icon(Icons.warning_amber_rounded),
                            Expanded(
                              child: Padding(
                                padding: const EdgeInsets.only(left: 10),
                                child: L10nKey.currencyNotEditable
                                    .toText(style: theme.textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w700)),
                              ),
                            ),
                          ],
                        ),
                      ),
                    )
                ],
              ),
            )),
          ),
        ),
      );
    }

    handleCurrencyStream(Size size) {
      return StreamWrapper(
        stream: _currencyStreamController.stream,
        onSuccess: (ctx, snap) => buildVerticalLayout(snap.data!, size),
        onLoading: (ctx, snap) => const Center(child: CircularProgressIndicator()),
        onError: (ctx, snap) => L10nKey.currencyNotFound.toText(),
      );
    }

    return AdaptiveScaffold(verticalBuilder: (_, __, size) => handleCurrencyStream(size));
  }

  Future<void> _editCurrency(CustomCurrency currency) async {
    if (!_isValid || !_isCustom) return;
    try {
      await currency.update(
        name: _nameController.text,
        symbol: _symbolController.text,
        decimalPlaces: int.parse(_decimalPlacesController.text),
        isoCode: _isoCodeController.text.isEmpty ? null : _isoCodeController.text,
      );
      if (!mounted) return;
      L10nKey.commonEditObjectSuccess.showSnack(context, namedArgs: {'object': _nameController.text});
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
