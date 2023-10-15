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

fn locale_matches(left: Option<&Locale>, right: &Locale) -> bool {
    if *right == Locale::UND {
        true
    } else if let Some(left) = left {
        *left == *right
    } else {
        true
    }
}

impl Parse for bool {
    fn parse(text: &str, locale: Option<&Locale>) -> Option<bool> {
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
    use crate::parse;
    use icu_locid::locale;

    #[test]
    fn basic() {
        assert_eq!(Some(true), parse::<bool>("1", None));
        assert_eq!(Some(true), parse::<bool>("ok", None));
        assert_eq!(Some(true), parse::<bool>("okay", None));
        assert_eq!(Some(true), parse::<bool>("on", None));
        assert_eq!(Some(true), parse::<bool>("true", None));
        assert_eq!(Some(true), parse::<bool>("yep", None));
        assert_eq!(Some(true), parse::<bool>("yes", None));

        assert_eq!(Some(false), parse::<bool>("0", None));
        assert_eq!(Some(false), parse::<bool>("false", None));
        assert_eq!(Some(false), parse::<bool>("no", None));
        assert_eq!(Some(false), parse::<bool>("nope", None));
        assert_eq!(Some(false), parse::<bool>("off", None));

        let bad_locale = locale!("no");
        assert_eq!(Some(true), parse::<bool>("1", Some(&bad_locale)));
        assert_eq!(Some(false), parse::<bool>("0", Some(&bad_locale)));
        assert_eq!(None, parse::<bool>("okay", Some(&bad_locale)));
        assert_eq!(None, parse::<bool>("nope", Some(&bad_locale)));
    }
}
