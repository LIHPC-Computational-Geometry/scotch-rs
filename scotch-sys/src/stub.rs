/* automatically generated by rust-bindgen 0.65.1 */

pub const SCOTCH_NUMSTRING: &[u8; 3usize] = b"%d\0";
pub const SCOTCH_VERSION: u32 = 6;
pub const SCOTCH_RELEASE: u32 = 1;
pub const SCOTCH_PATCHLEVEL: u32 = 3;
pub const SCOTCH_COARSENNONE: u32 = 0;
pub const SCOTCH_COARSENFOLD: u32 = 256;
pub const SCOTCH_COARSENFOLDDUP: u32 = 768;
pub const SCOTCH_COARSENNOMERGE: u32 = 16384;
pub const SCOTCH_STRATDEFAULT: u32 = 0;
pub const SCOTCH_STRATQUALITY: u32 = 1;
pub const SCOTCH_STRATSPEED: u32 = 2;
pub const SCOTCH_STRATBALANCE: u32 = 4;
pub const SCOTCH_STRATSAFETY: u32 = 8;
pub const SCOTCH_STRATSCALABILITY: u32 = 16;
pub const SCOTCH_STRATRECURSIVE: u32 = 256;
pub const SCOTCH_STRATREMAP: u32 = 512;
pub const SCOTCH_STRATLEVELMAX: u32 = 4096;
pub const SCOTCH_STRATLEVELMIN: u32 = 8192;
pub const SCOTCH_STRATLEAFSIMPLE: u32 = 16384;
pub const SCOTCH_STRATSEPASIMPLE: u32 = 32768;
pub const SCOTCH_STRATDISCONNECTED: u32 = 65536;
pub type FILE = [u64; 27usize];
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
pub type SCOTCH_Idx = ::std::os::raw::c_int;
pub type SCOTCH_Num = ::std::os::raw::c_int;
pub type SCOTCH_GraphPart2 = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Arch {
    pub dummy: [f64; 11usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Arch() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Arch> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Arch>(),
        88usize,
        concat!("Size of: ", stringify!(SCOTCH_Arch))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Arch>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Arch))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Arch),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_ArchDom {
    pub dummy: [f64; 5usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_ArchDom() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_ArchDom> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_ArchDom>(),
        40usize,
        concat!("Size of: ", stringify!(SCOTCH_ArchDom))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_ArchDom>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_ArchDom))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_ArchDom),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Geom {
    pub dummy: [f64; 2usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Geom() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Geom> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Geom>(),
        16usize,
        concat!("Size of: ", stringify!(SCOTCH_Geom))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Geom>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Geom))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Geom),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Graph {
    pub dummy: [f64; 13usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Graph() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Graph> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Graph>(),
        104usize,
        concat!("Size of: ", stringify!(SCOTCH_Graph))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Graph>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Graph))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Graph),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Mesh {
    pub dummy: [f64; 15usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Mesh() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Mesh> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Mesh>(),
        120usize,
        concat!("Size of: ", stringify!(SCOTCH_Mesh))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Mesh>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Mesh))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Mesh),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Mapping {
    pub dummy: [f64; 4usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Mapping() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Mapping> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Mapping>(),
        32usize,
        concat!("Size of: ", stringify!(SCOTCH_Mapping))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Mapping>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Mapping))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Mapping),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Ordering {
    pub dummy: [f64; 12usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Ordering() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Ordering> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Ordering>(),
        96usize,
        concat!("Size of: ", stringify!(SCOTCH_Ordering))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Ordering>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Ordering))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Ordering),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCOTCH_Strat {
    pub dummy: [f64; 1usize],
}
#[test]
fn bindgen_test_layout_SCOTCH_Strat() {
    const UNINIT: ::std::mem::MaybeUninit<SCOTCH_Strat> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SCOTCH_Strat>(),
        8usize,
        concat!("Size of: ", stringify!(SCOTCH_Strat))
    );
    assert_eq!(
        ::std::mem::align_of::<SCOTCH_Strat>(),
        8usize,
        concat!("Alignment of ", stringify!(SCOTCH_Strat))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dummy) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCOTCH_Strat),
            "::",
            stringify!(dummy)
        )
    );
}
extern "C" {
    pub fn SCOTCH_archAlloc() -> *mut SCOTCH_Arch;
}
extern "C" {
    pub fn SCOTCH_archSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archInit(arg1: *mut SCOTCH_Arch) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archExit(arg1: *mut SCOTCH_Arch);
}
extern "C" {
    pub fn SCOTCH_archLoad(arg1: *mut SCOTCH_Arch, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archSave(arg1: *const SCOTCH_Arch, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archBuild(
        arg1: *mut SCOTCH_Arch,
        arg2: *const SCOTCH_Graph,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
        arg5: *const SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archBuild0(
        arg1: *mut SCOTCH_Arch,
        arg2: *const SCOTCH_Graph,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
        arg5: *const SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archBuild2(
        arg1: *mut SCOTCH_Arch,
        arg2: *const SCOTCH_Graph,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archName(arg1: *const SCOTCH_Arch) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn SCOTCH_archSize(arg1: *const SCOTCH_Arch) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_archVar(arg1: *const SCOTCH_Arch) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archCmplt(arg1: *mut SCOTCH_Arch, arg2: SCOTCH_Num) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archCmpltw(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archHcub(arg1: *mut SCOTCH_Arch, arg2: SCOTCH_Num) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archMesh2(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archMesh3(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archMeshX(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archSub(
        arg1: *mut SCOTCH_Arch,
        arg2: *mut SCOTCH_Arch,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archTleaf(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
        arg4: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archTorus2(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archTorus3(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archTorusX(
        arg1: *mut SCOTCH_Arch,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archVcmplt(arg1: *mut SCOTCH_Arch) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archVhcub(arg1: *mut SCOTCH_Arch) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archDomAlloc() -> *mut SCOTCH_ArchDom;
}
extern "C" {
    pub fn SCOTCH_archDomSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archDomNum(arg1: *mut SCOTCH_Arch, arg2: *const SCOTCH_ArchDom) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_archDomTerm(
        arg1: *mut SCOTCH_Arch,
        arg2: *mut SCOTCH_ArchDom,
        arg3: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archDomSize(arg1: *mut SCOTCH_Arch, arg2: *const SCOTCH_ArchDom) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_archDomWght(arg1: *mut SCOTCH_Arch, arg2: *const SCOTCH_ArchDom) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_archDomDist(
        arg1: *mut SCOTCH_Arch,
        arg2: *const SCOTCH_ArchDom,
        arg3: *const SCOTCH_ArchDom,
    ) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_archDomFrst(
        arg1: *mut SCOTCH_Arch,
        arg2: *mut SCOTCH_ArchDom,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_archDomBipart(
        arg1: *mut SCOTCH_Arch,
        arg2: *const SCOTCH_ArchDom,
        arg3: *mut SCOTCH_ArchDom,
        arg4: *mut SCOTCH_ArchDom,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_errorProg(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SCOTCH_errorPrint(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn SCOTCH_errorPrintW(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn SCOTCH_geomAlloc() -> *mut SCOTCH_Geom;
}
extern "C" {
    pub fn SCOTCH_geomSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_geomInit(arg1: *mut SCOTCH_Geom) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_geomExit(arg1: *mut SCOTCH_Geom);
}
extern "C" {
    pub fn SCOTCH_geomData(arg1: *const SCOTCH_Geom, arg2: *mut SCOTCH_Num, arg3: *mut *mut f64);
}
extern "C" {
    pub fn SCOTCH_graphAlloc() -> *mut SCOTCH_Graph;
}
extern "C" {
    pub fn SCOTCH_graphSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphInit(arg1: *mut SCOTCH_Graph) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphExit(arg1: *mut SCOTCH_Graph);
}
extern "C" {
    pub fn SCOTCH_graphFree(arg1: *mut SCOTCH_Graph);
}
extern "C" {
    pub fn SCOTCH_graphLoad(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut FILE,
        arg3: SCOTCH_Num,
        arg4: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphSave(arg1: *const SCOTCH_Graph, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphBuild(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
        arg5: *const SCOTCH_Num,
        arg6: *const SCOTCH_Num,
        arg7: *const SCOTCH_Num,
        arg8: SCOTCH_Num,
        arg9: *const SCOTCH_Num,
        arg10: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphBase(arg1: *mut SCOTCH_Graph, arg2: SCOTCH_Num) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_graphCheck(arg1: *const SCOTCH_Graph) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphCoarsen(
        arg1: *const SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: f64,
        arg4: SCOTCH_Num,
        arg5: *mut SCOTCH_Graph,
        arg6: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphCoarsenMatch(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: f64,
        arg4: SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphCoarsenBuild(
        arg1: *const SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Graph,
        arg5: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphColor(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphData(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut *mut SCOTCH_Num,
        arg5: *mut *mut SCOTCH_Num,
        arg6: *mut *mut SCOTCH_Num,
        arg7: *mut *mut SCOTCH_Num,
        arg8: *mut SCOTCH_Num,
        arg9: *mut *mut SCOTCH_Num,
        arg10: *mut *mut SCOTCH_Num,
    );
}
extern "C" {
    pub fn SCOTCH_graphSize(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
    );
}
extern "C" {
    pub fn SCOTCH_graphStat(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut SCOTCH_Num,
        arg8: *mut SCOTCH_Num,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut SCOTCH_Num,
        arg12: *mut SCOTCH_Num,
        arg13: *mut SCOTCH_Num,
        arg14: *mut f64,
        arg15: *mut f64,
    );
}
extern "C" {
    pub fn SCOTCH_graphDiamPV(arg1: *const SCOTCH_Graph) -> SCOTCH_Num;
}
extern "C" {
    pub fn SCOTCH_graphDump(
        arg1: *const SCOTCH_Graph,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
        arg4: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomLoadChac(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomLoadHabo(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomLoadMmkt(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomLoadScot(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomSaveChac(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomSaveMmkt(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphGeomSaveScot(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphInduceList(
        arg1: *const SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
        arg4: *mut SCOTCH_Graph,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphInducePart(
        arg1: *const SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_GraphPart2,
        arg4: SCOTCH_GraphPart2,
        arg5: *mut SCOTCH_Graph,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapInit(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *const SCOTCH_Arch,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapExit(arg1: *const SCOTCH_Graph, arg2: *mut SCOTCH_Mapping);
}
extern "C" {
    pub fn SCOTCH_graphMapLoad(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapSave(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Mapping,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapCompute(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapFixedCompute(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMap(
        arg1: *mut SCOTCH_Graph,
        arg2: *const SCOTCH_Arch,
        arg3: *mut SCOTCH_Strat,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapFixed(
        arg1: *mut SCOTCH_Graph,
        arg2: *const SCOTCH_Arch,
        arg3: *mut SCOTCH_Strat,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphMapView(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Mapping,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphPart(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Strat,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphPartFixed(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Strat,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphPartOvl(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Strat,
        arg4: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphPartOvlView(
        arg1: *const SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
        arg4: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemap(
        arg1: *mut SCOTCH_Graph,
        arg2: *const SCOTCH_Arch,
        arg3: *mut SCOTCH_Num,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemapFixed(
        arg1: *mut SCOTCH_Graph,
        arg2: *const SCOTCH_Arch,
        arg3: *mut SCOTCH_Num,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemapCompute(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *mut SCOTCH_Mapping,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemapFixedCompute(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Mapping,
        arg3: *mut SCOTCH_Mapping,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemapView(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Mapping,
        arg3: *const SCOTCH_Mapping,
        arg4: f64,
        arg5: *mut SCOTCH_Num,
        arg6: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRemapViewRaw(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Mapping,
        arg3: *const SCOTCH_Mapping,
        arg4: f64,
        arg5: *mut SCOTCH_Num,
        arg6: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRepart(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphRepartFixed(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: f64,
        arg5: *const SCOTCH_Num,
        arg6: *mut SCOTCH_Strat,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphTabLoad(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Num,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphTabSave(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Num,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderInit(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Ordering,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderExit(arg1: *const SCOTCH_Graph, arg2: *mut SCOTCH_Ordering);
}
extern "C" {
    pub fn SCOTCH_graphOrderLoad(
        arg1: *const SCOTCH_Graph,
        arg2: *mut SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderSave(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderSaveMap(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderSaveTree(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderCompute(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Ordering,
        arg3: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderComputeList(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Ordering,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
        arg5: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrder(
        arg1: *mut SCOTCH_Graph,
        arg2: *mut SCOTCH_Strat,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderList(
        arg1: *mut SCOTCH_Graph,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
        arg4: *mut SCOTCH_Strat,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
        arg8: *mut SCOTCH_Num,
        arg9: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_graphOrderCheck(
        arg1: *const SCOTCH_Graph,
        arg2: *const SCOTCH_Ordering,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_mapAlloc() -> *mut SCOTCH_Mapping;
}
extern "C" {
    pub fn SCOTCH_mapSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_memFree(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn SCOTCH_memCur() -> SCOTCH_Idx;
}
extern "C" {
    pub fn SCOTCH_memMax() -> SCOTCH_Idx;
}
extern "C" {
    pub fn SCOTCH_meshAlloc() -> *mut SCOTCH_Mesh;
}
extern "C" {
    pub fn SCOTCH_meshSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshInit(arg1: *mut SCOTCH_Mesh) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshExit(arg1: *mut SCOTCH_Mesh);
}
extern "C" {
    pub fn SCOTCH_meshLoad(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut FILE,
        arg3: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshSave(arg1: *const SCOTCH_Mesh, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshBuild(
        arg1: *mut SCOTCH_Mesh,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: SCOTCH_Num,
        arg5: SCOTCH_Num,
        arg6: *const SCOTCH_Num,
        arg7: *const SCOTCH_Num,
        arg8: *const SCOTCH_Num,
        arg9: *const SCOTCH_Num,
        arg10: *const SCOTCH_Num,
        arg11: SCOTCH_Num,
        arg12: *const SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshCheck(arg1: *const SCOTCH_Mesh) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshSize(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
    );
}
extern "C" {
    pub fn SCOTCH_meshData(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
        arg6: *mut *mut SCOTCH_Num,
        arg7: *mut *mut SCOTCH_Num,
        arg8: *mut *mut SCOTCH_Num,
        arg9: *mut *mut SCOTCH_Num,
        arg10: *mut *mut SCOTCH_Num,
        arg11: *mut SCOTCH_Num,
        arg12: *mut *mut SCOTCH_Num,
        arg13: *mut SCOTCH_Num,
    );
}
extern "C" {
    pub fn SCOTCH_meshStat(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Num,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut SCOTCH_Num,
        arg8: *mut SCOTCH_Num,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut SCOTCH_Num,
        arg12: *mut SCOTCH_Num,
        arg13: *mut f64,
        arg14: *mut f64,
    );
}
extern "C" {
    pub fn SCOTCH_meshGraph(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Graph,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshGraphDual(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Graph,
        arg3: SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshGeomLoadHabo(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshGeomLoadScot(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshGeomSaveScot(
        arg1: *const SCOTCH_Mesh,
        arg2: *const SCOTCH_Geom,
        arg3: *mut FILE,
        arg4: *mut FILE,
        arg5: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderInit(
        arg1: *const SCOTCH_Mesh,
        arg2: *mut SCOTCH_Ordering,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderExit(arg1: *const SCOTCH_Mesh, arg2: *mut SCOTCH_Ordering);
}
extern "C" {
    pub fn SCOTCH_meshOrderSave(
        arg1: *const SCOTCH_Mesh,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderSaveMap(
        arg1: *const SCOTCH_Mesh,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderSaveTree(
        arg1: *const SCOTCH_Mesh,
        arg2: *const SCOTCH_Ordering,
        arg3: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderCompute(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut SCOTCH_Ordering,
        arg3: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderComputeList(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut SCOTCH_Ordering,
        arg3: SCOTCH_Num,
        arg4: *const SCOTCH_Num,
        arg5: *mut SCOTCH_Strat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrder(
        arg1: *mut SCOTCH_Mesh,
        arg2: *mut SCOTCH_Strat,
        arg3: *mut SCOTCH_Num,
        arg4: *mut SCOTCH_Num,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderList(
        arg1: *mut SCOTCH_Mesh,
        arg2: SCOTCH_Num,
        arg3: *const SCOTCH_Num,
        arg4: *mut SCOTCH_Strat,
        arg5: *mut SCOTCH_Num,
        arg6: *mut SCOTCH_Num,
        arg7: *mut SCOTCH_Num,
        arg8: *mut SCOTCH_Num,
        arg9: *mut SCOTCH_Num,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_meshOrderCheck(
        arg1: *const SCOTCH_Mesh,
        arg2: *const SCOTCH_Ordering,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_numSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_orderAlloc() -> *mut SCOTCH_Ordering;
}
extern "C" {
    pub fn SCOTCH_orderSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_randomLoad(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_randomSave(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_randomProc(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SCOTCH_randomReset();
}
extern "C" {
    pub fn SCOTCH_randomSeed(arg1: SCOTCH_Num);
}
extern "C" {
    pub fn SCOTCH_stratAlloc() -> *mut SCOTCH_Strat;
}
extern "C" {
    pub fn SCOTCH_stratSizeof() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratInit(arg1: *mut SCOTCH_Strat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratExit(arg1: *mut SCOTCH_Strat);
}
extern "C" {
    pub fn SCOTCH_stratFree(arg1: *mut SCOTCH_Strat);
}
extern "C" {
    pub fn SCOTCH_stratSave(arg1: *const SCOTCH_Strat, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphBipart(
        arg1: *mut SCOTCH_Strat,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphMap(
        arg1: *mut SCOTCH_Strat,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphMapBuild(
        arg1: *mut SCOTCH_Strat,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphClusterBuild(
        arg1: *mut SCOTCH_Strat,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: f64,
        arg5: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphPartOvl(
        arg1: *mut SCOTCH_Strat,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphPartOvlBuild(
        arg1: *mut SCOTCH_Strat,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphOrder(
        arg1: *mut SCOTCH_Strat,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratGraphOrderBuild(
        arg1: *mut SCOTCH_Strat,
        arg2: SCOTCH_Num,
        arg3: SCOTCH_Num,
        arg4: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratMeshOrder(
        arg1: *mut SCOTCH_Strat,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_stratMeshOrderBuild(
        arg1: *mut SCOTCH_Strat,
        arg2: SCOTCH_Num,
        arg3: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SCOTCH_version(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    );
}
