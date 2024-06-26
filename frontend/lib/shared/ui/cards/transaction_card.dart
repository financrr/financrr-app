import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:restrr/restrr.dart';

import '../../../modules/settings/providers/l10n.provider.dart';
import '../../../modules/transactions/views/transaction_page.dart';
import '../../../utils/text_utils.dart';

class TransactionCard extends ConsumerWidget {
  final Id id;
  final Id? source;
  final Id? destination;
  final UnformattedAmount amount;
  final String name;
  final String? description;
  final DateTime executedAt;
  final DateTime createdAt;
  final Account account;
  final TransactionType? type;

  final bool interactive;

  TransactionCard({super.key, required Account? account, required Transaction transaction, this.interactive = true})
      : id = transaction.id.value,
        source = transaction.sourceId?.value,
        destination = transaction.destinationId?.value,
        amount = transaction.getDisplayAmount(account ?? (transaction.sourceId ?? transaction.destinationId)!.get()!),
        name = transaction.name,
        description = transaction.description,
        executedAt = transaction.executedAt,
        createdAt = transaction.createdAt,
        account = account ?? (transaction.sourceId ?? transaction.destinationId)!.get()!,
        type = account == null ? null : transaction.getType(account);

  const TransactionCard.fromData(
      {super.key,
      required this.id,
      this.source,
      this.destination,
      required this.amount,
      required this.name,
      required this.description,
      required this.executedAt,
      required this.createdAt,
      required this.account,
      required this.type,
      this.interactive = true});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var theme = ref.watch(themeProvider);
    var l10n = ref.watch(l10nProvider);

    return FinancrrCard(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
      onTap: !interactive
          ? null
          : () => context.goPath(TransactionPage.pagePath
              .build(params: {'accountId': account.id.value.toString(), 'transactionId': id.toString()})),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(name, style: theme.textTheme.titleSmall),
          if (description != null) Text(description!, style: theme.textTheme.bodyMedium),
          Text(TextUtils.formatIBAN(account.iban) ?? account.name),
          const SizedBox(height: 10),
          Row(
            mainAxisSize: MainAxisSize.max,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(l10n.dateFormat.format(executedAt)),
              Text(
                  amount.formatWithCurrency(account.currencyId.get()!, l10n.decimalSeparator,
                      thousandsSeparator: l10n.thousandSeparator),
                  style: theme.textTheme.titleMedium?.copyWith(
                      color: amount.rawAmount < 0 ? theme.financrrExtension.error : theme.financrrExtension.primary)),
            ],
          ),
        ],
      ),
    );
  }
}
