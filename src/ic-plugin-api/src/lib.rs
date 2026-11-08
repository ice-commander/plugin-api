use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

pub const IC_HOST_MAGIC: u64 = 0x4943_5f48_4f53_5400;
pub const IC_ABI_VERSION: u32 = 1;

pub const IC_OK: c_int = 0;
pub const IC_ERR_HOST_TOO_OLD: c_int = 1;
pub const IC_ERR_HOST_UNKNOWN: c_int = 2;
pub const IC_ERR_INIT_FAILED: c_int = 3;
/// A failed filesystem call; the reason is in `last_error`.
pub const IC_ERR_IO: c_int = 4;
/// `init` declining this host. Not reported as a failure.
pub const IC_ERR_NOT_THIS_HOST: c_int = 5;

pub type IcClickFn = extern "C" fn(user_data: *mut c_void, parent_window: *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcSelectionItem {
    pub path: *const c_char,
    pub key: *const c_char,
    pub is_dir: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcSelection {
    pub items: *const IcSelectionItem,
    pub count: u32,
}

impl IcSelection {
    pub const EMPTY: IcSelection = IcSelection {
        items: std::ptr::null(),
        count: 0,
    };

    pub fn as_slice(&self) -> &[IcSelectionItem] {
        if self.items.is_null() || self.count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.items, self.count as usize) }
    }
}

impl IcSelectionItem {
    pub fn path_string(&self) -> Option<String> {
        if self.path.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(self.path) }
                .to_string_lossy()
                .to_string(),
        )
    }

    pub fn key_string(&self) -> Option<String> {
        if self.key.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(self.key) }
                .to_string_lossy()
                .to_string(),
        )
    }

    pub fn is_directory(&self) -> bool {
        self.is_dir != 0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcColumn {
    pub key: *const c_char,
    pub title: *const c_char,
    pub width: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcTable {
    pub columns: *const IcColumn,
    pub column_count: u32,
    pub cells: *const *const c_char,
    pub row_count: u32,
    pub key_column: u32,
}

impl IcTable {
    pub const EMPTY: IcTable = IcTable {
        columns: std::ptr::null(),
        column_count: 0,
        cells: std::ptr::null(),
        row_count: 0,
        key_column: 0,
    };

    pub fn columns_slice(&self) -> &[IcColumn] {
        if self.columns.is_null() || self.column_count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.columns, self.column_count as usize) }
    }

    pub fn cell(&self, row: u32, column: u32) -> Option<String> {
        if self.cells.is_null() || row >= self.row_count || column >= self.column_count {
            return None;
        }
        let index = row as usize * self.column_count as usize + column as usize;
        let ptr = unsafe { *self.cells.add(index) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
    }
}

impl IcColumn {
    pub fn key_string(&self) -> String {
        if self.key.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(self.key) }
            .to_string_lossy()
            .to_string()
    }

    pub fn title_string(&self) -> String {
        if self.title.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(self.title) }
            .to_string_lossy()
            .to_string()
    }
}

pub type IcTableFn = extern "C" fn(user_data: *mut c_void) -> IcTable;

pub const IC_TREE_MAGIC: u64 = 0x4943_5f54_5245_4500;

/// Like `IcTable` plus `enterable`; unlike it, can grow via `struct_size`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcTree {
    pub magic: u64,
    pub struct_size: u32,
    pub column_count: u32,
    pub columns: *const IcColumn,
    pub cells: *const *const c_char,
    pub row_count: u32,
    pub key_column: u32,
    /// Non-zero where the row opens; null means none do.
    pub enterable: *const c_int,
}

impl IcTree {
    pub const EMPTY: IcTree = IcTree {
        magic: IC_TREE_MAGIC,
        struct_size: std::mem::size_of::<IcTree>() as u32,
        column_count: 0,
        columns: std::ptr::null(),
        cells: std::ptr::null(),
        row_count: 0,
        key_column: 0,
        enterable: std::ptr::null(),
    };

    pub fn is_sound(&self) -> bool {
        self.magic == IC_TREE_MAGIC
            && self.struct_size as usize
                >= std::mem::size_of::<IcTree>() - std::mem::size_of::<*const c_int>()
    }

    pub fn columns_slice(&self) -> &[IcColumn] {
        if self.columns.is_null() || self.column_count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.columns, self.column_count as usize) }
    }

    pub fn cell(&self, row: u32, column: u32) -> Option<String> {
        if self.cells.is_null() || row >= self.row_count || column >= self.column_count {
            return None;
        }
        let at = (row * self.column_count + column) as usize;
        let pointer = unsafe { *self.cells.add(at) };
        if pointer.is_null() {
            return Some(String::new());
        }
        Some(
            unsafe { CStr::from_ptr(pointer) }
                .to_string_lossy()
                .to_string(),
        )
    }

    /// False when the plugin's struct ends before `enterable`.
    pub fn opens(&self, row: u32) -> bool {
        let known = std::mem::size_of::<IcTree>();
        if (self.struct_size as usize) < known || self.enterable.is_null() || row >= self.row_count
        {
            return false;
        }
        unsafe { *self.enterable.add(row as usize) != 0 }
    }
}

pub const IC_DRIVES_MAGIC: u64 = 0x4943_5f44_5249_5645;

#[repr(C)]
pub struct IcDrive {
    /// Must stay the same across restarts: favourites hang off it.
    pub key: *const c_char,
    pub name: *const c_char,
    pub subtitle: *const c_char,
    /// Passed to the connection kind's `open` when picked.
    pub settings: *const c_char,
    /// Null or empty uses the icon the source was registered with.
    pub svg: *const c_char,
    pub online: c_int,
}

#[repr(C)]
pub struct IcDrives {
    pub magic: u64,
    pub struct_size: u32,
    pub count: u32,
    pub rows: *const IcDrive,
}

impl IcDrives {
    pub const EMPTY: IcDrives = IcDrives {
        magic: IC_DRIVES_MAGIC,
        struct_size: std::mem::size_of::<IcDrives>() as u32,
        count: 0,
        rows: std::ptr::null(),
    };
}

/// Called on the frontend thread on every redraw; answer from memory, no I/O.
pub type IcDrivesFn = extern "C" fn(user_data: *mut c_void) -> IcDrives;

pub const IC_PINNED_MAGIC: u64 = 0x4943_5f50_494e_4e00;

/// The user can neither rename nor delete it.
#[repr(C)]
pub struct IcPinnedConnection {
    pub id: *const c_char,
    pub title: *const c_char,
    /// Empty uses the application's own icon.
    pub svg: *const c_char,
    /// A `register_view` id; its `describe` gets this entry's `id`.
    pub view: *const c_char,
}

