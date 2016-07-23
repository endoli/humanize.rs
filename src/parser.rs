// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use language_tags::LanguageTag;
use matchers::*;

/// Parse text input by a human and return a primitive value.
///
/// The actual matching is done by `Matcher` objects which are installed
/// by default when the parser is created. A parser without pre-installed
/// matchers can be created with `new_without_default_matchers`. Additional
/// or custom matchers can be added with `register_matcher`.
pub struct HumanizedParser<'p> {
    /// The matchers which have been registered with this parser.
    ///
    /// Use `HumanizedParser.register_matcher` to add a new matcher.
    matchers: Vec<Matcher<'p>>,
}

impl<'p> HumanizedParser<'p> {
    /// Construct a new parser, including the default matchers.
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct a new parser, but without any of the default matchers.
    pub fn new_without_default_matchers() -> Self {
        HumanizedParser { matchers: vec![] }
    }

    /// Parse `text`, looking for a `bool` value.
    ///
    /// If you don't want to limit the matching to a particular language,
    /// pass `Default::default()` for the `language`.
    pub fn parse_boolean(&self, text: &str, language: LanguageTag) -> Option<bool> {
        let matches = self.parse(text, ValueType::Boolean, language);
        match matches.first().map(|ref m| &m.value) {
            Some(&HumanValue::Boolean(val)) => Some(val),
            _ => None,
        }
    }

    /// Parse `text`, looking for a value of the [desired type],  using
    /// the optionally provided language.
    ///
    /// The resulting collection of matches will be ordered by their
    /// weight of likelihood with the most likely first.
    ///
    /// You won't typically invoke this directly, instead preferring to
    /// use type-specific methods like `parse_boolean`.
    ///
    /// [desired type]: matchers/enum.ValueType.html
    pub fn parse(&self, text: &str, desired: ValueType, language: LanguageTag) -> Vec<Match> {
        let mut matches = vec![];
        for matcher in &self.matchers {
            if matcher.result_type == desired && language.matches(&matcher.language) {
                if let Some(m) = (matcher.matcher)(text) {
                    matches.push(m);
                }
            }
        }
        matches
    }

    /// Install a new `Matcher` to be used by this parser.
    pub fn register_matcher(&mut self, matcher: Matcher<'p>) {
        self.matchers.push(matcher);
    }
}

impl<'p> Default for HumanizedParser<'p> {
    fn default() -> Self {
        let mut p = HumanizedParser { matchers: vec![] };
        english::register(&mut p);
        p
    }
}
