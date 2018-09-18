#![allow(clippy::match_bool)] // disable false positives
#![warn(clippy::clone_on_ref_ptr, clippy::decimal_literal_representation, clippy::doc_markdown,
        clippy::else_if_without_else, clippy::empty_enum, clippy::enum_glob_use, clippy::expl_impl_clone_on_copy,
        clippy::fallible_impl_from, clippy::filter_map, clippy::if_not_else, clippy::inline_always,
        clippy::invalid_upcast_comparisons, clippy::int_plus_one, clippy::invalid_upcast_comparisons,
        clippy::items_after_statements, clippy::linkedlist, clippy::match_same_arms, clippy::mem_forget,
        /* clippy::multiple_crate_versions /* enable before ship to reconcile multiple crate versions in dependencies */, */
        clippy::mut_mut, clippy::missing_debug_implementations, clippy::mut_mut, clippy::mutex_integer,
        clippy::needless_borrow, clippy::needless_continue, clippy::nonminimal_bool, clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else, clippy::option_map_unwrap_or_else, clippy::option_unwrap_used,
        clippy::pub_enum_variant_names, clippy::range_plus_one, clippy::replace_consts, clippy::redundant_closure,
        clippy::result_map_unwrap_or_else, clippy::result_unwrap_used, clippy::shadow_unrelated,
        /* clippy::similar_names /* suppress `args` false-positive in main() with structopt */, */
        clippy::stutter, clippy::trivial_casts, clippy::non_camel_case_types, clippy::trivial_numeric_casts,
        clippy::unicode_not_nfc, clippy::unseparated_literal_suffix, clippy::use_self, clippy::used_underscore_binding,
        clippy::unused_import_braces, clippy::unnecessary_mut_passed, clippy::unused_must_use,
        clippy::unused_qualifications, clippy::wrong_pub_self_convention)]
// Safety-critical application lints (pedantic--use for safety-critical applications only)
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter)]
#![forbid(overflowing_literals)]
// End of safety-critical lint section
#![feature(tool_lints, try_trait)]

#[macro_use]
extern crate failure;

mod consts;
mod error;
mod game;
#[cfg(test)]
mod unit_tests;

use consts::*;
pub use error::Error;
pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

