// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Boolean Humanization
//!
//! ## Parsing
//!
//! For all languages, we support parsing `"1"` and `"0"` as `true`
//! and `false` respectively.
//!
//! In English, these lexical values map to `true`:
//!
//! * `"ok"`
//! * `"okay"`
//! * `"on"`
//! * `"true"`
//! * `"yep"`
//! * `"yes"`
//!
//! and these to `false`:
//!
//! * `"false"`
//! * `"no"`
//! * `"nope"`
//! * `"off"`

use crate::Parse;
use language_tags::{langtag, LanguageTag};

impl Parse for bool {
    fn parse(text: &str, language: &LanguageTag) -> Option<bool> {
        let en = langtag!(en);
        match &*text.to_lowercase() {
            "1" => Some(true),
            "0" => Some(false),
            "ok" | "okay" | "on" | "true" | "yep" | "yes" if language.matches(&en) => Some(true),
            "false" | "no" | "nope" | "off" if language.matches(&en) => Some(false),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, parse_with_language};
    use language_tags::langtag;

    #[test]
    fn basic() {
        assert_eq!(Some(true), parse::<bool>("1"));
        assert_eq!(Some(true), parse::<bool>("ok"));
        assert_eq!(Some(true), parse::<bool>("okay"));
        assert_eq!(Some(true), parse::<bool>("on"));
        assert_eq!(Some(true), parse::<bool>("true"));
        assert_eq!(Some(true), parse::<bool>("yep"));
        assert_eq!(Some(true), parse::<bool>("yes"));

        assert_eq!(Some(false), parse::<bool>("0"));
        assert_eq!(Some(false), parse::<bool>("false"));
        assert_eq!(Some(false), parse::<bool>("no"));
        assert_eq!(Some(false), parse::<bool>("nope"));
        assert_eq!(Some(false), parse::<bool>("off"));

        let badlang = langtag!("no");
        assert_eq!(Some(true), parse_with_language::<bool>("1", &badlang));
        assert_eq!(Some(false), parse_with_language::<bool>("0", &badlang));
        assert_eq!(None, parse_with_language::<bool>("okay", &badlang));
        assert_eq!(None, parse_with_language::<bool>("nope", &badlang));
    }
}
