//! What the C header says its own layouts are.
//!
//! The answers come from `src/offsets.c`, compiled against `include/ic_plugin.h`
//! by the build script. Everything here is a thin way of asking C a question
//! that Rust can then compare with `offset_of!`; the comparing itself is in
//! `tests/layouts_agree.rs`.

use std::ffi::CStr;

/// Which table of the header is being asked about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Table {
    Host = 0,
    Filesystem = 1,
    Window = 2,
    Connection = 3,
    About = 4,
    DirEntry = 5,
    Sizes = 6,
    Viewer = 7,
}

extern "C" {
    fn ic_conformance_count(which: i32) -> usize;
    fn ic_conformance_name(which: i32, at: usize) -> *const std::os::raw::c_char;
    fn ic_conformance_value(which: i32, at: usize) -> usize;
    fn ic_conformance_host_magic() -> u64;
    fn ic_conformance_abi_version() -> u32;
    fn ic_conformance_about_magic() -> u64;
    fn ic_conformance_tree_magic() -> u64;
    fn ic_conformance_drives_magic() -> u64;
    fn ic_conformance_pinned_magic() -> u64;
    fn ic_conformance_columns_magic() -> u64;
    fn ic_conformance_rows_magic() -> u64;
    fn ic_conformance_word_count() -> usize;
    fn ic_conformance_word(at: usize) -> *const std::os::raw::c_char;
}

/// Every field of that table as C lays it out, in the order the header
/// declares them: the name, and the offset or the size.
pub fn as_c_sees_it(table: Table) -> Vec<(String, usize)> {
    let which = table as i32;
    (0..unsafe { ic_conformance_count(which) })
        .map(|at| {
            let name = unsafe { ic_conformance_name(which, at) };
            assert!(!name.is_null(), "the header names nothing at {at}");
            (
                unsafe { CStr::from_ptr(name) }
                    .to_string_lossy()
                    .into_owned(),
                unsafe { ic_conformance_value(which, at) },
            )
        })
        .collect()
}

/// The constants the header defines, so they are compared as well as the
/// layouts: a magic that differs is a plugin the application refuses to talk to.
pub fn constants_in_the_header() -> [(&'static str, u64); 8] {
    [
        ("IC_HOST_MAGIC", unsafe { ic_conformance_host_magic() }),
        ("IC_ABI_VERSION", unsafe { ic_conformance_abi_version() }
            as u64),
        ("IC_ABOUT_MAGIC", unsafe { ic_conformance_about_magic() }),
        ("IC_TREE_MAGIC", unsafe { ic_conformance_tree_magic() }),
        ("IC_DRIVES_MAGIC", unsafe { ic_conformance_drives_magic() }),
        ("IC_PINNED_MAGIC", unsafe { ic_conformance_pinned_magic() }),
        ("IC_COLUMNS_MAGIC", unsafe {
            ic_conformance_columns_magic()
        }),
        ("IC_ROWS_MAGIC", unsafe { ic_conformance_rows_magic() }),
    ]
}

/// The strings the header defines, in the order `src/offsets.c` lists them.
pub fn words_in_the_header() -> Vec<String> {
    (0..unsafe { ic_conformance_word_count() })
        .map(|at| {
            let word = unsafe { ic_conformance_word(at) };
            assert!(!word.is_null(), "the header names no word at {at}");
            unsafe { CStr::from_ptr(word) }
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}