#[repr(C)]
pub struct IcPinnedConnections {
    pub magic: u64,
    pub struct_size: u32,
    pub count: u32,
    pub rows: *const IcPinnedConnection,
}

impl IcPinnedConnections {
    pub const EMPTY: IcPinnedConnections = IcPinnedConnections {
        magic: IC_PINNED_MAGIC,
        struct_size: std::mem::size_of::<IcPinnedConnections>() as u32,
        count: 0,
        rows: std::ptr::null(),
    };
}

/// Called on the frontend thread on every redraw.
pub type IcPinnedConnectionsFn = extern "C" fn(user_data: *mut c_void) -> IcPinnedConnections;

/// `path` is empty at the root, otherwise the row keys joined by `/`.
pub type IcTreeFn = extern "C" fn(path: *const c_char, user_data: *mut c_void) -> IcTree;

/// Called once, on the frontend thread.
pub type IcAnswerFn = extern "C" fn(answer: *const u8, answer_len: u64, user_data: *mut c_void);

pub type IcFsHandle = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcDirEntry {
    pub name: *const c_char,
    pub is_dir: c_int,
    pub size: u64,
    pub modified: u64,
    pub permissions: u32,
    pub has_permissions: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcListing {
    pub items: *const IcDirEntry,
    pub count: u32,
}

/// Pointers a plugin answers with stay its own, valid until its next call on the same handle.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcBytes {
    pub data: *const u8,
    pub len: u64,
}

impl IcListing {
    pub const EMPTY: IcListing = IcListing {
        items: std::ptr::null(),
        count: 0,
    };

    pub fn as_slice(&self) -> &[IcDirEntry] {
        if self.items.is_null() || self.count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.items, self.count as usize) }
    }
}

impl IcBytes {
    pub const EMPTY: IcBytes = IcBytes {
        data: std::ptr::null(),
        len: 0,
    };

    pub fn as_slice(&self) -> &[u8] {
        if self.data.is_null() || self.len == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl IcDirEntry {
    pub fn name_string(&self) -> String {
        if self.name.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(self.name) }
            .to_string_lossy()
            .to_string()
    }

    pub fn is_directory(&self) -> bool {
        self.is_dir != 0
    }

    pub fn permissions_opt(&self) -> Option<u32> {
        if self.has_permissions != 0 {
            Some(self.permissions)
        } else {
            None
        }
    }
}

pub const IC_COLUMNS_MAGIC: u64 = 0x4943_5f43_4f4c_5300;
pub const IC_ROWS_MAGIC: u64 = 0x4943_5f52_4f57_5300;

/// `IcFsColumn::kind`. A check cell is clickable and reported to `cell_clicked`.
pub const IC_COLUMN_TEXT: u32 = 0;
pub const IC_COLUMN_CHECK: u32 = 1;

/// A mount's own column; `IcColumn` is frozen.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcFsColumn {
    pub key: *const c_char,
    pub title: *const c_char,
    pub width: i32,
    /// `IC_COLUMN_TEXT` or `IC_COLUMN_CHECK`.
    pub kind: u32,
}

/// Added after the panel's own columns, never replacing them.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcColumns {
    pub magic: u64,
    pub struct_size: u32,
    pub count: u32,
    pub items: *const IcFsColumn,
}

impl IcFsColumn {
    pub fn key_string(&self) -> String {
        text_at(self.key)
    }

    pub fn title_string(&self) -> String {
        text_at(self.title)
    }

    pub fn is_check(&self) -> bool {
        self.kind == IC_COLUMN_CHECK
    }
}

fn text_at(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
}

/// Anything else reads as unticked.
pub const IC_CELL_TICKED: &str = "1";

pub fn cell_is_ticked(cell: &str) -> bool {
    cell == IC_CELL_TICKED
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcRow {
    pub entry: IcDirEntry,
    /// One string per declared column, in order; a null array or cell reads as empty.
    pub extra: *const *const c_char,
    /// Bounds every read of `extra`.
    pub extra_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcRows {
    pub magic: u64,
    pub struct_size: u32,
    pub count: u32,
    pub items: *const IcRow,
}

impl IcColumns {
    pub const EMPTY: IcColumns = IcColumns {
        magic: IC_COLUMNS_MAGIC,
        struct_size: std::mem::size_of::<IcColumns>() as u32,
        count: 0,
        items: std::ptr::null(),
    };

    pub fn is_sound(&self) -> bool {
        self.magic == IC_COLUMNS_MAGIC
            && self.struct_size as usize >= std::mem::size_of::<IcColumns>()
    }

    pub fn as_slice(&self) -> &[IcFsColumn] {
        if !self.is_sound() || self.items.is_null() || self.count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.items, self.count as usize) }
    }
}

impl IcRows {
    pub const EMPTY: IcRows = IcRows {
        magic: IC_ROWS_MAGIC,
        struct_size: std::mem::size_of::<IcRows>() as u32,
        count: 0,
        items: std::ptr::null(),
    };

    pub fn is_sound(&self) -> bool {
        self.magic == IC_ROWS_MAGIC && self.struct_size as usize >= std::mem::size_of::<IcRows>()
    }

    pub fn as_slice(&self) -> &[IcRow] {
        if !self.is_sound() || self.items.is_null() || self.count == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.items, self.count as usize) }
    }
}

impl IcRow {
    /// Empty past `extra_count`.
    pub fn extra_at(&self, index: usize) -> String {
        if self.extra.is_null() || index >= self.extra_count as usize {
            return String::new();
        }
        let ptr = unsafe { *self.extra.add(index) };
        if ptr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
    }
}

/// `mount` is the one the press happened in; null if the panel has left it since.
pub type IcFsClickFn =
    extern "C" fn(mount: IcFsHandle, user_data: *mut c_void, parent: *mut c_void);

/// `dir` and `name` as `list` spelled them. The host lists again afterwards.
pub type IcFsCellClickedFn = extern "C" fn(
    handle: IcFsHandle,
    dir: *const c_char,
    name: *const c_char,
    column_key: *const c_char,
    ticked: c_int,
) -> c_int;

/// Mask of `IC_ACTION_SHOWN`, `IC_ACTION_ENABLED`, `IC_ACTION_ON`. Called on the frontend
/// thread on every toolbar refresh; answer from memory, no I/O.
pub type IcFsActionStateFn = extern "C" fn(handle: IcFsHandle, action_id: *const c_char) -> u32;

pub type IcFsColumnsFn = extern "C" fn(handle: IcFsHandle) -> IcColumns;
pub type IcFsRowsFn = extern "C" fn(handle: IcFsHandle, path: *const c_char) -> IcRows;

