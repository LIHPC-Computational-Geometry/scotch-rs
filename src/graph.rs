use crate::Architecture;
use crate::Error;
use crate::Num;
use crate::Result;
use crate::Strategy;
use scotch_sys as s;
use std::convert::TryFrom as _;
use std::fs;
use std::io;
use std::mem;
use std::os::unix;
use std::path;
use std::ptr;
use std::slice;
use unix::io::IntoRawFd as _;

/// A mapping between a source graph and an architecture.
///
/// Equivalent of `SCOTCH_Mapping`.
pub struct Mapping<'a> {
    inner: s::SCOTCH_Mapping,
    graph: &'a mut Graph,
}

impl<'a> Mapping<'a> {
    /// Equivalent of `SCOTCH_graphMapCompute`.
    ///
    /// # Mutability
    ///
    /// While this function modifies neither the graph nor the strategy, Scotch doesn't specify any
    /// `const` modifier and the Rust borrows must be mutable.
    pub fn compute(&mut self, strategy: &mut Strategy) -> Result<&mut Mapping<'a>> {
        let inner_graph = &mut self.graph.inner as *mut s::SCOTCH_Graph;
        let inner_mapping = &mut self.inner as *mut s::SCOTCH_Mapping;
        let inner_strategy = &mut strategy.inner as *mut s::SCOTCH_Strat;

        unsafe {
            if s::SCOTCH_graphMapCompute(inner_graph, inner_mapping, inner_strategy) != 0 {
                return Err(Error::Other);
            }
        }

        Ok(self)
    }

    /// Equivalent of `SCOTCH_graphMapSave`.
    ///
    /// This function closes the given file descriptor.
    ///
    /// # Safety
    ///
    /// The given file descriptor must be valid for writing and must not be a shared memory object.
    unsafe fn save(&self, fd: unix::io::RawFd) -> io::Result<()> {
        // SAFETY: caller must make sure the file descriptor is valid for writing.
        let file = unsafe { crate::fdopen(fd, "w\0")? };

        let inner_graph = &self.graph.inner as *const s::SCOTCH_Graph;
        let inner_mapping = &self.inner as *const s::SCOTCH_Mapping;

        // SAFETY: file descriptor is valid and inner is initialized.
        unsafe {
            let ret = s::SCOTCH_graphMapSave(inner_graph, inner_mapping, file);
            s::fclose(file);
            if ret != 0 {
                return Err(io::ErrorKind::Other.into());
            }
        }

        Ok(())
    }

    /// Write the mapping to standard output.
    ///
    /// This function closes standard output.
    ///
    /// Convenience wrapper around `SCOTCH_graphMapSave`.
    pub fn write_to_stdout(&self) -> io::Result<()> {
        unsafe { self.save(1) }
    }

    /// Write the mapping to standard error.
    ///
    /// This function closes standard error.
    ///
    /// Convenience wrapper around `SCOTCH_graphMapSave`.
    pub fn write_to_stderr(&self) -> io::Result<()> {
        unsafe { self.save(2) }
    }

    /// Write the mapping to the given file.
    ///
    /// Convenience wrapper around `SCOTCH_graphMapSave`.
    pub fn write_to_file(&self, path: impl AsRef<path::Path>) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let fd = file.into_raw_fd();

        // SAFETY: file is open for writing and is not a shared memory object.
        unsafe { self.save(fd) }
    }
}

