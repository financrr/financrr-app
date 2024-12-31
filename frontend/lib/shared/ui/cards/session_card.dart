import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_circle_avatar.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../modules/settings/providers/l10n.provider.dart';

class SessionCard extends StatefulHookConsumerWidget {
  final Id id;
  final String name;
  final String? description;
  final SessionPlatform platform;
  final DateTime createdAt;
  final DateTime expiresAt;
  final bool isCurrent;
  final bool interactive;
  final Function()? onDelete;

  SessionCard({super.key, required PartialSession session, this.interactive = true, this.onDelete})
      : id = session.id.value,
        name = session.name,
        description = session.description,
        platform = session.platform,
        createdAt = session.createdAt,
        expiresAt = session.expiresAt,
        isCurrent = session.api.session.id.value == session.id.value;

  const SessionCard.fromData(
      {super.key,
      required this.id,
      required this.name,
      this.description,
      required this.platform,
      required this.createdAt,
      required this.expiresAt,
      required this.isCurrent,
      this.interactive = true,
      this.onDelete});

  @override
  ConsumerState<SessionCard> createState() => _SessionCardState();
}

class _SessionCardState extends ConsumerState<SessionCard> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    var theme = ref.watch(themeProvider);
    var l10n = ref.watch(l10nProvider);

    return FinancrrCard(
      onLongPress: widget.interactive ? () => setState(() => _expanded = !_expanded) : null,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 10),
      child: Row(
        children: [
          FinancrrCircleAvatar(radius: 25, child: Icon(_getPlatformIcon(widget.platform), size: 30)),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('${widget.name} (${_getPlatformApp(widget.platform)})', style: theme.textTheme.titleSmall),
                if (widget.description != null && _expanded)
                  Padding(
                    padding: const EdgeInsets.only(top: 5),
                    child: Text(widget.description!, style: theme.textTheme.bodySmall),
                  ),
                Padding(
                  padding: const EdgeInsets.only(top: 5),
                  child: Text('${l10n.dateFormat.format(widget.createdAt)} ${_expanded ? '(${widget.id})' : ''}'),
                ),
              ],
            ),
          ),
          IconButton(
            onPressed: widget.onDelete,
            icon: Icon(widget.isCurrent ? Icons.logout_rounded : Icons.delete_outline_rounded),
          )
        ],
      ),
    );
  }

  IconData _getPlatformIcon(SessionPlatform platform) {
    return switch (platform) {
      SessionPlatform.android => Icons.android_rounded,
      SessionPlatform.ios => Icons.phone_iphone_rounded,
      SessionPlatform.web => Icons.language_rounded,
      SessionPlatform.windows => Icons.desktop_windows_rounded,
      SessionPlatform.macos => Icons.desktop_windows_rounded,
      SessionPlatform.linux => Icons.desktop_windows_rounded,
      SessionPlatform.unknown => Icons.devices_other_rounded,
    };
  }

  String _getPlatformApp(SessionPlatform platform) {
    return switch (platform) {
      SessionPlatform.android => 'financrr App',
      SessionPlatform.ios => 'financrr App',
      SessionPlatform.windows => 'financrr Client',
      SessionPlatform.macos => 'financrr Client',
      SessionPlatform.linux => 'financrr Client',
      SessionPlatform.web => 'financrr Web',
      SessionPlatform.unknown => 'Unknown',
    };
  }
}
