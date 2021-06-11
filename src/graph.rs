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
use std::slice;

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
        let d = Data {
            baseval,
            verttab,
            vendtab,
            velotab,
            vlbltab,
            edgetab,
            edlotab,
        };
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

/// Equivalent of `SCOTCH_Graph`.
pub struct Graph {
    inner: s::SCOTCH_Graph,
}

impl Graph {
    /// Equivalent of `SCOTCH_graphInit`.
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

    /// Underlying graph data.
    pub fn data(&self) -> Data<'_> {
        let mut baseval_raw = mem::MaybeUninit::uninit();
        let mut vertnbr_raw = mem::MaybeUninit::uninit();
        let mut verttab_raw = mem::MaybeUninit::uninit();
        let mut vendtab_raw = mem::MaybeUninit::uninit();
        let mut velotab_raw = mem::MaybeUninit::uninit();
        let mut vlbltab_raw = mem::MaybeUninit::uninit();
        let mut edgenbr_raw = mem::MaybeUninit::uninit();
        let mut edgetab_raw = mem::MaybeUninit::uninit();
        let mut edlotab_raw = mem::MaybeUninit::uninit();

        let inner = &self.inner as *const s::SCOTCH_Graph;

        let d = unsafe {
            s::SCOTCH_graphData(
                inner,
                baseval_raw.as_mut_ptr(),
                vertnbr_raw.as_mut_ptr(),
                verttab_raw.as_mut_ptr(),
                vendtab_raw.as_mut_ptr(),
                velotab_raw.as_mut_ptr(),
                vlbltab_raw.as_mut_ptr(),
                edgenbr_raw.as_mut_ptr(),
                edgetab_raw.as_mut_ptr(),
                edlotab_raw.as_mut_ptr(),
            );

            let baseval_raw = baseval_raw.assume_init();
            let vertnbr_raw = crate::trusted_num_to_usize(vertnbr_raw.assume_init());
            let verttab_raw = verttab_raw.assume_init();
            let vendtab_raw = vendtab_raw.assume_init();
            let velotab_raw = velotab_raw.assume_init();
            let vlbltab_raw = vlbltab_raw.assume_init();
            let edgenbr_raw = crate::trusted_num_to_usize(edgenbr_raw.assume_init());
            let edgetab_raw = edgetab_raw.assume_init();
            let edlotab_raw = edlotab_raw.assume_init();

            let verttab: &[Num];
            let vendtab: &[Num];
            if vendtab_raw.is_null() {
                verttab = slice::from_raw_parts(verttab_raw, vertnbr_raw + 1);
                vendtab = &[];
            } else {
                verttab = slice::from_raw_parts(verttab_raw, vertnbr_raw);
                vendtab = slice::from_raw_parts(vendtab_raw, vertnbr_raw);
            };
            let velotab = if velotab_raw.is_null() {
                &[]
            } else {
                slice::from_raw_parts(velotab_raw, vertnbr_raw)
            };
            let vlbltab = if vlbltab_raw.is_null() {
                &[]
            } else {
                slice::from_raw_parts(vlbltab_raw, vertnbr_raw)
            };
            let edgetab = slice::from_raw_parts(edgetab_raw, edgenbr_raw);
            let edlotab = if edlotab_raw.is_null() {
                &[]
            } else {
                slice::from_raw_parts(edlotab_raw, vertnbr_raw)
            };

            Data {
                baseval: baseval_raw,
                verttab,
                vendtab,
                velotab,
                vlbltab,
                edgetab,
                edlotab,
            }
        };

        #[cfg(debug_assertions)]
        d.check();

        d
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