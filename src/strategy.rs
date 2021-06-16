use crate::Error;
use crate::Result;
use scotch_sys as s;
use std::ffi;
use std::mem;

pub struct Strategy {
    pub(crate) inner: s::SCOTCH_Strat,
}

impl Strategy {
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

    pub fn graph_part_ovl(&mut self, strategy_string: impl AsRef<ffi::CStr>) -> Result<()> {
        let inner = &mut self.inner as *mut s::SCOTCH_Strat;
        let strategy_string = strategy_string.as_ref().as_ptr();

        unsafe {
            if s::SCOTCH_stratGraphPartOvl(inner, strategy_string) != 0 {
                return Err(Error::Other);
            }
        }

        Ok(())
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
