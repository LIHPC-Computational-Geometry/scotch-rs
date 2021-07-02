//! A wrapper around Scotch bindings.
//!
//! This crate provides a thin but idiomatic API for libscotch.
//!
//! # Example
//!
//! Here is an example of graph partitioning:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let partnbr = 3; // divide the graph in three parts.
//! let mut strat = scotch::Strategy::new(); // use the default strategy.
//! let arch = scotch::Architecture::complete(partnbr); // all parts are equal.
//! let mut graph = scotch::Graph::from_file("testdata/grid.grf", -1)?; // load a graph file.
//!
//! // Partition the graph:
//! let (vertnbr, _edgenbr) = graph.size();
//! let mut parttab = vec![0; vertnbr as usize];
//! graph
//!     .mapping(&arch, &mut parttab)
//!     .compute(&mut strat)?
//!     .write_to_stdout()?;
//! # Ok(())
//! # }
//! ```

#![allow(unused_unsafe)]
#![warn(clippy::doc_markdown)]
#![deny(missing_docs)]

pub use crate::architecture::Architecture;
pub use crate::graph::Graph;
pub use crate::strategy::Strategy;
use scotch_sys as s;
use std::fmt;
use std::io;
use std::os;

pub mod architecture;
pub mod graph;
pub mod strategy;

#[cfg(not(unix))]
compile_error!("Scotch is only supported on UNIX platforms");

#[cfg(test)]
#[test]
fn bindings_are_for_the_correct_version_of_scotch() {
    const BINDINGS_VERSION: u32 = 6;
    const BINDINGS_RELEASE: u32 = 0;
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

#[cfg(debug_assertions)]
fn trusted_num_to_usize(n: Num) -> usize {
    use std::convert::TryFrom;
    usize::try_from(n).unwrap_or_else(|_| panic!("Scotch returned a bad size: {}", n))
}

#[cfg(not(debug_assertions))]
fn trusted_num_to_usize(n: Num) -> usize {
    n as usize
}

/// Error type for Scotch functions.
///
/// # Error handling
///
/// Scotch doesn't provide a way to differentiate errors.  When an error is
/// returned by a function, Scotch will typically already have printed an error
/// message on standard error.
///
/// Some errors are hidden from these bindings as panics, that is the Rust
/// function panics if Scotch returns an error. This should only happen when
/// there is a mismatch between Scotch's target architecture and its
/// configuration (e.g. type size mismatch).  These checks are enabled when
/// Scotch is built with debug assertions enabled.
#[derive(Debug)]
#[non_exhaustive]
pub struct Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scotch function returned an error")
    }
}

impl std::error::Error for Error {}

/// Convenience wrapper around [`std::result::Result`] which bears the Scotch
/// error type.
///
/// See [`Error`] for notes on error handling.
pub type Result<T> = std::result::Result<T, Error>;

trait ErrorCode {
    /// Makes a [Result] from a return code (int) from Scotch.
    fn wrap(self) -> Result<()>;
}

impl ErrorCode for os::raw::c_int {
    fn wrap(self) -> Result<()> {
        if self == 0 {
            Ok(())
        } else {
            Err(Error {})
        }
    }
}

/// Convenience wrapper around [`s::fdopen`].
///
/// # Safety
///
/// Two assumptions are made and must be uphold by the caller:
///
/// - `fd` must be valid for reading or writing, depending on the given `mode`,
/// - the `mode` string must end with a nul byte.
unsafe fn fdopen(fd: os::unix::io::RawFd, mode: &str) -> io::Result<*mut s::FILE> {
    let file = s::fdopen(fd, mode.as_ptr() as *const i8);
    if file.is_null() {
        return Err(io::Error::last_os_error());
    }
    Ok(file)
}
