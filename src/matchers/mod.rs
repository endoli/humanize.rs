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

#[allow(missing_docs)]
pub struct Matcher {
    pub language: LanguageTag,
    pub result_type: ValueType,
    pub matcher: Box<Fn(&str) -> Option<HumanValue>>,
}
