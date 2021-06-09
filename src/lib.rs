//! A wrapper around Scotch bindings.

#![allow(unused_unsafe)]

use scotch_sys as s;
use std::fs;
use std::io;
use std::mem;
use std::os::unix;
use std::path;
use std::ptr;

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
        s::SCOTCH_RELEASE
    );
}

/// Scotch's numeral type.
///
/// It is defined as a signed C `int`.  In most platforms it maps to an [i32].  However it can also
/// map to an [i16].  In both cases, most functions and associated constants from both types will
/// work.
pub type Num = s::SCOTCH_Num;

pub struct Graph {
    inner: s::SCOTCH_Graph,
}

impl Graph {
    pub fn new() -> Graph {
        let mut inner = mem::MaybeUninit::uninit();

        // SAFETY: inner should be initialized if SCOTCH_graphInit returns zero.
        let inner = unsafe {
            if s::SCOTCH_graphInit(inner.as_mut_ptr()) != 0 {
                panic!("Scotch internal error");
            }
            inner.assume_init()
        };

        Graph { inner }
    }

    /// Load a [Graph] from the given file descriptor.
    ///
    /// This function closes the given file descriptor.
    ///
    /// # Safety
    ///
    /// The given file descriptor must be valid for reading and must not be a shared memory object.
    unsafe fn load(fd: unix::io::RawFd, baseval: Num) -> io::Result<Graph> {
        // SAFETY: caller must make sure the file descriptor is valid for reading.
        let file = unsafe {
            let file = s::fdopen(fd, "r\0".as_ptr() as *const i8);
            if file.is_null() {
                return Err(io::Error::last_os_error());
            }
            file
        };

        let mut graph = Graph::new();
        let inner = &mut graph.inner as *mut s::SCOTCH_Graph;

        // SAFETY: file descriptor is valid and inner has been initialized.
        unsafe {
            if s::SCOTCH_graphLoad(inner, file, baseval, 0) != 0 {
                s::fclose(file);
                return Err(io::ErrorKind::Other.into());
            }
            s::fclose(file);
        }

        Ok(graph)
    }

    pub fn from_stdin(baseval: Num) -> io::Result<Graph> {
        // SAFETY: Standard input is open for reading and is not a shared memory object.
        unsafe { Graph::load(0, baseval) }
    }

    pub fn from_file(path: impl AsRef<path::Path>, baseval: Num) -> io::Result<Graph> {
        use unix::io::IntoRawFd as _;

        let file = fs::File::open(path)?;
        let fd = file.into_raw_fd();

        // SAFETY: file is open for reading and is not a shared memory object.
        unsafe { Graph::load(fd, baseval) }
    }

    /// Set the `baseval` and retrieve its old value.
    pub fn set_base(&mut self, baseval: Num) -> Num {
        let inner = &mut self.inner as *mut s::SCOTCH_Graph;
        unsafe { s::SCOTCH_graphBase(inner, baseval) }
    }

    /// Fill the source graph with data.
    ///
    /// This function returns `Ok(())` if Scotch successfully filled the graph, and `Err(())` else.
    ///
    /// During development stage, it is recommended to call [Graph::check] after calling this
    /// function, to ensure graph data is consistent.
    ///
    /// # Panics
    ///
    /// This function requires the following:
    ///
    /// - if `vendtab` is empty,
    ///     1. `verttab` must be non-empty,
    ///     2. `velotab`, if non-empty, must have exactly one less element than `verttab`, and
    ///     3. `vlbltab`, if non-empty, must have exactly one less element than `verttab`,
    /// - if `vendtab` is non-empty,
    ///     1. `verttab` and `vendtab` must have the same length,
    ///     2. `velotab`, if non-empty, must have the same length as `verttab`, and
    ///     3. `vlbltab`, if non-empty, must have the same length as `verttab`, and
    /// - `edlotab`, if non-empty, must have the same length as `edgetab`.
    /// - The length of `verttab` must fit in a [Num],
    /// - The length of `edgetab` must fit in a [Num].
    ///
    /// If any of those conditions is not met, this function will panic.
    pub fn build(
        &mut self,
        baseval: Num,
        verttab: &[Num],
        vendtab: &[Num],
        velotab: &[Num],
        vlbltab: &[Num],
        edgetab: &[Num],
        edlotab: &[Num],
    ) -> Result<(), ()> {
        assert!(verttab.len() < Num::MAX as usize, "Array too large");
        assert!(edgetab.len() < Num::MAX as usize, "Array too large");

        if vendtab.is_empty() {
            assert_ne!(verttab.len(), 0);
            if !velotab.is_empty() {
                assert_eq!(verttab.len(), 1 + velotab.len());
            }
            if !vlbltab.is_empty() {
                assert_eq!(verttab.len(), 1 + vlbltab.len());
            }
        } else {
            assert_eq!(verttab.len(), vendtab.len());
            if !velotab.is_empty() {
                assert_eq!(verttab.len(), velotab.len());
            }
            if !vlbltab.is_empty() {
                assert_eq!(verttab.len(), vlbltab.len());
            }
        }
        if !edlotab.is_empty() {
            assert_eq!(edgetab.len(), edlotab.len());
        }

        let vendtab = if vendtab.is_empty() {
            ptr::null()
        } else {
            vendtab.as_ptr()
        };
        let velotab = if velotab.is_empty() {
            ptr::null()
        } else {
            velotab.as_ptr()
        };
        let vlbltab = if vlbltab.is_empty() {
            ptr::null()
        } else {
            vlbltab.as_ptr()
        };
        let edlotab = if edlotab.is_empty() {
            ptr::null()
        } else {
            edlotab.as_ptr()
        };

        let inner = &mut self.inner as *mut s::SCOTCH_Graph;

        // SAFETY: hopefully this function's invariants are enforced with above asserts.
        unsafe {
            let ret_code = s::SCOTCH_graphBuild(
                inner,
                baseval,
                verttab.len() as Num,
                verttab.as_ptr(),
                vendtab,
                velotab,
                vlbltab,
                edgetab.len() as Num,
                edgetab.as_ptr(),
                edlotab,
            );
            if ret_code != 0 {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn check(&self) -> Result<(), ()> {
        let inner = &self.inner as *const s::SCOTCH_Graph;
        let ret_code = unsafe { s::SCOTCH_graphCheck(inner) };
        if ret_code == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            let inner = &mut self.inner as *mut s::SCOTCH_Graph;
            s::SCOTCH_graphFree(inner);
        }
    }
}