pub type IcFsCloseFn = extern "C" fn(handle: IcFsHandle);
pub type IcFsListFn = extern "C" fn(handle: IcFsHandle, path: *const c_char) -> IcListing;
pub type IcFsReadFn = extern "C" fn(handle: IcFsHandle, path: *const c_char) -> IcBytes;
pub type IcFsFlagFn = extern "C" fn(handle: IcFsHandle) -> c_int;
pub type IcFsErrorFn = extern "C" fn(handle: IcFsHandle) -> *const c_char;
pub type IcFsWriteFn =
    extern "C" fn(handle: IcFsHandle, path: *const c_char, bytes: *const u8, len: u64) -> c_int;
pub type IcFsPathFn = extern "C" fn(handle: IcFsHandle, path: *const c_char) -> c_int;
pub type IcFsRenameFn =
    extern "C" fn(handle: IcFsHandle, from: *const c_char, to: *const c_char) -> c_int;
/// `mode` is the Unix permission bits, `0o7777` at most.
pub type IcFsPermissionsFn =
    extern "C" fn(handle: IcFsHandle, path: *const c_char, mode: u32) -> c_int;

/// The mirror of `IcStream`: that one is the host's, this one the plugin's.
pub type IcFsStream = *mut c_void;

/// One connection and one transfer. `size` is the byte count, 0 when not known.
pub type IcFsStreamOpenFn =
    extern "C" fn(handle: IcFsHandle, path: *const c_char, mode: u32, size: u64) -> IcFsStream;

/// 0 is the end of the file, -1 an error; short of `len` is neither.
pub type IcFsStreamReadFn = extern "C" fn(stream: IcFsStream, into: *mut u8, len: u64) -> i64;

pub type IcFsStreamWriteFn = extern "C" fn(stream: IcFsStream, from: *const u8, len: u64) -> i64;

/// A write may only fail here, on the flush.
pub type IcFsStreamCloseFn = extern "C" fn(stream: IcFsStream) -> c_int;

/// Replaces `stream_close` when the host abandons a write; the destination keeps what it had.
pub type IcFsStreamAbortFn = extern "C" fn(stream: IcFsStream);

/// Copies without the bytes crossing; anything but `IC_OK` falls back to an ordinary copy.
pub type IcFsCopyWithinFn =
    extern "C" fn(handle: IcFsHandle, from: *const c_char, to: *const c_char) -> c_int;

/// The file's real path on disk, for a mount over local files; null when none.
pub type IcFsOnDiskFn = extern "C" fn(handle: IcFsHandle, path: *const c_char) -> *const c_char;

pub type IcShellHandle = *mut c_void;

// One host thread drives all five. `shell_read` must not block: empty when idle,
// null `data` once the shell has ended.
pub type IcShellOpenFn =
    extern "C" fn(handle: IcFsHandle, cwd: *const c_char, rows: u32, cols: u32) -> IcShellHandle;
pub type IcShellReadFn = extern "C" fn(shell: IcShellHandle) -> IcBytes;
pub type IcShellWriteFn = extern "C" fn(shell: IcShellHandle, bytes: *const u8, len: u64) -> c_int;
pub type IcShellResizeFn = extern "C" fn(shell: IcShellHandle, rows: u32, cols: u32) -> c_int;
pub type IcShellCloseFn = extern "C" fn(shell: IcShellHandle);

/// Per mount; absent means every mount has a shell iff `shell_open` is set.
pub type IcShellAvailableFn = extern "C" fn(handle: IcFsHandle) -> c_int;

/// Host-owned and opaque to the plugin.
pub type IcFsSource = *mut c_void;

/// Host-owned.
pub type IcStream = *mut c_void;

/// Opens a mount inside a file on another filesystem: gets its source and path, never its bytes.
pub type IcFsOpenInFn =
    extern "C" fn(source: IcFsSource, path: *const c_char, user_data: *mut c_void) -> IcFsHandle;

/// Ask for no more than needed: a read-only source refuses `IC_OPEN_WRITE`.
pub const IC_OPEN_READ: u32 = 0;
/// Truncates what is there, or makes the file if it is not.
pub const IC_OPEN_WRITE: u32 = 1;
/// Reads and writes without truncating.
pub const IC_OPEN_UPDATE: u32 = 2;

pub const IC_SIZE_UNKNOWN: u64 = 0;

pub const IC_SEEK_SET: u32 = 0;
pub const IC_SEEK_CURRENT: u32 = 1;
pub const IC_SEEK_END: u32 = 2;

/// Costs, not permissions: seeking without `IC_FS_SEEK` still works, the host pays for it.
pub const IC_FS_SEEK: u32 = 1 << 0;
/// Forward reads are cheap; seeking back is not.
pub const IC_FS_SEQUENTIAL: u32 = 1 << 1;
pub const IC_FS_WRITE: u32 = 1 << 2;
pub const IC_FS_LIST: u32 = 1 << 3;
/// `fs_local_path` can answer with a real path on this machine.
pub const IC_FS_LOCAL_PATH: u32 = 1 << 4;

/// Copied by the host; what its pointers point at must stay alive. `Option` slots may be null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcFsVTable {
    pub struct_size: u32,
    pub open_in: IcFsOpenInFn,
    pub close: extern "C" fn(handle: IcFsHandle),
    pub list: extern "C" fn(handle: IcFsHandle, path: *const c_char) -> IcListing,
    pub read: extern "C" fn(handle: IcFsHandle, path: *const c_char) -> IcBytes,
    pub is_read_only: extern "C" fn(handle: IcFsHandle) -> c_int,
    pub last_error: extern "C" fn(handle: IcFsHandle) -> *const c_char,
    pub write: Option<IcFsWriteFn>,
    pub create_dir: Option<IcFsPathFn>,
    pub remove: Option<IcFsPathFn>,
    pub rename: Option<IcFsRenameFn>,
    pub shell_open: Option<IcShellOpenFn>,
    pub shell_read: Option<IcShellReadFn>,
    pub shell_write: Option<IcShellWriteFn>,
    pub shell_resize: Option<IcShellResizeFn>,
    pub shell_close: Option<IcShellCloseFn>,
    pub shell_available: Option<IcShellAvailableFn>,
    /// Asked once per listing, before the rows.
    pub columns: Option<IcFsColumnsFn>,
    /// `list` with the cells for `columns`; preferred when both are set.
    pub list_rows: Option<IcFsRowsFn>,
    /// Absent gives `IC_ACTION_DEFAULT` for every button.
    pub action_state: Option<IcFsActionStateFn>,
    pub cell_clicked: Option<IcFsCellClickedFn>,
    /// Null: no chmod is offered on this filesystem.
    pub set_permissions: Option<IcFsPermissionsFn>,

    // Null `stream_open`: files move whole. Called on the thread that opened the stream;
    // the host closes what it opens.
    pub stream_open: Option<IcFsStreamOpenFn>,
    pub stream_read: Option<IcFsStreamReadFn>,
    pub stream_write: Option<IcFsStreamWriteFn>,
    pub stream_close: Option<IcFsStreamCloseFn>,
    pub copy_within: Option<IcFsCopyWithinFn>,
    pub on_disk: Option<IcFsOnDiskFn>,
    pub stream_abort: Option<IcFsStreamAbortFn>,
}

