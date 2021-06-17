use crate::Num;
use scotch_sys as s;
use std::io;
use std::mem;
use std::os::unix;
use std::path;

pub struct Architecture {
    pub(crate) inner: s::SCOTCH_Arch,
}

impl Architecture {
    /// Equivalent of `SCOTCH_archInit`.
    pub fn new() -> Architecture {
        let mut inner = mem::MaybeUninit::uninit();

        // SAFETY: inner should be initialized if SCOTCH_archInit returns zero.
        let inner = unsafe {
            if s::SCOTCH_archInit(inner.as_mut_ptr()) != 0 {
                panic!("Scotch internal error during architecture initialization");
            }
            inner.assume_init()
        };

        Architecture { inner }
    }

    /// Load an [Architecture] from the given file descriptor.
    ///
    /// This function closes the given file descriptor.
    ///
    /// # Safety
    ///
    /// The given file descriptor must be valid for reading and must not be a shared memory object.
    unsafe fn load(fd: unix::io::RawFd) -> io::Result<Architecture> {
        // SAFETY: caller must make sure the file descriptor is valid for reading.
        let file = unsafe { crate::fdopen(fd, "r\0")? };

        let mut architecture = Architecture::new();
        let inner = &mut architecture.inner as *mut s::SCOTCH_Arch;

        // SAFETY: file descriptor is valid and inner has been initialized.
        unsafe {
            if s::SCOTCH_archLoad(inner, file) != 0 {
                s::fclose(file);
                return Err(io::ErrorKind::Other.into());
            }
            s::fclose(file);
        }

        Ok(architecture)
    }

    pub fn from_stdin() -> io::Result<Architecture> {
        // SAFETY: Standard input is open for reading and is not a shared memory object.
        unsafe { Architecture::load(0) }
    }

    pub fn from_file(path: impl AsRef<path::Path>) -> io::Result<Architecture> {
        use std::fs;
        use unix::io::IntoRawFd as _;

        let file = fs::File::open(path)?;
        let fd = file.into_raw_fd();

        // SAFETY: file is open for reading and is not a shared memory object.
        unsafe { Architecture::load(fd) }
    }

    pub fn complete(partnbr: Num) -> Architecture {
        let mut architecture = Architecture::new();
        let inner = &mut architecture.inner as *mut s::SCOTCH_Arch;

        // SAFETY: inner is initialized.
        unsafe {
            if s::SCOTCH_archCmplt(inner, partnbr) != 0 {
                panic!("Scotch internal error during architecture initialization");
            }
        }

        architecture
    }
}

impl Drop for Architecture {
    fn drop(&mut self) {
        unsafe {
            let inner = &mut self.inner as *mut s::SCOTCH_Arch;
            s::SCOTCH_archExit(inner);
        }
    }
}
