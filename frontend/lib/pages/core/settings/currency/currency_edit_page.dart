import 'dart:async';
import 'dart:math';

import 'package:financrr_frontend/pages/core/settings/currency_settings_page.dart';
import 'package:financrr_frontend/util/extensions.dart';
import 'package:financrr_frontend/util/input_utils.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:go_router/go_router.dart';
import 'package:restrr/restrr.dart';

import '../../../../layout/adaptive_scaffold.dart';
import '../../../../router.dart';
import '../../../../widgets/async_wrapper.dart';

class CurrencyEditPage extends StatefulWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: CurrencySettingsPage.pagePath, path: 'edit');

  final String? currencyId;

  const CurrencyEditPage({super.key, required this.currencyId});

  @override
  State<StatefulWidget> createState() => _CurrencyEditPageState();
}

class _CurrencyEditPageState extends State<CurrencyEditPage> {
  final StreamController<Currency> _currencyStreamController = StreamController.broadcast();
  final GlobalKey<FormState> _formKey = GlobalKey();

  late final Restrr _api = context.api!;

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
    return AdaptiveScaffold(verticalBuilder: (_, __, size) => SafeArea(child: _handleCurrencyStream(size)));
  }

  Widget _handleCurrencyStream(Size size) {
    return StreamWrapper(
      stream: _currencyStreamController.stream,
      onSuccess: (ctx, snap) {
        return _buildVerticalLayout(snap.data!, size);
      },
      onLoading: (ctx, snap) {
        return const Center(child: CircularProgressIndicator());
      },
      onError: (ctx, snap) {
        return const Text('Could not find currency');
      },
    );
  }

  Widget _buildVerticalLayout(Currency currency, Size size) {
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
                Card(
                  child: ListTile(
                    leading: const Text('Preview'),
                    title: Text(
                        '${_randomNumber.toStringAsFixed(int.tryParse(_decimalPlacesController.text) ?? 0)} ${_symbolController.text}'),
                  ),
                ),
                SizedBox(
                  width: size.width,
                  child: DataTable(columns: const [
                    DataColumn(label: Text('Symbol')),
                    DataColumn(label: Text('Name')),
                    DataColumn(label: Text('ISO Code')),
                  ], rows: [
                    DataRow(cells: [
                      DataCell(Text(_symbolController.text)),
                      DataCell(Text(_nameController.text)),
                      DataCell(Text(_isoCodeController.text))
                    ])
                  ]),
                ),
                const Divider(),
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 10),
                  child: TextFormField(
                    controller: _nameController,
                    readOnly: !_isCustom,
                    decoration: const InputDecoration(labelText: 'Name'),
                    validator: (value) => InputValidators.nonNull('Name', value),
                    inputFormatters: [
                      LengthLimitingTextInputFormatter(32),
                    ],
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: TextFormField(
                    controller: _symbolController,
                    readOnly: !_isCustom,
                    decoration: const InputDecoration(labelText: 'Symbol'),
                    validator: (value) => InputValidators.nonNull('Symbol', value),
                    inputFormatters: [
                      LengthLimitingTextInputFormatter(6),
                    ],
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: TextFormField(
                    controller: _isoCodeController,
                    readOnly: !_isCustom,
                    decoration: const InputDecoration(labelText: 'ISO Code'),
                    inputFormatters: [
                      LengthLimitingTextInputFormatter(3),
                    ],
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: TextFormField(
                    controller: _decimalPlacesController,
                    readOnly: !_isCustom,
                    decoration: const InputDecoration(labelText: 'Decimal Places'),
                    validator: (value) => InputValidators.nonNull('Decimal Places', value),
                    inputFormatters: [
                      FilteringTextInputFormatter.digitsOnly,
                      LengthLimitingTextInputFormatter(1),
                    ],
                  ),
                ),
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: _isValid && _isCustom ? () => _editCurrency(currency as CustomCurrency) : null,
                    child: Text(_nameController.text.isEmpty ? 'Edit Currency' : 'Edit "${_nameController.text}"'),
                  ),
                ),
                if (!_isCustom)
                  Padding(
                    padding: const EdgeInsets.only(top: 10),
                    child: Row(
                      mainAxisSize: MainAxisSize.max,
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        // TODO: Wrap text
                        const Icon(Icons.warning),
                        Padding(
                          padding: const EdgeInsets.only(left: 5),
                          child: Text('This currency is not custom and can therefore not be edited!',
                              style: context.textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w700)),
                        ),
                      ],
                    ),
                  )
              ],
            ),
          )),
        ),
      ),
    );
  }

  Future<void> _editCurrency(CustomCurrency currency) async {
    if (!_isValid || !_isCustom) return;
    try {
      await currency.update(
          name: _nameController.text,
          symbol: _symbolController.text,
          isoCode: _isoCodeController.text,
          decimalPlaces: int.parse(_decimalPlacesController.text));
      if (!mounted) return;
      context.showSnackBar('Successfully edited "${_nameController.text}"');
      context.pop();
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}