// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Register Matchers
//!
//! Matchers can be provided to augment the built-in parsing and
//! recognition capabilities of this library.
//!
//! _We will expand upon this in the future once our own infrastructure
//! for doing matchers well is in place._
//!
//! ...

use language_tags::LanguageTag;
use std::time::{Duration, Instant};

pub mod english;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    Boolean,
    Duration,
    Instant,
    Integer,
    Ordinal,
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum HumanValue {
    Boolean(bool),
    Duration(Duration),
    Instant(Instant),
    Integer(i64),
    Ordinal(i64),
}

/// A possible match for a value within some text.
///
/// A `Match` result is obtained by calling [`parse`] on
/// some input text. They are created by [`Matcher`]s.
///
/// [`parse`]: ../struct.HumanizedParser.html#method.parse
/// [`Matcher`]: struct.Matcher.html
#[derive(Debug)]
pub struct Match {
    /// The value determined for this match.
    pub value: HumanValue,

    /// Strength of the match.
    ///
    /// This is useful when there is more than one possible match.
    ///
    /// TODO: Should be this be a percentage? What is the range?
    /// Should it be an enum with values like 'Likely', 'Unlikely',
    /// and 'Certain'?
    pub weight: i32,
}

#[allow(missing_docs)]
pub struct Matcher<'m> {
    pub name: &'m str,
    pub language: LanguageTag,
    pub result_type: ValueType,
    pub matcher: Box<Fn(&str) -> Option<Match>>,
}
