// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Humanize
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
//! * Python's humanize library.
//! * JavaScript's moment.js.
//! * The 'at' command's input parsing.
//!
//! Contributions extending our functionality are welcome, as are
//! contributions that add support for additional languages.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate language_tags;

pub mod parse;
