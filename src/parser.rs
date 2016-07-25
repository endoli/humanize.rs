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
pub struct Parser {
    /// The matchers which have been registered with this parser.
    ///
    /// Use `Parser.register_matcher` to add a new matcher.
    matchers: Vec<Matcher>,

    /// The language being supported by this parser.
    language: LanguageTag,
}

impl Parser {
    /// Construct a new parser, including the default matchers.
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct a new parser, but without any of the default matchers.
    pub fn new_without_default_matchers() -> Self {
        Parser {
            matchers: vec![],
            language: Default::default(),
        }
    }

    /// Parse `text`, looking for a `bool` value.
    pub fn parse_boolean(&self, text: &str) -> Option<bool> {
        match self.parse(text, ValueType::Boolean) {
            Some(HumanValue::Boolean(val)) => Some(val),
            _ => None,
        }
    }

    /// Parse `text`, looking for a value of the [desired type].
    ///
    /// You won't typically invoke this directly, instead preferring to
    /// use type-specific methods like `parse_boolean`.
    ///
    /// [desired type]: matchers/enum.ValueType.html
    pub fn parse(&self, text: &str, desired: ValueType) -> Option<HumanValue> {
        for matcher in &self.matchers {
            if matcher.result_type == desired {
                if let Some(m) = (matcher.matcher)(text) {
                    return Some(m);
                }
            }
        }
        None
    }

    /// Install a new `Matcher` to be used by this parser.
    ///
    /// If the matcher's language does not match the parser's
    /// language, it will not be added.
    ///
    /// The return type indicates whether or not the matcher
    /// was added to the parser.
    pub fn register_matcher(&mut self, matcher: Matcher) -> bool {
        if self.language.matches(&matcher.language) {
            self.matchers.push(matcher);
            true
        } else {
            false
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        let mut p = Parser {
            matchers: vec![],
            language: Default::default(),
        };
        english::register(&mut p);
        p
    }
}
