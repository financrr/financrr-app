import 'dart:math';

import 'package:financrr_frontend/pages/core/settings/currency_settings_page.dart';
import 'package:financrr_frontend/util/extensions.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:restrr/restrr.dart';

import '../../../../layout/adaptive_scaffold.dart';
import '../../../../router.dart';
import '../../../../util/form_fields.dart';

class CurrencyCreatePage extends StatefulWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: CurrencySettingsPage.pagePath, path: 'create');

  const CurrencyCreatePage({super.key});

  @override
  State<StatefulWidget> createState() => _CurrencyCreatePageState();

  static List<Widget> buildCurrencyPreview(
      {required Size size,
      required String symbol,
      required String name,
      required String isoCode,
      required String decimalPlaces,
      required double previewAmount}) {
    return [
      Card.outlined(
        child: ListTile(
          leading: const Text('Preview'),
          title: Text('${previewAmount.toStringAsFixed(int.tryParse(decimalPlaces) ?? 0)} $symbol'),
        ),
      ),
      SizedBox(
        width: size.width,
        child: DataTable(columns: const [
          DataColumn(label: Text('Symbol')),
          DataColumn(label: Text('Name')),
          DataColumn(label: Text('ISO')),
        ], rows: [
          DataRow(cells: [DataCell(Text(symbol)), DataCell(Text(name)), DataCell(Text(isoCode))])
        ]),
      ),
    ];
  }
}

class _CurrencyCreatePageState extends State<CurrencyCreatePage> {
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = context.api!;

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
    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => SafeArea(child: _buildVerticalLayout(size)),
    );
  }

  Widget _buildVerticalLayout(Size size) {
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
                const Divider(),
                ...FormFields.currency(
                    nameController: _nameController,
                    symbolController: _symbolController,
                    isoCodeController: _isoCodeController,
                    decimalPlacesController: _decimalPlacesController),
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: _isValid ? () => _createCurrency() : null,
                    child: Text(_nameController.text.isEmpty ? 'Create Currency' : 'Create "${_nameController.text}"'),
                  ),
                ),
              ],
            ),
          )),
        ),
      ),
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
      context.showSnackBar('Successfully created "${_nameController.text}"');
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
