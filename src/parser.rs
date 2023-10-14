// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use icu_locid::Locale;

/// Construct `Self` by parsing humanized text.
pub trait Parse: Sized {
    /// Perform the conversion.
    fn parse(text: &str, locale: &Locale) -> Option<Self>;
}

/// Construct a value by parsing humanized text.
///
/// This uses a wild card for the locale, so text in any locale
/// supported by the library should work.
pub fn parse<T: Parse>(text: &str) -> Option<T> {
    let locale = Locale::default();
    T::parse(text, &locale)
}

/// Construct a value by parsing humanized `text`, with a `default`
/// value when parsing fails.
///
/// This uses a wild card for the locale, so text in any locale
/// supported by the library should work.
pub fn parse_or<T: Parse>(text: &str, default: T) -> T {
    parse::<T>(text).unwrap_or(default)
}

/// Construct a value by parsing humanized `text` using the specified [`locale`].
///
/// [`locale`]: Locale
pub fn parse_with_locale<T: Parse>(text: &str, locale: &Locale) -> Option<T> {
    T::parse(text, locale)
}

/// Construct a value by parsing humanized `text` using the specified [`locale`],
/// with a `default` value when parsing fails.
///
/// [`locale`]: Locale
pub fn parse_with_locale_or<T: Parse>(text: &str, locale: &Locale, default: T) -> T {
    T::parse(text, locale).unwrap_or(default)
}
