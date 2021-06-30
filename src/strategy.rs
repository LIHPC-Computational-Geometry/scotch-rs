//! Functions and data structure related to [`Strategy`].

use crate::ErrorCode;
use crate::Result;
use scotch_sys as s;
use std::ffi;
use std::mem;

/// Equivalent of `SCOTCH_Strat`.
pub struct Strategy {
    pub(crate) inner: s::SCOTCH_Strat,
}

impl Strategy {
    /// Equivalent of `SCOTCH_stratInit`.
    pub fn new() -> Strategy {
        let mut inner = mem::MaybeUninit::uninit();

        let inner = unsafe {
            if s::SCOTCH_stratInit(inner.as_mut_ptr()) != 0 {
                panic!("Scotch internal error");
            }
            inner.assume_init()
        };

        Strategy { inner }
    }

    /// Equivalent of `SCOTCH_startGraphPartOvl`.
    ///
    /// # Errors
    ///
    /// This function returns an error when Scotch cannot parse the given
    /// `strategy_string`.
    pub fn graph_part_ovl(&mut self, strategy_string: impl AsRef<ffi::CStr>) -> Result<()> {
        let inner = &mut self.inner as *mut s::SCOTCH_Strat;
        let strategy_string = strategy_string.as_ref().as_ptr();

        unsafe { s::SCOTCH_stratGraphPartOvl(inner, strategy_string).wrap() }
    }
}

impl Drop for Strategy {
    fn drop(&mut self) {
        unsafe {
            let inner = &mut self.inner as *mut s::SCOTCH_Strat;
            s::SCOTCH_stratFree(inner);
        }
    }
}

impl Default for Strategy {
    fn default() -> Strategy {
        Strategy::new()
    }
}
