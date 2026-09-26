//! A plugin with one of everything the checker knows how to look at: a
//! filesystem, a connection kind with a form, its own words and a picture.
//!
//! It is the checker's own test subject, and the smallest complete answer to
//! "what does a plugin have to export". `plugin-sdk` is where the worked
//! examples live; this one exists so `ic-plugin-check` can be run against
//! something that is known to be right.

// Every entry point below is called from C with raw pointers: that is what the
// boundary is. The lint is right in general and wrong for this whole crate.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ic_plugin_api::{
    check_host, needs_up_to, HostCheck, IcBytes, IcConnectionVTable, IcDirEntry, IcFsHandle,
    IcFsSource, IcFsVTable, IcHost, IcListing, IcViewVTable, IcViewerVTable, IC_ABI_VERSION,
    IC_ERR_HOST_TOO_OLD, IC_ERR_HOST_UNKNOWN, IC_ERR_INIT_FAILED, IC_OPEN_READ, IC_SEEK_END,
};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

ic_plugin_api::declare_about!(
    "ic-example-plugin",
    "Example Plugin",
    "0.1.0",
    "Registers one of everything, for ic-plugin-check to read back"
);

pub const EXTENSIONS: &str = ".example";
pub const KIND: &str = "example";
pub const VIEWER: &str = "example-viewer";

/// The form the host draws when this kind is picked in the connections dialog.
pub const DOCUMENT: &str = include_str!("../documents/example.json");

pub const ENGLISH: &str =
    r#"{ "example.address": "Address", "example.summary": "An example connection" }"#;
pub const RUSSIAN: &str =
    r#"{ "example.address": "Адрес", "example.summary": "Пример подключения" }"#;

pub const PICTURE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7"/></svg>"#;

/// The single row this filesystem shows: the file it was opened in, whole.
const WHOLE: &str = "contents";

/// The table the application handed us at startup, kept because reading a
/// file goes through it.
static HOST: AtomicUsize = AtomicUsize::new(0);

fn host() -> *const IcHost {
    HOST.load(Ordering::Relaxed) as *const IcHost
}

#[derive(Default)]
struct Opened {
    /// The filesystem the opened file lives on, and where on it. Not its
    /// bytes: those are read when they are wanted, however large the file.
    source: IcFsSource,
    path: CString,
    names: Vec<CString>,
    view: Vec<IcDirEntry>,
    bytes: Vec<u8>,
    error: CString,
}

/// The whole of the opened file, read through the host a chunk at a time.
fn read_through_the_host(source: IcFsSource, path: &CStr) -> Option<Vec<u8>> {
    let host = host();
    if host.is_null() {
        return None;
    }
    let stream = unsafe { ((*host).fs_open)(source, path.as_ptr(), IC_OPEN_READ) };
    if stream.is_null() {
        return None;
    }
    let mut held = Vec::new();
    let mut buffer = [0u8; 8 * 1024];
    loop {
        let read = unsafe { ((*host).fs_read)(stream, buffer.as_mut_ptr(), buffer.len() as u64) };
        if read <= 0 {
            break;
        }
        held.extend_from_slice(&buffer[..read as usize]);
    }
    unsafe { ((*host).fs_close)(stream) };
    Some(held)
}

/// How long that file is, asked for without reading any of it.
fn length_of(source: IcFsSource, path: &CStr) -> u64 {
    let host = host();
    if host.is_null() {
        return 0;
    }
    let stream = unsafe { ((*host).fs_open)(source, path.as_ptr(), IC_OPEN_READ) };
    if stream.is_null() {
        return 0;
    }
    let end = unsafe { ((*host).fs_seek)(stream, 0, IC_SEEK_END) };
    unsafe { ((*host).fs_close)(stream) };
    end.max(0) as u64
}

fn with_opened<R>(handle: IcFsHandle, f: impl FnOnce(&mut Opened) -> R) -> Option<R> {
    if handle.is_null() {
        return None;
    }
    let cell = unsafe { &*(handle as *const RefCell<Opened>) };
    let mut held = cell.borrow_mut();
    Some(f(&mut held))
}

