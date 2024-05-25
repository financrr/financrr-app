import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_button.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../models/log_entry.model.dart';
import '../models/log_store.dart';
import '../../../shared/models/store.dart';
import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../../routing/page_path.dart';
import '../../../../modules/settings/views/settings_page.dart';
import '../../../utils/common_actions.dart';

class LogSettingsPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: SettingsPage.pagePath, path: 'logs');

  const LogSettingsPage({super.key});

  @override
  ConsumerState<LogSettingsPage> createState() => _LogSettingsPageState();
}

class _LogSettingsPageState extends ConsumerState<LogSettingsPage> {
  bool _sortTimeAscending = false;
  int? _selectedEntryIndex;
  late List<LogEntry> _entries;

  @override
  void initState() {
    super.initState();
    _entries = LogEntryStore().getAsList();
    sortEntries();
  }

  void sortEntries() {
    _selectedEntryIndex = null;
    _entries.sort((a, b) => _sortTimeAscending ? a.timestamp.compareTo(b.timestamp) : b.timestamp.compareTo(a.timestamp));
  }

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);

    getColorTint(LogLevel level) {
      return switch (level) {
        LogLevel.config => theme.financrrExtension.primary,
        LogLevel.warning => Colors.orange,
        LogLevel.severe => theme.financrrExtension.error,
        LogLevel.shout => theme.financrrExtension.error,
        _ => null,
      };
    }

    buildLogEntryTile(LogEntry entry, int index, {bool expanded = false}) {
      Color? color = getColorTint(entry.level);
      return FinancrrCard(
        padding: const EdgeInsets.all(10),
        borderColor: color,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(entry.loggerName, style: theme.textTheme.bodyMedium?.copyWith(color: color, fontWeight: FontWeight.bold)),
            Text(entry.message, maxLines: expanded ? null : 1),
            Row(
              children: [
                Padding(
                  padding: const EdgeInsets.only(right: 5),
                  child: Icon(_getIcon(entry.level), color: color, size: 17),
                ),
                Expanded(child: Text('${entry.level.name}, ${StoreKey.dateTimeFormat.readSync()!.format(entry.timestamp)}')),
              ],
            ),
          ],
        ),
      );
    }

    buildDivider() {
      return Column(
        children: [
          Row(
            children: [
              FinancrrTextButton(
                onPressed: () => setState(() {
                  _sortTimeAscending = !_sortTimeAscending;
                  sortEntries();
                }),
                icon: Icon(_sortTimeAscending ? Icons.arrow_downward : Icons.arrow_upward, size: 17),
                label: (_sortTimeAscending ? L10nKey.commonSortOldestFirst : L10nKey.commonSortNewestFirst).toText(),
              ),
              const Spacer(),
              // TODO: add plurals
              Text('${_entries.length} entries'),
              IconButton(
                  onPressed: () => setState(() {
                        LogEntryStore().clear();
                        _entries.clear();
                      }),
                  icon: const Icon(Icons.delete_sweep_outlined))
            ],
          ),
          const SizedBox(height: 20),
        ],
      );
    }

    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Center(
          child: SizedBox(
            width: size.width / 1.1,
            child: ListView.separated(
                // +1 for the divider
                // +1 for the notice card if there are no logs
                itemCount: _entries.length + 1,
                separatorBuilder: (_, index) => index == 0 ? const SizedBox() : const SizedBox(height: 10),
                itemBuilder: (context, index) {
                  if (index == 0) {
                    return buildDivider();
                  }
                  return GestureDetector(
                    onTap: () => setState(() {
                      _selectedEntryIndex = _selectedEntryIndex == index - 1 ? null : index - 1;
                    }),
                    onLongPress: () => CommonActions.copyToClipboard(this, _entries[index - 1].message),
                    child: buildLogEntryTile(_entries[index - 1], index - 1, expanded: index - 1 == _selectedEntryIndex),
                  );
                }),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }

  IconData _getIcon(LogLevel level) {
    return switch (level) {
      LogLevel.finest => Icons.notes_outlined,
      LogLevel.finer => Icons.notes_outlined,
      LogLevel.fine => Icons.notes_outlined,
      LogLevel.config => Icons.handyman_outlined,
      LogLevel.info => Icons.info_outline,
      LogLevel.warning => Icons.warning_amber_outlined,
      LogLevel.severe => Icons.error_outline,
      LogLevel.shout => Icons.error_outline,
    };
  }
}