/// Both answer JSON: a view document, and the reply to an event.
pub type IcViewDescribeFn =
    extern "C" fn(ctx: *const u8, ctx_len: u64, user_data: *mut c_void) -> IcBytes;
pub type IcViewEventFn =
    extern "C" fn(event: *const u8, event_len: u64, user_data: *mut c_void) -> IcBytes;
pub type IcViewClosedFn = extern "C" fn(instance: u64, user_data: *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcViewVTable {
    pub struct_size: u32,
    pub describe: IcViewDescribeFn,
    pub on_event: Option<IcViewEventFn>,
    pub closed: Option<IcViewClosedFn>,
}

// The `type` of an event handed to `on_event`.
pub const IC_EVENT_OPENED: &str = "opened";
/// Also sent for a key from the document's `keys`; `node` names its node.
pub const IC_EVENT_ACTIVATE: &str = "activate";
pub const IC_EVENT_CHANGE: &str = "change";
pub const IC_EVENT_EXPAND: &str = "expand";
pub const IC_EVENT_COLLAPSE: &str = "collapse";
/// A `media` node played to its end; `node` names it.
pub const IC_EVENT_ENDED: &str = "ended";

/// Gets the file's source and path, never its bytes: read through `fs_open`.
pub type IcViewerOpenFn = extern "C" fn(
    instance: u64,
    source: IcFsSource,
    path: *const c_char,
    user_data: *mut c_void,
) -> c_int;

/// Bytes for a `part:<name>` reference; the plugin's until it is asked again.
pub type IcViewerContentFn =
    extern "C" fn(instance: u64, name: *const c_char, user_data: *mut c_void) -> IcBytes;

/// The only `api` so far; a plugin that does not know `api` draws nothing.
pub const IC_CANVAS_GL: u32 = 1;

/// Handed over once the host has a drawing surface; resolve GL symbols via `get_proc_address`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IcCanvas {
    pub struct_size: u32,
    pub api: u32,
    pub get_proc_address: extern "C" fn(ctx: *mut c_void, name: *const c_char) -> *mut c_void,
    pub proc_ctx: *mut c_void,
}

/// Width and height are in pixels, scale already applied.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IcFrame {
    pub struct_size: u32,
    pub fbo: i32,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
}

/// The context is current. Answer `IC_OK` to be drawn.
pub type IcCanvasReadyFn =
    extern "C" fn(instance: u64, canvas: *const IcCanvas, user_data: *mut c_void) -> c_int;

/// Frontend thread only, between `canvas_ready` and `canvas_gone`.
pub type IcCanvasDrawFn =
    extern "C" fn(instance: u64, frame: *const IcFrame, user_data: *mut c_void);

/// Free what `canvas_ready` made; the context is still current.
pub type IcCanvasGoneFn = extern "C" fn(instance: u64, user_data: *mut c_void);

/// A view over one file; its context carries `"instance"`. Pictures are named
/// `file:<path>` (on the viewer's source) or `part:<name>` (from `content`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcViewerVTable {
    pub struct_size: u32,
    pub view: *const IcViewVTable,
    pub open: IcViewerOpenFn,
    /// The source must not be used after this.
    pub closed: Option<IcViewClosedFn>,
    /// Answers `part:` references.
    pub content: Option<IcViewerContentFn>,
    /// Anything but `IC_OK` keeps the window open and describes it again; `closed` follows
    /// a real close.
    pub closing: Option<IcViewerClosingFn>,
    /// The three canvas slots are set or null together.
    pub canvas_ready: Option<IcCanvasReadyFn>,
    pub canvas_draw: Option<IcCanvasDrawFn>,
    pub canvas_gone: Option<IcCanvasGoneFn>,
}

/// `IC_OK` lets the window close.
pub type IcViewerClosingFn = extern "C" fn(instance: u64, user_data: *mut c_void) -> c_int;

pub type IcPlayerOpenFn =
    extern "C" fn(source: IcFsSource, path: *const c_char, user_data: *mut c_void) -> c_int;

/// `of` is 0 while the length is not known.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcPlayedSoFar {
    pub at: f64,
    pub of: f64,
    pub paused: c_int,
    pub ended: c_int,
}

pub type IcPlayerWhereFn = extern "C" fn(user_data: *mut c_void) -> IcPlayedSoFar;
pub type IcPlayerPauseFn = extern "C" fn(paused: c_int, user_data: *mut c_void) -> c_int;
pub type IcPlayerSeekFn = extern "C" fn(to: f64, user_data: *mut c_void) -> c_int;
pub type IcPlayerVolumeFn = extern "C" fn(volume: f64, user_data: *mut c_void) -> c_int;
pub type IcPlayerStopFn = extern "C" fn(user_data: *mut c_void);

/// Only the sound: the list, the titles and the artwork stay with the application.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcPlayerVTable {
    pub struct_size: u32,
    pub open: IcPlayerOpenFn,
    pub played_so_far: IcPlayerWhereFn,
    pub pause: IcPlayerPauseFn,
    pub stop: IcPlayerStopFn,
    pub seek: Option<IcPlayerSeekFn>,
    pub volume: Option<IcPlayerVolumeFn>,
}

/// JSON of strings; the host adds `ic.connect_timeout_secs` and `ic.request_timeout_secs`.
pub type IcConnectionOpenFn =
    extern "C" fn(settings: *const u8, settings_len: u64, user_data: *mut c_void) -> IcFsHandle;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcConnectionVTable {
    pub struct_size: u32,
    pub open: IcConnectionOpenFn,
    pub fs: *const IcFsVTable,
    /// The form to show instead of the registered schema; asked each time it opens.
    /// May run off the frontend thread: only `log_warn` is safe to call back.
    pub describe: Option<IcViewDescribeFn>,
    /// Events from the form, answered like a view (`put`, `set`, `clipboard`, `redescribe`).
    pub on_event: Option<IcViewEventFn>,
}

pub const IC_ENABLE_ALWAYS: u32 = 0;
pub const IC_ENABLE_ON_FILE: u32 = 1 << 0;
pub const IC_ENABLE_ON_DIR: u32 = 1 << 1;
pub const IC_ENABLE_ON_EMPTY: u32 = 1 << 2;