fn wanted(path: *const c_char) -> String {
    if path.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(path) }
        .to_string_lossy()
        .trim_matches('/')
        .to_string()
}

extern "C" fn fs_open_in(
    source: IcFsSource,
    path: *const c_char,
    _user_data: *mut c_void,
) -> IcFsHandle {
    let opened = Opened {
        source,
        path: if path.is_null() {
            CString::default()
        } else {
            unsafe { CStr::from_ptr(path) }.to_owned()
        },
        ..Opened::default()
    };
    Box::into_raw(Box::new(RefCell::new(opened))) as IcFsHandle
}

extern "C" fn fs_close(handle: IcFsHandle) {
    if handle.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(handle as *mut RefCell<Opened>) });
}

extern "C" fn fs_list(handle: IcFsHandle, path: *const c_char) -> IcListing {
    let inside = wanted(path);
    with_opened(handle, |opened| {
        if !inside.is_empty() {
            return IcListing::EMPTY;
        }
        // One row for what the file itself holds, and its length asked for
        // rather than measured by reading — the point of the exercise.
        let size = length_of(opened.source, &opened.path);
        opened.names = vec![CString::new(WHOLE).unwrap_or_default()];
        opened.view = vec![IcDirEntry {
            name: opened.names[0].as_ptr(),
            is_dir: 0,
            size,
            modified: 0,
            permissions: 0o644,
            has_permissions: 1,
        }];
        IcListing {
            items: opened.view.as_ptr(),
            count: opened.view.len() as u32,
        }
    })
    .unwrap_or(IcListing::EMPTY)
}

extern "C" fn fs_read(handle: IcFsHandle, path: *const c_char) -> IcBytes {
    let asked = wanted(path);
    with_opened(handle, |opened| {
        if asked != WHOLE {
            opened.error =
                CString::new(format!("no file named {asked:?} in here")).unwrap_or_default();
            return IcBytes::EMPTY;
        }
        match read_through_the_host(opened.source, &opened.path) {
            Some(bytes) => {
                opened.bytes = bytes;
                IcBytes {
                    data: opened.bytes.as_ptr(),
                    len: opened.bytes.len() as u64,
                }
            }
            None => {
                opened.error = CString::new("the host would not open the file").unwrap_or_default();
                IcBytes::EMPTY
            }
        }
    })
    .unwrap_or(IcBytes::EMPTY)
}

extern "C" fn fs_is_read_only(_handle: IcFsHandle) -> c_int {
    1
}

extern "C" fn fs_last_error(handle: IcFsHandle) -> *const c_char {
    with_opened(handle, |opened| opened.error.as_ptr()).unwrap_or(std::ptr::null())
}

pub fn vtable() -> IcFsVTable {
    IcFsVTable {
        struct_size: std::mem::size_of::<IcFsVTable>() as u32,
        open_in: fs_open_in,
        close: fs_close,
        list: fs_list,
        read: fs_read,
        is_read_only: fs_is_read_only,
        last_error: fs_last_error,
        write: None,
        create_dir: None,
        remove: None,
        rename: None,
        shell_open: None,
        shell_read: None,
        shell_write: None,
        shell_resize: None,
        shell_close: None,
        shell_available: None,
        columns: None,
        list_rows: None,
        action_state: None,
        cell_clicked: None,
        set_permissions: None,
    }
}

/// A saved record mounts the same two files, whatever it says in it.
extern "C" fn connection_open(
    _settings: *const u8,
    _settings_len: u64,
    _user_data: *mut c_void,
) -> IcFsHandle {
    Box::into_raw(Box::new(RefCell::new(Opened::default()))) as IcFsHandle
}

/// The host copies what it is handed, so this may live on the caller's stack.
pub fn connection_vtable(fs: *const IcFsVTable) -> IcConnectionVTable {
    IcConnectionVTable {
        struct_size: std::mem::size_of::<IcConnectionVTable>() as u32,
        open: connection_open,
        fs,
        describe: None,
        on_event: None,
    }
}

