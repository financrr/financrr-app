import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../modules/settings/views/currency_edit_page.dart';
import '../custom_replacements/custom_circle_avatar.dart';

class CurrencyCard extends ConsumerWidget {
  final Id id;
  final String name;
  final String symbol;
  final int decimalPlaces;
  final String? isoCode;
  final bool isCustom;
  final bool interactive;
  final Function()? onDelete;

  CurrencyCard({super.key, required Currency currency, this.interactive = true, this.onDelete})
      : id = currency.id.value,
        name = currency.name,
        symbol = currency.symbol,
        decimalPlaces = currency.decimalPlaces,
        isoCode = currency.isoCode,
        isCustom = currency is CustomCurrency;

  const CurrencyCard.fromData(
      {super.key,
      required this.id,
      required this.name,
      required this.symbol,
      required this.decimalPlaces,
      this.isoCode,
      required this.isCustom,
      this.interactive = true,
      this.onDelete});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var theme = ref.watch(themeProvider);

    return FinancrrCard(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
      child: Row(
        children: [
          FinancrrCircleAvatar.text(text: symbol, radius: 25),
          const SizedBox(width: 20),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(name, style: theme.textTheme.titleSmall),
                if (isoCode != null) Text(isoCode!),
              ],
            ),
          ),
          PopupMenuButton(
            enabled: interactive && isCustom,
            itemBuilder: (context) {
              return [
                PopupMenuItem(
                  onTap: () => context.goPath(CurrencyEditPage.pagePath.build(params: {'currencyId': id.toString()})),
                  child: L10nKey.commonEdit.toText(),
                ),
                if (onDelete != null)
                  PopupMenuItem(
                    onTap: onDelete,
                    child: L10nKey.commonDelete.toText(),
                  ),
              ];
            },
          )
        ],
      ),
    );
  }
}
