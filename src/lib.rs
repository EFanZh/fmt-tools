#![warn(
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::panic_in_result_fn,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies
)]
#![no_std]

//! Provides various tools for creating objects that implement [`Debug`](`core::fmt::Debug`) and
//! [`Display`](`core::fmt::Display`) traits.

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
