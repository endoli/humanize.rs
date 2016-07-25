// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use language_tags::LanguageTag;

/// Construct `Self` by parsing humanized text.
pub trait Parse: Sized {
    /// Perform the conversion.
    fn parse(text: &str, language: &LanguageTag) -> Option<Self>;
}

/// Construct a value by parsing humanized text.
///
/// This uses a wild card for the language, so text in any language
/// supported by the library should work.
pub fn parse<T: Parse>(text: &str) -> Option<T> {
    let language = LanguageTag::default();
    T::parse(text, &language)
}

/// Construct a value by parsing humanized text using the specified language.
pub fn parse_with_language<T: Parse>(text: &str, language: &LanguageTag) -> Option<T> {
    T::parse(text, language)
}
