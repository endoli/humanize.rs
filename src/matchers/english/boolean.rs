// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Booleans - English Humanization

// This is just a dummy example for now. It should clearly be
// much better and actually work correctly. :)

use matchers::*;
use parser::HumanizedParser;

/// Register matchers for English expressions for boolean values.
///
/// This doesn't need to be invoked directly typically. This will
/// be called by `humanize::matchers::english::register`.
pub fn register(parser: &mut HumanizedParser) {
    parser.register_matcher(Matcher {
        name: "English Booleans",
        language: langtag!(en),
        result_type: ValueType::Boolean,
        matcher: Box::new(move |_text: &str| -> Option<Match> {
            Some(Match {
                value: HumanValue::Boolean(true),
                weight: 100,
            })
        }),
    });
}
