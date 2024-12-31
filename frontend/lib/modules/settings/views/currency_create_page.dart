import 'dart:math';

import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_button.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/cards/currency_card.dart';
import '../../../utils/form_fields.dart';
import 'currency_settings_page.dart';

class CurrencyCreatePage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: CurrencySettingsPage.pagePath, path: 'create');

  const CurrencyCreatePage({super.key});

  @override
  ConsumerState<CurrencyCreatePage> createState() => _CurrencyCreatePageState();

  static List<Widget> buildCurrencyPreview(
      {required Size size,
      required String symbol,
      required String name,
      required String isoCode,
      required String decimalPlaces,
      required double previewAmount}) {
    return [
      FinancrrCard(
        child: ListTile(
          leading: L10nKey.commonPreview.toText(),
          title: Text('${previewAmount.toStringAsFixed(int.tryParse(decimalPlaces) ?? 0)} $symbol'),
        ),
      ),
      const SizedBox(height: 10),
      CurrencyCard.fromData(
        id: -1,
        name: name,
        symbol: symbol,
        decimalPlaces: int.tryParse(decimalPlaces) ?? 0,
        isoCode: isoCode.isEmpty ? null : isoCode,
        isCustom: true,
        interactive: false,
      )
    ];
  }
}

class _CurrencyCreatePageState extends ConsumerState<CurrencyCreatePage> {
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = api;

  late final TextEditingController _symbolController;
  late final TextEditingController _nameController;
  late final TextEditingController _isoCodeController;
  late final TextEditingController _decimalPlacesController;

  late final double _randomNumber;

  bool _isValid = false;

  @override
  void initState() {
    super.initState();
    _symbolController = TextEditingController(text: '€');
    _nameController = TextEditingController(text: 'Euro');
    _isoCodeController = TextEditingController(text: 'EUR');
    _decimalPlacesController = TextEditingController(text: '2');
    _isValid = _formKey.currentState?.validate() ?? false;
    final Random random = Random();
    _randomNumber = random.nextDouble() + (random.nextInt(128) + 128);
  }

  @override
  void dispose() {
    _symbolController.dispose();
    _nameController.dispose();
    _isoCodeController.dispose();
    _decimalPlacesController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
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
                      decimalPlacesController: _decimalPlacesController),
                  const SizedBox(height: 20),
                  FinancrrButton(
                    onPressed: _isValid ? () => _createCurrency() : null,
                    text: _nameController.text.isEmpty
                        ? L10nKey.currencyCreate.toString()
                        : L10nKey.commonCreateObject.toString(namedArgs: {'object': _nameController.text}),
                  ),
                ],
              ),
            )),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }

  Future<void> _createCurrency() async {
    if (!_isValid) return;
    try {
      await _api.createCurrency(
        name: _nameController.text,
        symbol: _symbolController.text,
        decimalPlaces: int.parse(_decimalPlacesController.text),
        isoCode: _isoCodeController.text.isEmpty ? null : _isoCodeController.text,
      );
      if (!mounted) return;
      L10nKey.commonCreateObjectSuccess.showSnack(context, namedArgs: {'object': _nameController.text});
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