pub fn enabled_for(flags: u32, files: u32, dirs: u32) -> bool {
    if flags == IC_ENABLE_ALWAYS {
        return true;
    }
    if files > 0 && flags & IC_ENABLE_ON_FILE != 0 {
        return true;
    }
    if dirs > 0 && flags & IC_ENABLE_ON_DIR != 0 {
        return true;
    }
    if files == 0 && dirs == 0 && flags & IC_ENABLE_ON_EMPTY != 0 {
        return true;
    }
    false
}

/// `kind` of `register_fs_action`.
pub const IC_ACTION_BUTTON: u32 = 0;
pub const IC_ACTION_TOGGLE: u32 = 1;

/// Bits of the `action_state` answer.
pub const IC_ACTION_SHOWN: u32 = 1 << 0;
pub const IC_ACTION_ENABLED: u32 = 1 << 1;
/// Pressed; only for `IC_ACTION_TOGGLE`.
pub const IC_ACTION_ON: u32 = 1 << 2;

/// Assumed for a mount without `action_state`.
pub const IC_ACTION_DEFAULT: u32 = IC_ACTION_SHOWN | IC_ACTION_ENABLED;

pub const IC_SIDE_LEFT: u32 = 0;
pub const IC_SIDE_RIGHT: u32 = 1;

/// One table per process, outliving every plugin; the pointer may be kept. Slots run on the
/// init thread unless they say otherwise. Grows only at the end: never insert a field.
#[repr(C)]
pub struct IcHost {
    // What a plugin checks before it trusts anything below.
    pub magic: u64,
    pub struct_size: u32,
    pub abi_version: u32,
    pub host_version: extern "C" fn() -> *const c_char,

    // The host itself: what it is, what it says, what it keeps.
    pub log_warn: extern "C" fn(message: *const c_char),
    /// Two-letter code of the UI language; valid for the process.
    pub language: extern "C" fn() -> *const c_char,
    pub register_locales:
        extern "C" fn(language: *const c_char, catalogue: *const u8, catalogue_len: u64) -> c_int,
    /// Referred to as `asset:<plugin_id>/<name>`.
    pub register_plugin_asset: extern "C" fn(
        plugin_id: *const c_char,
        name: *const c_char,
        bytes: *const u8,
        bytes_len: u64,
    ) -> c_int,
    /// Empty when unset. Valid until the next call on this thread.
    pub settings_read: extern "C" fn(plugin_id: *const c_char, key: *const c_char) -> IcBytes,
    /// A null `bytes` deletes the key. `flags`: `IC_SETTING_PLAIN` or `IC_SETTING_SECRET`.
    pub settings_write: extern "C" fn(
        plugin_id: *const c_char,
        key: *const c_char,
        bytes: *const u8,
        len: u64,
        flags: u32,
    ) -> c_int,
    /// `spec`: JSON with `buttons` [{id, label, role}] and optional heading, body, detail, input, choice.
    /// `answered` gets `{"button", "text"?, "choice"?}` once; dismissal reports the first button.
    pub ask: extern "C" fn(
        spec: *const u8,
        spec_len: u64,
        answered: IcAnswerFn,
        user_data: *mut c_void,
    ) -> c_int,

    // The chrome a plugin may add to and hide.
    pub add_toolbar_button: extern "C" fn(
        id: *const c_char,
        svg: *const c_char,
        tooltip: *const c_char,
        side: u32,
        priority: i32,
        enable_flags: u32,
        on_click: IcClickFn,
        user_data: *mut c_void,
    ) -> c_int,
    /// Inside mounts of `extensions` (spelled as in `register_filesystem`). Default: shown.
    pub set_default_toolbar_visible:
        extern "C" fn(extensions: *const c_char, shown: c_int) -> c_int,
    pub add_header_button: extern "C" fn(
        id: *const c_char,
        svg: *const c_char,
        label: *const c_char,
        tooltip: *const c_char,
        side: u32,
        priority: i32,
        on_click: IcClickFn,
        user_data: *mut c_void,
    ) -> c_int,
    /// An empty label leaves only the icon. Callable from any thread.
    pub set_header_label: extern "C" fn(id: *const c_char, label: *const c_char) -> c_int,
    /// Callable from any thread.
    pub set_header_icon: extern "C" fn(id: *const c_char, svg: *const c_char) -> c_int,
    /// Callable from any thread.
    pub set_header_visible: extern "C" fn(id: *const c_char, shown: c_int) -> c_int,

    // Panels: what is in them, and what a plugin may put there.
    pub selection: extern "C" fn() -> IcSelection,
    pub register_panel_source: extern "C" fn(
        id: *const c_char,
        title: *const c_char,
        svg: *const c_char,
        rows: IcTableFn,
        user_data: *mut c_void,
    ) -> c_int,
    pub open_panel_source: extern "C" fn(id: *const c_char) -> c_int,
    pub close_panel_source: extern "C" fn() -> c_int,
    pub register_panel_action: extern "C" fn(
        source_id: *const c_char,
        action_id: *const c_char,
        svg: *const c_char,
        tooltip: *const c_char,
        enable_flags: u32,
        on_click: IcClickFn,
        user_data: *mut c_void,
    ) -> c_int,
    pub register_panel_tree: extern "C" fn(
        id: *const c_char,
        title: *const c_char,
        svg: *const c_char,
        rows: IcTreeFn,
        user_data: *mut c_void,
    ) -> c_int,

    // Where the places in the sidebar come from.
    /// Extra entries in the drives list; picking one calls `open` of connection kind `kind`.
    pub register_drive_source: extern "C" fn(
        kind: *const c_char,
        // One icon for every entry; empty draws reachability instead.
        svg: *const c_char,
        rows: IcDrivesFn,
        user_data: *mut c_void,
    ) -> c_int,
    /// Callable from any thread.
    pub drives_changed: extern "C" fn() -> c_int,
    /// An empty `schema`: the kind only mounts and is never offered for creation.
    pub register_connection_kind: extern "C" fn(
        id: *const c_char,
        schema: *const u8,
        schema_len: u64,
        vtable: *const IcConnectionVTable,
        user_data: *mut c_void,
    ) -> c_int,
    /// Registering the same `id` again replaces the entries.
    pub register_pinned_connections: extern "C" fn(
        id: *const c_char,
        rows: IcPinnedConnectionsFn,
        user_data: *mut c_void,
    ) -> c_int,
    /// Safe to call from a thread of the plugin's own.
    pub pinned_connections_changed: extern "C" fn() -> c_int,

