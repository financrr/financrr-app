import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_button.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../../routing/page_path.dart';
import '../../../../modules/settings/views/settings_page.dart';
import '../../../../modules/settings/views/currency_create_page.dart';
import '../../../shared/ui/cards/currency_card.dart';
import '../../../shared/ui/paginated_wrapper.dart';

class CurrencySettingsPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: SettingsPage.pagePath, path: 'currencies');

  const CurrencySettingsPage({super.key});

  @override
  ConsumerState<CurrencySettingsPage> createState() => _CurrencySettingsPageState();
}

class _CurrencySettingsPageState extends ConsumerState<CurrencySettingsPage> {
  final GlobalKey<PaginatedWrapperState<Currency>> _paginatedCurrencyKey = GlobalKey();
  late final Restrr _api = api;

  final ValueNotifier<int> _amount = ValueNotifier(0);

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Center(
          child: SizedBox(
            width: size.width / 1.1,
            child: RefreshIndicator(
              onRefresh: () async => _paginatedCurrencyKey.currentState?.reset(),
              child: ListView(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      FinancrrTextButton(
                        onPressed: () => context.goPath(CurrencyCreatePage.pagePath.build()),
                        icon: const Icon(Icons.add),
                        label: L10nKey.commonCreate.toText(),
                      ),
                      ValueListenableBuilder(
                        valueListenable: _amount,
                        builder: (context, value, child) {
                          // TODO: add plurals
                          return Text('${_amount.value} currencies');
                        },
                      ),
                    ],
                  ),
                  const SizedBox(height: 20),
                  PaginatedWrapper(
                    key: _paginatedCurrencyKey,
                    initialPageFunction: (forceRetrieve) => _api.retrieveAllCurrencies(limit: 10, forceRetrieve: forceRetrieve),
                    onSuccess: (context, snap) {
                      final PaginatedDataResult<Currency> currencies = snap.data!;
                      WidgetsBinding.instance.addPostFrameCallback((_) {
                        _amount.value = currencies.total;
                      });
                      return Column(
                        children: [
                          for (Currency c in currencies.items)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 10),
                              child:
                                  CurrencyCard(currency: c, onDelete: c is! CustomCurrency ? null : () => _deleteCurrency(c)),
                            ),
                          if (currencies.nextPage != null)
                            FinancrrTextButton(
                              onPressed: () => currencies.nextPage!(_api),
                              label: L10nKey.commonLoadMore.toText(),
                            ),
                        ],
                      );
                    },
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }

  void _deleteCurrency(CustomCurrency currency) async {
    try {
      await currency.delete();
      if (!mounted) return;
      L10nKey.commonDeleteObjectSuccess.showSnack(context, namedArgs: {'object': currency.name});
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
