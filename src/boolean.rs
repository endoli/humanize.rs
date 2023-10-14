// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Boolean Humanization
//!
//! ## Parsing
//!
//! For all locales, we support parsing `"1"` and `"0"` as `true`
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
use icu_locid::{locale, Locale};

fn locale_matches(left: &Locale, right: &Locale) -> bool {
    (*left == *right) || (*left == Locale::UND) || (*right == Locale::UND)
}

impl Parse for bool {
    fn parse(text: &str, locale: &Locale) -> Option<bool> {
        let en = locale!("en");
        match &*text.to_lowercase() {
            "1" => Some(true),
            "0" => Some(false),
            "ok" | "okay" | "on" | "true" | "yep" | "yes" if locale_matches(locale, &en) => {
                Some(true)
            }
            "false" | "no" | "nope" | "off" if locale_matches(locale, &en) => Some(false),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, parse_with_locale};
    use icu_locid::locale;

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

        let bad_locale = locale!("no");
        assert_eq!(Some(true), parse_with_locale::<bool>("1", &bad_locale));
        assert_eq!(Some(false), parse_with_locale::<bool>("0", &bad_locale));
        assert_eq!(None, parse_with_locale::<bool>("okay", &bad_locale));
        assert_eq!(None, parse_with_locale::<bool>("nope", &bad_locale));
    }
}
