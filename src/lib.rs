#![no_std]

pub use self::fmt_debug::{fmt_debug, FmtDebug};
pub use self::fmt_display::{fmt_display, FmtDisplay};
pub use self::fmt_fn::{fmt_fn, FmtFn};
pub use self::fmt_list::{debug_fmt_list, display_fmt_list, FmtList};
pub use self::fmt_map::{debug_fmt_map, display_fmt_map, FmtMap};
pub use self::fmt_set::{debug_fmt_set, display_fmt_set, FmtSet};
pub use self::separated_list::{debug_separated_list, display_separated_list, separated_list, SeparatedList};

#[cfg(test)]
extern crate std;

mod fmt_debug;
mod fmt_display;
mod fmt_fn;
mod fmt_list;
mod fmt_map;
mod fmt_set;
mod separated_list;
