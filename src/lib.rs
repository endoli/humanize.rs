// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Humanize
//!
//! Make your user interface more human friendly!
//!
//! This library provides functionality for both formatting values
//! into human friendly forms as well as parsing human input to get
//! back likely values.
//!
//! _Actually, the formatting isn't implemented yet. Contributions
//! are welcome!_
//!
//! This library is inspired by many other things, including:
//!
//! * Python's [humanize library].
//! * JavaScript's [moment.js].
//! * The ['at' command]'s input parsing.
//!
//! Contributions extending our functionality are welcome, as are
//! contributions that add support for additional languages.
//!
//! # Human-friendly Parsing
//!
//! When dealing with humans, you often want them to be able to
//! input values in a flexible manner. For example, you might want
//! to be able to input a `bool` using text like `"on"`, `"off"`,
//! `"yes"`, `"no"` or perhaps even `"nope"`.
//!
//! First, you'll want to construct a parser:
//!
//! ```
//! use humanize::HumanizedParser;
//!
//! let parser = HumanizedParser::new();
//! ```
//!
//! Then, you can use that parser to examine some input. In the typical
//! case, you can invoke a type-specific parse method like `parse_boolean`.
//!
//! ```
//! # use humanize::HumanizedParser;
//! #
//! # let parser = HumanizedParser::new();
//! let enabled = parser.parse_boolean("on").unwrap_or(false);
//! assert_eq!(enabled, true);
//! ```
//!
//! The parser stores no state related to an actual parse operation. It
//! simply stores the matchers which have been registered, so this can
//! and should be cached and used across multiple parse operations.
//!
//! # Ideas for the Future
//!
//! * Actually implement formatting.
//! * Color input. Allow inputting a color using a wide variety of color
//!   names, such as those from CSS.
//!
//! [humanize library]: https://pypi.python.org/pypi/humanize
//! [moment.js]: http://momentjs.com/
//! ['at' command]: http://www.computerhope.com/unix/uat.htm

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate language_tags;

pub mod matchers;
mod parser;

pub use parser::HumanizedParser;
