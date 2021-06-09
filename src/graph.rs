use crate::Error;
use crate::Num;
use crate::Result;
use scotch_sys as s;
use std::fs;
use std::io;
use std::mem;
use std::os::unix;
use std::path;
use std::ptr;

/// Deconstructed graph data.
///
/// # Invariants
///
/// This structure ensures the following invariants are met:
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
pub struct Data<'a> {
    baseval: Num,
    verttab: &'a [Num],
    vendtab: &'a [Num],
    velotab: &'a [Num],
    vlbltab: &'a [Num],
    edgetab: &'a [Num],
    edlotab: &'a [Num],
}

impl<'a> Data<'a> {
    /// Group-up graph data.
    ///
    /// # Panics
    ///
    /// The invariants of [Data] must be uphold, otherwise this function will panic.
    pub fn new(
        baseval: Num,
        verttab: &'a [Num],
        vendtab: &'a [Num],
        velotab: &'a [Num],
        vlbltab: &'a [Num],
        edgetab: &'a [Num],
        edlotab: &'a [Num],
    ) -> Data<'a> {
        let d = Data { baseval, verttab, vendtab, velotab, vlbltab, edgetab, edlotab };
        d.check();
        d
    }

    /// Panic iff the data structure is invalid.
    fn check(&self) {
        assert!(self.baseval == 0 || self.baseval == 1);
        assert!(self.verttab.len() < Num::MAX as usize, "Array too large");
        assert!(self.edgetab.len() < Num::MAX as usize, "Array too large");

        if self.vendtab.is_empty() {
            assert_ne!(self.verttab.len(), 0);
            if !self.velotab.is_empty() {
                assert_eq!(self.verttab.len(), 1 + self.velotab.len());
            }
            if !self.vlbltab.is_empty() {
                assert_eq!(self.verttab.len(), 1 + self.vlbltab.len());
            }
        } else {
            assert_eq!(self.verttab.len(), self.vendtab.len());
            if !self.velotab.is_empty() {
                assert_eq!(self.verttab.len(), self.velotab.len());
            }
            if !self.vlbltab.is_empty() {
                assert_eq!(self.verttab.len(), self.vlbltab.len());
            }
        }
        if !self.edlotab.is_empty() {
            assert_eq!(self.edgetab.len(), self.edlotab.len());
        }
    }
}

pub struct Graph {
    inner: s::SCOTCH_Graph,
}

impl Graph {
    pub fn new() -> Graph {
        let mut inner = mem::MaybeUninit::uninit();

        // SAFETY: inner should be initialized if SCOTCH_graphInit returns zero.
        let inner = unsafe {
            if s::SCOTCH_graphInit(inner.as_mut_ptr()) != 0 {
                panic!("Scotch internal error during graph initialization");
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
    pub fn build(&mut self, data: &Data) -> Result<()> {
        let vendtab = if data.vendtab.is_empty() {
            ptr::null()
        } else {
            data.vendtab.as_ptr()
        };
        let velotab = if data.velotab.is_empty() {
            ptr::null()
        } else {
            data.velotab.as_ptr()
        };
        let vlbltab = if data.vlbltab.is_empty() {
            ptr::null()
        } else {
            data.vlbltab.as_ptr()
        };
        let edlotab = if data.edlotab.is_empty() {
            ptr::null()
        } else {
            data.edlotab.as_ptr()
        };

        let inner = &mut self.inner as *mut s::SCOTCH_Graph;

        // SAFETY: hopefully this function's invariants are enforced with above asserts.
        unsafe {
            let ret_code = s::SCOTCH_graphBuild(
                inner,
                data.baseval,
                data.verttab.len() as Num,
                data.verttab.as_ptr(),
                vendtab,
                velotab,
                vlbltab,
                data.edgetab.len() as Num,
                data.edgetab.as_ptr(),
                edlotab,
            );
            if ret_code != 0 {
                return Err(Error::Other);
            }
        }

        Ok(())
    }

    pub fn check(&self) -> Result<()> {
        let inner = &self.inner as *const s::SCOTCH_Graph;
        let ret_code = unsafe { s::SCOTCH_graphCheck(inner) };
        if ret_code == 0 {
            Ok(())
        } else {
            Err(Error::Other)
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
