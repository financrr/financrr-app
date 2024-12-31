class StringUtils {
  const StringUtils._();

  /// Returns the number of occurrences of [search] in [str].
  static int count(String str, String search) {
    return search.allMatches(str).length;
  }

  static T? tryEnumFromString<T extends Enum>(String? value, List<T> values) {
    if (value == null) {
      return null;
    }
    for (T v in values) {
      if (v.name == value) {
        return v;
      }
    }
    return null;
  }
}
