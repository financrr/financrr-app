import 'package:financrr_frontend/shared/ui/custom_replacements/custom_card.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../modules/settings/providers/theme.provider.dart';

class NavDestination {
  final L10nKey label;
  final IconData iconData;
  final IconData? selectedIconData;

  const NavDestination({required this.label, required this.iconData, this.selectedIconData});
}

class FinancrrNavigationRail extends ConsumerWidget {
  final List<NavDestination> destinations;
  final int selectedIndex;
  final bool extended;
  final Function(int)? onDestinationSelected;
  final List<Widget> Function(bool)? trailingBuilder;

  const FinancrrNavigationRail(
      {super.key,
      required this.destinations,
      required this.selectedIndex,
      this.extended = true,
      this.onDestinationSelected,
      this.trailingBuilder});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var theme = ref.watch(themeProvider);

    buildDestination(NavDestination destination, int index) {
      final bool isSelected = index == selectedIndex;
      bool isHovered = false;

      return StatefulBuilder(
        builder: (context, setState) {
          return MouseRegion(
            onHover: (_) => setState(() => isHovered = true),
            onExit: (_) => setState(() => isHovered = false),
            child: FinancrrCard(
              onTap: () => onDestinationSelected?.call(index),
              borderColor: Colors.transparent,
              child: Row(
                mainAxisAlignment: extended ? MainAxisAlignment.start : MainAxisAlignment.center,
                children: [
                  if (extended) const SizedBox(width: 40),
                  AnimatedContainer(
                    duration: const Duration(milliseconds: 200),
                    padding: const EdgeInsets.all(5),
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(width: 3, color: isSelected ? theme.financrrExtension.primary : Colors.transparent),
                      color: isSelected ? null : theme.financrrExtension.surfaceVariant1,
                    ),
                    child: Icon(isSelected ? destination.selectedIconData ?? destination.iconData : destination.iconData,
                        color: isSelected ? theme.financrrExtension.primary : theme.financrrExtension.surfaceVariant3),
                  ),
                  if (extended) ...[
                    const SizedBox(width: 25),
                    destination.label.toText(
                        softWrap: true,
                        style: theme.textTheme.bodyMedium?.copyWith(
                            color: isSelected ? theme.financrrExtension.primary : null,
                            fontWeight: isSelected || isHovered ? FontWeight.w600 : FontWeight.w500))
                  ]
                ],
              ),
            ),
          );
        },
      );
    }

    return AnimatedSize(
      duration: const Duration(milliseconds: 100),
      child: SizedBox(
        width: extended ? 350 : 100,
        child: Column(
          crossAxisAlignment: extended ? CrossAxisAlignment.start : CrossAxisAlignment.center,
          children: [
            const SizedBox(height: 10),
            for (NavDestination destination in destinations)
              Padding(
                  padding: const EdgeInsets.symmetric(vertical: 5),
                  child: buildDestination(destination, destinations.indexOf(destination))),
            if (trailingBuilder != null) ...trailingBuilder!(extended),
          ],
        ),
      ),
    );
  }
}
