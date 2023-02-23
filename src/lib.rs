#![allow(
    // clippy is broken and shows wrong warnings
    // clippy on stable does not know yet about the lint name
    unknown_lints,
    // https://github.com/rust-lang/rust-clippy/issues/8560
    clippy::only_used_in_recursion,
    // https://github.com/rust-lang/rust-clippy/issues/8867
    clippy::derive_partial_eq_without_eq,
    // https://github.com/rust-lang/rust-clippy/issues/9101
    clippy::explicit_auto_deref
)]

pub mod sparsedata;