impl Drop for Mapping<'_> {
    fn drop(&mut self) {
        unsafe {
            let inner_graph = &self.graph.inner as *const s::SCOTCH_Graph;
            let inner_mapping = &mut self.inner as *mut s::SCOTCH_Mapping;
            s::SCOTCH_graphMapExit(inner_graph, inner_mapping);
        }
    }
}

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
    pub baseval: Num,
    pub verttab: &'a [Num],
    pub vendtab: &'a [Num],
    pub velotab: &'a [Num],
    pub vlbltab: &'a [Num],
    pub edgetab: &'a [Num],
    pub edlotab: &'a [Num],
    _private_field: (), // allow new fields to be added
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
            _private_field: (),
        };
        d.check();
        d
    }

    /// Panic iff the data structure is invalid.
    fn check(&self) {
        assert!(self.baseval == 0 || self.baseval == 1);
        let _ = Num::try_from(self.verttab.len()).expect("verttab is larger than Num::MAX");
        let _ = Num::try_from(self.edgetab.len()).expect("edgetab is larger than Num::MAX");

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

    /// The number of vertices.
    pub fn vertnbr(&self) -> Num {
        let verttab_len = self.verttab.len();
        if self.vendtab.is_empty() {
            (verttab_len - 1) as Num
        } else {
            verttab_len as Num
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
    /// Equivalent of `SCOTCH_graphLoad`.
    ///
    /// This function closes the given file descriptor.
    ///
    /// # Safety
    ///
    /// The given file descriptor must be valid for reading and must not be a shared memory object.
    unsafe fn load(fd: unix::io::RawFd, baseval: Num) -> io::Result<Graph> {
        // SAFETY: caller must make sure the file descriptor is valid for reading.
        let file = unsafe { crate::fdopen(fd, "r\0")? };

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

    /// Build a [Graph] from the data found in standard input.
    ///
    /// This function closes standard input.
    ///
    /// Convenience wrapper around `SCOTCH_graphLoad`.
    pub fn from_stdin(baseval: Num) -> io::Result<Graph> {
        // SAFETY: Standard input is open for reading and is not a shared memory object.
        unsafe { Graph::load(0, baseval) }
    }

    /// Build a [Graph] from the data found in the given file.
    ///
    /// Convenience wrapper around `SCOTCH_graphLoad`.
    pub fn from_file(path: impl AsRef<path::Path>, baseval: Num) -> io::Result<Graph> {
        let file = fs::File::open(path)?;
        let fd = file.into_raw_fd();

        // SAFETY: file is open for reading and is not a shared memory object.
        unsafe { Graph::load(fd, baseval) }
    }

    /// Set the `baseval` and retrieve its old value.
    ///
    /// Equivalent of `SCOTCH_graphBase`.
    ///
    /// # Panics
    ///
    /// This function panics iff `baseval` is neither 0 nor 1.
    pub fn set_base(&mut self, baseval: Num) -> Num {
        assert!(
            baseval == 0 || baseval == 1,
            "baseval must either be 0 or 1"
        );
        let inner = &mut self.inner as *mut s::SCOTCH_Graph;
        unsafe { s::SCOTCH_graphBase(inner, baseval) }
    }

    /// Fill the source graph with data.
    ///
    /// During development stage, it is recommended to call [Graph::check] after calling this
    /// function, to ensure graph data is consistent.
    ///
    /// Equivalent of `SCOTCH_graphBuild`.
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

        // SAFETY: hopefully this function's invariants are enforced by Data.
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

    /// Verify the integrity of the graph.
    ///
    /// Equivalent of `SCOTCH_graphCheck`.
    pub fn check(&self) -> Result<()> {
        let inner = &self.inner as *const s::SCOTCH_Graph;
        let ret_code = unsafe { s::SCOTCH_graphCheck(inner) };
        if ret_code == 0 {
            Ok(())
        } else {
            Err(Error::Other)
        }
    }

    /// The number of vertices and edges in the graph.
    ///
    /// Equivalent of `SCOTCH_graphSize`.
    pub fn size(&self) -> (Num, Num) {
        let inner = &self.inner as *const s::SCOTCH_Graph;
        let mut vertnbr = mem::MaybeUninit::uninit();
        let mut edgenbr = mem::MaybeUninit::uninit();

        // SAFETY: vertnbr and edgenbr are initialized by SCOTCH_graphSize.
        let (vertnbr, edgenbr) = unsafe {
            s::SCOTCH_graphSize(inner, vertnbr.as_mut_ptr(), edgenbr.as_mut_ptr());
            (vertnbr.assume_init(), edgenbr.assume_init())
        };

        #[cfg(debug_assertions)]
        {
            usize::try_from(vertnbr)
                .expect("Scotch internal error: returned a negative graph size");
            usize::try_from(edgenbr)
                .expect("Scotch internal error: returned a negative graph size");
        }

        (vertnbr, edgenbr)
    }

    /// Underlying graph data.
    ///
    /// The resulting `vendtab` should always be non-empty?
    ///
    /// Equivalent of `SCOTCH_graphData`.
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
                _private_field: (),
            }
        };

        #[cfg(debug_assertions)]
        d.check();

        d
    }

    /// Create a [Mapping] between this graph and the given [Architecture].
    ///
    /// Equivalent of `SCOTCH_graphMapInit`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `parttab` is not equal to the number of vertices in
    /// the graph.
    ///
    /// # Mutability
    ///
    /// While this function doesn't modify the graph, Scotch doesn't specify the `const` modifier
    /// and the Rust borrow must be mutable.
    pub fn mapping<'a>(
        &'a mut self,
        architecture: &'a Architecture,
        parttab: &'a mut [Num],
    ) -> Mapping<'a> {
        assert_eq!(
            parttab.len(),
            self.data().vertnbr() as usize,
            "the length of parttab is not the number of vertices"
        );

        let inner_graph = &self.inner as *const s::SCOTCH_Graph;
        let mut inner_mapping = mem::MaybeUninit::uninit();
        let inner_arch = &architecture.inner as *const s::SCOTCH_Arch;

        // SAFETY: inner_mapping is initialized when SCOTCH_graphMapInit returns zero.
        let inner_mapping = unsafe {
            if s::SCOTCH_graphMapInit(
                inner_graph,
                inner_mapping.as_mut_ptr(),
                inner_arch,
                parttab.as_mut_ptr(),
            ) != 0
            {
                panic!("Scotch internal error during mapping initialization");
            }
            inner_mapping.assume_init()
        };

        Mapping {
            inner: inner_mapping,
            graph: self,
        }
    }

    /// Compute a partition with overlap of the given graph structure with respect to the given
    /// strategy.
    ///
    /// Equivalent of `SCOTCH_graphPartOvl`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of parttab is lower than the number of vertices.
    ///
    /// # Mutability
    ///
    /// While this function modifies neither the graph nor the strategy, Scotch doesn't specify any
    /// `const` modifier and the Rust borrows must be mutable.
    pub fn part_ovl(
        &mut self,
        num_parts: Num,
        strategy: &mut Strategy,
        parttab: &mut [Num],
    ) -> Result<()> {
        assert!(
            self.data().vertnbr() as usize <= parttab.len(),
            "parttab's length must be greater or equal to the number of vertices",
        );

        let inner_graph = &mut self.inner as *mut s::SCOTCH_Graph;
        let inner_strat = &mut strategy.inner as *mut s::SCOTCH_Strat;
        let parttab = parttab.as_mut_ptr();

        unsafe {
            if s::SCOTCH_graphPartOvl(inner_graph, num_parts, inner_strat, parttab) != 0 {
                return Err(Error::Other);
            }
        }

        Ok(())
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
