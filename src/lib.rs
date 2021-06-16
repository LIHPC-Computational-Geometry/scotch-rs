//! A wrapper around Scotch bindings.

#![allow(unused_unsafe)]

pub use crate::graph::Graph;
pub use crate::strategy::Strategy;
use scotch_sys as s;
use std::fmt;

pub mod graph;
pub mod strategy;

#[cfg(not(unix))]
compile_error!("Scotch is only supported on UNIX platforms");

#[cfg(test)]
#[test]
fn bindings_are_for_the_correct_version_of_scotch() {
    const BINDINGS_VERSION: u32 = 6;
    const BINDINGS_RELEASE: u32 = 1;
    assert!(
        s::SCOTCH_VERSION == BINDINGS_VERSION && s::SCOTCH_RELEASE == BINDINGS_RELEASE,
        "Rust bindings to Scotch have been made for Scotch {}.{}, your version of Scotch is {}.{}",
        BINDINGS_VERSION,
        BINDINGS_RELEASE,
        s::SCOTCH_VERSION,
        s::SCOTCH_RELEASE,
    );
}

/// Scotch's numeral type.
///
/// It is defined as a signed C `int`.  In most platforms it maps to an [i32].  However it can also
/// map to an [i16].  In both cases, most functions and associated constants from both types will
/// work.
pub type Num = s::SCOTCH_Num;

fn trusted_num_to_usize(n: Num) -> usize {
    use std::convert::TryFrom;

    usize::try_from(n).expect(&format!("Scotch returned a bad size: {}", n))
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Other,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scotch function returned an error")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