    // Filesystems a plugin brings, and telling the host they moved.
    // The vtable runs on a worker thread; from there only `log_warn` is safe to call back.
    pub register_filesystem: extern "C" fn(
        extensions: *const c_char,
        vtable: *const IcFsVTable,
        user_data: *mut c_void,
    ) -> c_int,
    /// Shown only inside mounts of `extensions` (spelled as in `register_filesystem`).
    pub register_fs_action: extern "C" fn(
        extensions: *const c_char,
        action_id: *const c_char,
        svg: *const c_char,
        tooltip: *const c_char,
        enable_flags: u32,
        // `IC_ACTION_BUTTON` or `IC_ACTION_TOGGLE`.
        kind: u32,
        on_click: IcFsClickFn,
        user_data: *mut c_void,
    ) -> c_int,
    /// Callable from any thread.
    pub fs_invalidate: extern "C" fn(extensions: *const c_char) -> c_int,
    /// Drops what the host caches for `path` on `source`; an empty path means all of it.
    pub fs_changed: extern "C" fn(source: IcFsSource, path: *const c_char) -> c_int,

    // A source from `open_in` lives as long as that mount, used only on that call's thread.
    /// `IC_FS_*` bits: a cost hint, not a permission.
    pub fs_caps: extern "C" fn(source: IcFsSource) -> u32,
    /// `mode` is `IC_OPEN_*`; `size` the byte count, 0 when not known. Null on failure.
    pub fs_open:
        extern "C" fn(source: IcFsSource, path: *const c_char, mode: u32, size: u64) -> IcStream,
    /// 0 is the end, -1 an error; a short read is neither.
    pub fs_read: extern "C" fn(stream: IcStream, into: *mut u8, len: u64) -> i64,
    /// The new position or -1; `whence` is `IC_SEEK_*`. Works without `IC_FS_SEEK`, slower.
    pub fs_seek: extern "C" fn(stream: IcStream, offset: i64, whence: u32) -> i64,
    /// Bytes taken, or -1.
    pub fs_write: extern "C" fn(stream: IcStream, from: *const u8, len: u64) -> i64,
    pub fs_truncate: extern "C" fn(stream: IcStream, len: u64) -> c_int,
    /// Written data is flushed by the time this returns.
    pub fs_close: extern "C" fn(stream: IcStream),
    /// Host-owned rows, valid until the next call on this thread.
    pub fs_list: extern "C" fn(source: IcFsSource, path: *const c_char) -> IcListing,
    /// A real local path, copied if needed and kept as long as the source; null if impossible.
    pub fs_local_path: extern "C" fn(source: IcFsSource, path: *const c_char) -> *const c_char,

    // Documents the host draws on the plugin's behalf.
    pub register_view: extern "C" fn(
        id: *const c_char,
        title: *const c_char,
        vtable: *const IcViewVTable,
        user_data: *mut c_void,
    ) -> c_int,
    pub open_view: extern "C" fn(id: *const c_char, arg: *const u8, arg_len: u64) -> c_int,
    pub view_invalidate: extern "C" fn(id: *const c_char) -> c_int,
    /// Asks for `canvas_draw` on the frontend thread; never draw from a thread of your own.
    pub canvas_invalidate: extern "C" fn(instance: u64) -> c_int,

    /// `extensions` like `".png,.jpg"`, matched by name only. The larger `priority` wins;
    /// the built-in text and hex viewer is always the fallback.
    pub register_viewer: extern "C" fn(
        viewer_id: *const c_char,
        extensions: *const c_char,
        priority: i32,
        vtable: *const IcViewerVTable,
        user_data: *mut c_void,
    ) -> c_int,

    pub screen: extern "C" fn() -> IcScreen,

