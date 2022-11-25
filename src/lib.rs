// // Copyright 2013-2017, The Gtk-rs Project Developers.
// // See the COPYRIGHT file at the top-level directory of this distribution.
// // Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
#![cfg_attr(feature = "dox", feature(doc_cfg))]

mod auto;
pub use auto::{traits::*, *};
mod global_context_ref;
mod string_ref;
mod value_ref;

pub use global_context_ref::*;
pub use string_ref::*;
pub use value_ref::*;
