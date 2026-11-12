/// Up to two initials from a name, upper-cased: the first letter or digit of the first two words
/// (`"ada lovelace"` is `"AL"`, `"Ada"` is `"A"`, `"Ada King Lovelace"` is `"AK"`). Words are split on
/// whitespace; a word with no letter or digit (a lone `-`) is skipped, and punctuation at the start of a
/// word is ignored (`"(Ada) Lovelace"` is `"AL"`). An empty name gives an empty string.
///
/// Used by [`crate::AvatarFallback`] (`name`) and [`crate::ShellUser`].
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().find(|c| c.is_alphanumeric()))
        .take(2)
        .flat_map(char::to_uppercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::initials;

    #[test]
    fn takes_the_first_letters_of_the_first_two_words() {
        assert_eq!(initials("ada lovelace"), "AL");
        assert_eq!(initials("Ada"), "A");
        assert_eq!(initials("Ada King Lovelace"), "AK");
        assert_eq!(initials("  Grace   Hopper  "), "GH");
    }

    #[test]
    fn skips_punctuation_and_empty_words() {
        assert_eq!(initials("(Ada) Lovelace"), "AL");
        assert_eq!(initials("Ada - Lovelace"), "AL");
        assert_eq!(initials(""), "");
        assert_eq!(initials("   "), "");
        assert_eq!(initials("---"), "");
    }

    #[test]
    fn handles_non_ascii_letters() {
        assert_eq!(initials("élodie dupont"), "ÉD");
    }
}