// What a window is showing: which one it is, the filesystem the file lives on
// and where. One at a time here; a viewer that shows several keeps them by
// `instance`, which is what the host puts in the describe context.
thread_local! {
    static SHOWING: RefCell<Option<(u64, IcFsSource, CString)>> = const { RefCell::new(None) };
    // The document last answered with. The host reads it before asking again,
    // which is the whole of how long it has to live.
    static DRAWN: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Which window the host is asking about, dug out of the context without a
/// JSON parser — a plugin written in C does the same.
fn asking_about(ctx: *const u8, len: u64) -> u64 {
    if ctx.is_null() || len == 0 {
        return 0;
    }
    let context = String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(ctx, len as usize) });
    let Some(after) = context.split(r#""instance""#).nth(1) else {
        return 0;
    };
    after
        .trim_start_matches([':', ' '])
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

extern "C" fn viewer_open(
    instance: u64,
    source: IcFsSource,
    path: *const c_char,
    _user_data: *mut c_void,
) -> c_int {
    let named = if path.is_null() {
        CString::default()
    } else {
        unsafe { CStr::from_ptr(path) }.to_owned()
    };
    SHOWING.with(|held| *held.borrow_mut() = Some((instance, source, named)));
    0
}

/// The window itself: a picture the plugin makes and the length of the file,
/// asked for rather than read. Nothing here carries bytes — the picture is
/// named, and the host asks for it when it draws.
extern "C" fn viewer_describe(ctx: *const u8, len: u64, _user_data: *mut c_void) -> IcBytes {
    let asked = asking_about(ctx, len);
    let size = SHOWING.with(|held| match held.borrow().as_ref() {
        Some((instance, source, path)) if asked == 0 || *instance == asked => {
            length_of(*source, path)
        }
        _ => 0,
    });
    DRAWN.with(|drawn| {
        let mut drawn = drawn.borrow_mut();
        *drawn = format!(
            r#"{{"schema":1,"fields":[],"form":{{"t":"view","surface":"dialog","children":[
            {{"t":"image","id":"picture","src":"part:dot","fit":"contain"}},
            {{"t":"text","id":"size","text":{{"literal":"{size} bytes"}}}}]}}}}"#
        );
        IcBytes {
            data: drawn.as_ptr(),
            len: drawn.len() as u64,
        }
    })
}

/// The pixels behind `part:dot`. They belong to the plugin and are good until
/// it is asked again, so a picture it had to make would be kept here.
extern "C" fn viewer_content(
    _instance: u64,
    name: *const c_char,
    _user_data: *mut c_void,
) -> IcBytes {
    let asked = if name.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .into_owned()
    };
    if asked != "dot" {
        return IcBytes::EMPTY;
    }
    IcBytes {
        data: PICTURE.as_ptr(),
        len: PICTURE.len() as u64,
    }
}

extern "C" fn viewer_closed(instance: u64, _user_data: *mut c_void) {
    SHOWING.with(|held| {
        let mut held = held.borrow_mut();
        if held
            .as_ref()
            .is_some_and(|(shown, _, _)| *shown == instance)
        {
            *held = None;
        }
    });
}

pub fn view_vtable() -> IcViewVTable {
    IcViewVTable {
        struct_size: std::mem::size_of::<IcViewVTable>() as u32,
        describe: viewer_describe,
        on_event: None,
        closed: None,
    }
}

/// The host copies what it is handed, window table and all, so both of these
/// may live on the caller's stack.
pub fn viewer_vtable(view: *const IcViewVTable) -> IcViewerVTable {
    IcViewerVTable {
        struct_size: std::mem::size_of::<IcViewerVTable>() as u32,
        view,
        open: viewer_open,
        closed: Some(viewer_closed),
        content: Some(viewer_content),
        // Nothing to lose: this window closes whenever it is asked to.
        closing: None,
        canvas_ready: None,
        canvas_draw: None,
        canvas_gone: None,
    }
}