    /// Same extension list and priority rule as `register_viewer`.
    pub register_player: extern "C" fn(
        player_id: *const c_char,
        extensions: *const c_char,
        priority: i32,
        vtable: *const IcPlayerVTable,
        user_data: *mut c_void,
    ) -> c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IcScreen {
    pub width_dp: u32,
    pub height_dp: u32,
    /// Unreliable on Android; decide by `width_dp`.
    pub scale: f32,
}

pub const IC_SETTING_PLAIN: u32 = 0;
/// Stored in the system secret store.
pub const IC_SETTING_SECRET: u32 = 1 << 0;

/// The `kind` passed to `init`; answer anything but `IC_OK` to decline loading.
pub const IC_HOST_GTK: &str = "gtk";
pub const IC_HOST_WEB: &str = "web";
pub const IC_HOST_CONSOLE: &str = "console";

pub const IC_HOST_ANDROID: &str = "android";

pub fn draws_with_gtk(kind: &str) -> bool {
    kind == IC_HOST_GTK || kind == IC_HOST_ANDROID
}

pub type IcPluginInit = extern "C" fn(host: *const IcHost, kind: *const c_char) -> c_int;

pub const IC_PLUGIN_INIT_SYMBOL: &[u8] = b"ic_plugin_init";

/// Optional. Called once on an orderly exit, with a short time budget the host does not wait past.
pub type IcPluginShutdown = extern "C" fn();

pub const IC_PLUGIN_SHUTDOWN_SYMBOL: &[u8] = b"ic_plugin_shutdown";

pub const IC_ABOUT_MAGIC: u64 = 0x4943_5f41_424f_5554;

/// Read before `init`. Grows only at the end; nothing past `struct_size` is read.
#[repr(C)]
pub struct IcAbout {
    pub magic: u64,
    pub struct_size: u32,
    pub abi_version: u32,
    /// Stable identity, not the file name.
    pub id: *const c_char,
    /// Plain Latin display name, not a translation key.
    pub name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
}

pub type IcPluginAbout = extern "C" fn() -> *const IcAbout;

pub const IC_PLUGIN_ABOUT_SYMBOL: &[u8] = b"ic_plugin_about";

/// Fallbacks where `ic_plugin_about` left a field empty; `ic_plugin_about` wins.
pub type IcPluginText = extern "C" fn() -> *const c_char;

pub const IC_PLUGIN_NAME_SYMBOL: &[u8] = b"ic_plugin_name";
pub const IC_PLUGIN_VERSION_SYMBOL: &[u8] = b"ic_plugin_version";

impl IcAbout {
    /// The pointers must outlive the plugin: use `b"...\0"` literals.
    pub const fn new(
        id: *const c_char,
        name: *const c_char,
        version: *const c_char,
        description: *const c_char,
    ) -> IcAbout {
        IcAbout {
            magic: IC_ABOUT_MAGIC,
            struct_size: std::mem::size_of::<IcAbout>() as u32,
            abi_version: IC_ABI_VERSION,
            id,
            name,
            version,
            description,
        }
    }
}

/// Sound only because [`declare_about!`] fills it with `'static` literals.
#[repr(transparent)]
pub struct AboutSlot(pub IcAbout);

unsafe impl Sync for AboutSlot {}

/// Exports `ic_plugin_about`, `ic_plugin_name` and `ic_plugin_version`.
/// Every argument must expand to a string literal.
#[macro_export]
macro_rules! declare_about {
    ($id:expr, $name:expr, $version:expr, $description:expr) => {
        #[cfg_attr(feature = "export-abi", no_mangle)]
        pub extern "C" fn ic_plugin_about() -> *const $crate::IcAbout {
            static ABOUT: $crate::AboutSlot = $crate::AboutSlot($crate::IcAbout::new(
                concat!($id, "\0").as_ptr() as *const std::os::raw::c_char,
                concat!($name, "\0").as_ptr() as *const std::os::raw::c_char,
                concat!($version, "\0").as_ptr() as *const std::os::raw::c_char,
                concat!($description, "\0").as_ptr() as *const std::os::raw::c_char,
            ));
            &ABOUT.0
        }

        #[cfg_attr(feature = "export-abi", no_mangle)]
        pub extern "C" fn ic_plugin_name() -> *const std::os::raw::c_char {
            concat!($name, "\0").as_ptr() as *const std::os::raw::c_char
        }

        #[cfg_attr(feature = "export-abi", no_mangle)]
        pub extern "C" fn ic_plugin_version() -> *const std::os::raw::c_char {
            concat!($version, "\0").as_ptr() as *const std::os::raw::c_char
        }
    };
}

/// `None` past the `struct_size` the plugin claimed.
pub fn about_field<T: Copy>(about: *const IcAbout, offset: usize) -> Option<T> {
    if about.is_null() {
        return None;
    }
    let raw = about as *const u8;
    let magic = unsafe { std::ptr::read_unaligned(raw as *const u64) };
    if magic != IC_ABOUT_MAGIC {
        return None;
    }
    let claimed = unsafe { std::ptr::read_unaligned(raw.add(8) as *const u32) } as usize;
    let end = offset.checked_add(std::mem::size_of::<T>())?;
    if end > claimed || end > std::mem::size_of::<IcAbout>() {
        return None;
    }
    Some(unsafe { std::ptr::read_unaligned(raw.add(offset) as *const T) })
}

impl IcHost {
    pub fn offset_of_end() -> u32 {
        std::mem::size_of::<IcHost>() as u32
    }
}

#[derive(Clone)]
pub struct BoundedVTable {
    claimed: u32,
    bytes: Vec<u8>,
}

impl BoundedVTable {
    /// # Safety
    /// `vtable` is null or readable for the `u32` size it starts with.
    pub unsafe fn adopt(vtable: *const u8, known: usize) -> Option<BoundedVTable> {
        if vtable.is_null() || known < std::mem::size_of::<u32>() {
            return None;
        }
        let claimed = unsafe { std::ptr::read_unaligned(vtable as *const u32) };
        let usable = (claimed as usize).min(known);
        if usable < std::mem::size_of::<u32>() {
            return None;
        }
        let mut bytes = vec![0u8; known];
        unsafe {
            std::ptr::copy_nonoverlapping(vtable, bytes.as_mut_ptr(), usable);
        }
        Some(BoundedVTable { claimed, bytes })
    }

    pub fn field<T: Copy>(&self, offset: usize) -> Option<T> {
        let end = offset.checked_add(std::mem::size_of::<T>())?;
        if end > self.claimed as usize || end > self.bytes.len() {
            return None;
        }
        Some(unsafe { std::ptr::read_unaligned(self.bytes.as_ptr().add(offset) as *const T) })
    }

    pub fn claimed(&self) -> u32 {
        self.claimed
    }
}

pub fn needs_up_to(offset: usize) -> u32 {
    (offset + std::mem::size_of::<usize>()) as u32
}

pub fn needs_connection_kinds() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, register_connection_kind))
}

pub fn needs_views() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, view_invalidate))
}

pub fn needs_locales() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, register_locales))
}

pub fn needs_settings() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, settings_write))
}

pub fn needs_panel_tree() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, register_panel_tree))
}

pub fn needs_header_label() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, set_header_label))
}

pub fn needs_drive_source() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, drives_changed))
}

pub fn needs_fs_action() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, fs_invalidate))
}

pub fn needs_ask() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, ask))
}

pub fn needs_language() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, language))
}

pub fn needs_header_visible() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, set_header_visible))
}

pub fn needs_own_toolbar() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, set_default_toolbar_visible))
}

pub fn needs_header_icon() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, set_header_icon))
}

pub fn needs_pinned_connections() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, pinned_connections_changed))
}

pub fn needs_canvas() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, canvas_invalidate))
}

pub fn needs_plugin_assets() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, register_plugin_asset))
}

pub fn needs_player() -> u32 {
    needs_up_to(std::mem::offset_of!(IcHost, register_player))
}

