#![no_std]

//! Provides various tools for creating objects that implements [`Debug`](`core::fmt::Debug`) or
//! [`Display`](`core::fmt::Display`).

pub use self::fmt_debug::{fmt_debug, FmtDebug};
pub use self::fmt_display::{fmt_display, FmtDisplay};
pub use self::fmt_fn::{fmt_fn, FmtFn};
pub use self::fmt_list::{fmt_debug_list, fmt_display_list, FmtDebugList, FmtDisplayList};
pub use self::fmt_map::{fmt_debug_map, fmt_display_map, FmtDebugMap, FmtDisplayMap};
pub use self::fmt_separated_list::{
    fmt_separated_debug_list, fmt_separated_display_list, FmtSeparatedDebugList, FmtSeparatedDisplayList,
};
pub use self::fmt_set::{fmt_debug_set, fmt_display_set, FmtDebugSet, FmtDisplaySet};

#[cfg(test)]
extern crate std;

mod fmt_debug;
mod fmt_display;
mod fmt_fn;
mod fmt_list;
mod fmt_map;
mod fmt_separated_list;
mod fmt_set;
mod format_args_owned;
