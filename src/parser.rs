// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use icu_locid::Locale;

/// Construct `Self` by parsing humanized `text`.
pub trait Parse: Sized {
    /// Perform the conversion.
    fn parse(text: &str, locale: Option<&Locale>) -> Option<Self>;
}

/// Construct a value by parsing humanized `text`.
pub fn parse<T: Parse>(text: &str, locale: Option<&Locale>) -> Option<T> {
    T::parse(text, locale)
}

/// Construct a value by parsing humanized `text`, with a `default`
/// value when parsing fails.
pub fn parse_or<T: Parse>(text: &str, locale: Option<&Locale>, default: T) -> T {
    parse::<T>(text, locale).unwrap_or(default)
}
