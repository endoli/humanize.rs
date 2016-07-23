// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! English Humanization

use parser::Parser;

pub mod boolean;
// pub mod ordinal;

/// Register all of the English language matchers.
///
/// This doesn't need to be invoked directly typically. This will
/// be called when creating a default `Parser`.
pub fn register(parser: &mut Parser) {
    boolean::register(parser);
}