#[cfg_attr(feature = "export-abi", no_mangle)]
pub extern "C" fn ic_plugin_init(host: *const IcHost, _kind: *const c_char) -> c_int {
    let needs = needs_up_to(std::mem::offset_of!(IcHost, register_viewer));
    match check_host(host, IC_ABI_VERSION, needs) {
        HostCheck::Ok => {}
        HostCheck::WrongMagic => return IC_ERR_HOST_UNKNOWN,
        HostCheck::TooOld { .. } | HostCheck::Truncated { .. } => return IC_ERR_HOST_TOO_OLD,
    }
    HOST.store(host as usize, Ordering::Relaxed);
    let (Ok(extensions), Ok(kind), Ok(viewer), Ok(id), Ok(picture), Ok(english), Ok(russian)) = (
        CString::new(EXTENSIONS),
        CString::new(KIND),
        CString::new(VIEWER),
        CString::new("ic-example-plugin"),
        CString::new("dot.svg"),
        CString::new("en"),
        CString::new("ru"),
    ) else {
        return IC_ERR_INIT_FAILED;
    };

    let table = vtable();
    unsafe {
        if ((*host).register_filesystem)(extensions.as_ptr(), &table, std::ptr::null_mut()) != 0 {
            return IC_ERR_INIT_FAILED;
        }
        let connection = connection_vtable(&table);
        if ((*host).register_connection_kind)(
            kind.as_ptr(),
            DOCUMENT.as_ptr(),
            DOCUMENT.len() as u64,
            &connection,
            std::ptr::null_mut(),
        ) != 0
        {
            return IC_ERR_INIT_FAILED;
        }
        let window = view_vtable();
        let shows = viewer_vtable(&window);
        if ((*host).register_viewer)(
            viewer.as_ptr(),
            extensions.as_ptr(),
            0,
            &shows,
            std::ptr::null_mut(),
        ) != 0
        {
            return IC_ERR_INIT_FAILED;
        }
        ((*host).register_locales)(english.as_ptr(), ENGLISH.as_ptr(), ENGLISH.len() as u64);
        ((*host).register_locales)(russian.as_ptr(), RUSSIAN.as_ptr(), RUSSIAN.len() as u64);
        ((*host).register_plugin_asset)(
            id.as_ptr(),
            picture.as_ptr(),
            PICTURE.as_ptr(),
            PICTURE.len() as u64,
        );
    }
    0
}

#[cfg_attr(feature = "export-abi", no_mangle)]
pub extern "C" fn ic_plugin_shutdown() {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without a host there is nothing to read through, so the mount answers
    /// an empty listing rather than inventing contents of its own. What it
    /// does with a real filesystem under it is what `ic-plugin-check` drives.
    #[test]
    fn a_mount_with_no_host_behind_it_invents_nothing() {
        let path = CString::new("sample.example").expect("a path");
        let handle = fs_open_in(std::ptr::null_mut(), path.as_ptr(), std::ptr::null_mut());
        let root = CString::new("").expect("a path");

        let listing = fs_list(handle, root.as_ptr());
        assert_eq!(listing.as_slice().len(), 1, "one row for the file itself");
        assert_eq!(listing.as_slice()[0].name_string(), WHOLE);
        assert_eq!(listing.as_slice()[0].size, 0, "nothing was read to say so");

        let asked = CString::new(WHOLE).expect("a path");
        assert_eq!(fs_read(handle, asked.as_ptr()).len, 0);
        fs_close(handle);
    }

    #[test]
    fn what_it_registers_is_spelled_the_way_the_host_reads_it() {
        assert!(EXTENSIONS.starts_with('.'));
        assert!(!KIND.is_empty());
        assert!(serde_json_free_check(DOCUMENT));
        assert!(serde_json_free_check(ENGLISH));
        assert!(serde_json_free_check(RUSSIAN));
    }

    /// The crate has no JSON library of its own on purpose — it is the plugin
    /// side of the boundary, and a plugin pays for what it brings. This is
    /// enough to catch a truncated file; `ic-plugin-check` parses it properly.
    fn serde_json_free_check(text: &str) -> bool {
        let text = text.trim();
        text.starts_with('{') && text.ends_with('}')
    }
}