pub fn host_has(host: *const IcHost, needs: u32) -> bool {
    if host.is_null() {
        return false;
    }
    let magic = unsafe { std::ptr::read_unaligned(host as *const u64) };
    if magic != IC_HOST_MAGIC {
        return false;
    }
    let struct_size = unsafe { std::ptr::read_unaligned((host as *const u8).add(8) as *const u32) };
    struct_size >= needs
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostCheck {
    Ok,
    WrongMagic,
    TooOld { needs: u32, found: u32 },
    Truncated { needs: u32, found: u32 },
}

pub fn check_host(host: *const IcHost, needs_abi: u32, needs_size: u32) -> HostCheck {
    if host.is_null() {
        return HostCheck::WrongMagic;
    }
    let magic = unsafe { std::ptr::read_unaligned(host as *const u64) };
    if magic != IC_HOST_MAGIC {
        return HostCheck::WrongMagic;
    }
    let struct_size = unsafe { std::ptr::read_unaligned((host as *const u8).add(8) as *const u32) };
    let abi_version =
        unsafe { std::ptr::read_unaligned((host as *const u8).add(12) as *const u32) };
    if abi_version < needs_abi {
        return HostCheck::TooOld {
            needs: needs_abi,
            found: abi_version,
        };
    }
    if struct_size < needs_size {
        return HostCheck::Truncated {
            needs: needs_size,
            found: struct_size,
        };
    }
    HostCheck::Ok
}

/// A do-nothing host table for plugin tests; behind the `testing` feature.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;

    extern "C" fn version() -> *const c_char {
        c"test".as_ptr()
    }
    extern "C" fn warn(_: *const c_char) {}
    extern "C" fn selection() -> IcSelection {
        IcSelection::EMPTY
    }
    extern "C" fn reg_source(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: IcTableFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }
    extern "C" fn open_source(_: *const c_char) -> c_int {
        IC_OK
    }
    extern "C" fn close_source() -> c_int {
        IC_OK
    }
    extern "C" fn add_header(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: u32,
        _: i32,
        _: IcClickFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }
    extern "C" fn reg_fs(_: *const c_char, _: *const IcFsVTable, _: *mut c_void) -> c_int {
        IC_OK
    }
    extern "C" fn reg_action(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: u32,
        _: IcClickFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }
    extern "C" fn add_button(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: u32,
        _: i32,
        _: u32,
        _: IcClickFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn reg_kind(
        _: *const c_char,
        _: *const u8,
        _: u64,
        _: *const IcConnectionVTable,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn reg_view(
        _: *const c_char,
        _: *const c_char,
        _: *const IcViewVTable,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }
    extern "C" fn open_view(_: *const c_char, _: *const u8, _: u64) -> c_int {
        IC_OK
    }
    extern "C" fn invalidate(_: *const c_char) -> c_int {
        IC_OK
    }
    extern "C" fn reg_locales(_: *const c_char, _: *const u8, _: u64) -> c_int {
        IC_OK
    }

    extern "C" fn read_setting(_: *const c_char, _: *const c_char) -> IcBytes {
        IcBytes::EMPTY
    }

    extern "C" fn reg_tree(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: IcTreeFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn set_label(_: *const c_char, _: *const c_char) -> c_int {
        IC_OK
    }

    extern "C" fn drives_moved() -> c_int {
        IC_OK
    }

    extern "C" fn fs_went_stale(_: *const c_char) -> c_int {
        IC_OK
    }

    extern "C" fn speaks() -> *const c_char {
        c"en".as_ptr()
    }

    extern "C" fn show_header(_: *const c_char, _: c_int) -> c_int {
        IC_OK
    }

    extern "C" fn repaint_header(_: *const c_char, _: *const c_char) -> c_int {
        IC_OK
    }

    extern "C" fn show_default_toolbar(_: *const c_char, _: c_int) -> c_int {
        IC_OK
    }

    extern "C" fn reg_fs_action(
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: *const c_char,
        _: u32,
        _: u32,
        _: IcFsClickFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn reg_pinned(_: *const c_char, _: IcPinnedConnectionsFn, _: *mut c_void) -> c_int {
        IC_OK
    }

    extern "C" fn reg_owned_asset(
        _: *const c_char,
        _: *const c_char,
        _: *const u8,
        _: u64,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn reg_drives(
        _: *const c_char,
        _: *const c_char,
        _: IcDrivesFn,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn write_setting(
        _: *const c_char,
        _: *const c_char,
        _: *const u8,
        _: u64,
        _: u32,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn host_fs_caps(_: IcFsSource) -> u32 {
        IC_FS_SEEK | IC_FS_LIST
    }

    extern "C" fn host_fs_open(_: IcFsSource, _: *const c_char, _: u32, _: u64) -> IcStream {
        std::ptr::null_mut()
    }

    extern "C" fn host_fs_read(_: IcStream, _: *mut u8, _: u64) -> i64 {
        0
    }

    extern "C" fn host_fs_seek(_: IcStream, _: i64, _: u32) -> i64 {
        0
    }

    extern "C" fn host_fs_write(_: IcStream, _: *const u8, _: u64) -> i64 {
        -1
    }

    extern "C" fn host_fs_truncate(_: IcStream, _: u64) -> c_int {
        IC_ERR_IO
    }

    extern "C" fn host_fs_close(_: IcStream) {}

    extern "C" fn host_fs_list(_: IcFsSource, _: *const c_char) -> IcListing {
        IcListing::EMPTY
    }

    extern "C" fn host_fs_local_path(_: IcFsSource, _: *const c_char) -> *const c_char {
        std::ptr::null()
    }

    extern "C" fn host_fs_changed(_: IcFsSource, _: *const c_char) -> c_int {
        IC_OK
    }

    extern "C" fn reg_viewer(
        _: *const c_char,
        _: *const c_char,
        _: i32,
        _: *const IcViewerVTable,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn reg_player(
        _: *const c_char,
        _: *const c_char,
        _: i32,
        _: *const IcPlayerVTable,
        _: *mut c_void,
    ) -> c_int {
        IC_OK
    }

    extern "C" fn invalidate_canvas(_: u64) -> c_int {
        IC_OK
    }

    /// Every slot filled, nothing behind them; replace the fields a test needs.
    pub fn silent_host() -> IcHost {
        extern "C" fn ask(
            _spec: *const u8,
            _spec_len: u64,
            _answered: IcAnswerFn,
            _user_data: *mut c_void,
        ) -> c_int {
            IC_OK
        }
        extern "C" fn screen() -> IcScreen {
            IcScreen {
                width_dp: 0,
                height_dp: 0,
                scale: 1.0,
            }
        }
        IcHost {
            ask,
            screen,
            register_player: reg_player,
            magic: IC_HOST_MAGIC,
            struct_size: std::mem::size_of::<IcHost>() as u32,
            abi_version: IC_ABI_VERSION,
            host_version: version,
            log_warn: warn,
            add_toolbar_button: add_button,
            selection,
            register_panel_source: reg_source,
            open_panel_source: open_source,
            close_panel_source: close_source,
            add_header_button: add_header,
            register_filesystem: reg_fs,
            register_panel_action: reg_action,
            register_connection_kind: reg_kind,
            register_view: reg_view,
            open_view,
            view_invalidate: invalidate,
            register_locales: reg_locales,
            settings_read: read_setting,
            settings_write: write_setting,
            register_panel_tree: reg_tree,
            set_header_label: set_label,
            register_drive_source: reg_drives,
            drives_changed: drives_moved,
            register_fs_action: reg_fs_action,
            fs_invalidate: fs_went_stale,
            set_header_visible: show_header,
            set_default_toolbar_visible: show_default_toolbar,
            language: speaks,
            set_header_icon: repaint_header,
            register_pinned_connections: reg_pinned,
            pinned_connections_changed: drives_moved,
            register_plugin_asset: reg_owned_asset,
            fs_caps: host_fs_caps,
            fs_open: host_fs_open,
            fs_read: host_fs_read,
            fs_seek: host_fs_seek,
            fs_write: host_fs_write,
            fs_truncate: host_fs_truncate,
            fs_close: host_fs_close,
            fs_list: host_fs_list,
            fs_local_path: host_fs_local_path,
            fs_changed: host_fs_changed,
            register_viewer: reg_viewer,
            canvas_invalidate: invalidate_canvas,
        }
    }
}
