/* Ice Commander plugin interface, for plugins written in C.
 *
 * The same boundary `ic-plugin-api` declares in Rust, written out as a C
 * header. The layouts have to agree to the byte: `src/c-conformance` compiles
 * this header and compares every offset with the Rust one, so a field added on
 * one side and forgotten on the other fails the tests rather than corrupting
 * memory at run time.
 *
 * What a plugin exports:
 *
 *     int  ic_plugin_init(const IcHost *host, const char *kind);   required
 *     void ic_plugin_shutdown(void);             optional
 *     const IcAbout *ic_plugin_about(void);      optional but wanted
 *     const char *ic_plugin_name(void);          optional
 *     const char *ic_plugin_version(void);       optional
 *
 * Rules that are not visible in the types:
 *
 *   - the host table is one instance for the process and outlives every
 *     plugin, so the pointer given to init may be kept and called later, from
 *     a thread of the plugin's own. Do not copy the table: it may be longer
 *     than this header knows, and a copy reads past the end of it;
 *   - check `magic`, `abi_version` and `struct_size` before calling anything,
 *     and never read a slot that lies beyond the `struct_size` you were given;
 *   - a vtable the plugin hands over is copied by the host, so it may live on
 *     the stack. What must stay alive is whatever its pointers point at;
 *   - every pointer a plugin answers with stays the plugin's to free, and must
 *     stay valid until the next call on that same handle;
 *   - a slot declared as a pointer to function may be NULL. The host then
 *     refuses that operation instead of calling through.
 */

#ifndef IC_PLUGIN_H
#define IC_PLUGIN_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define IC_HOST_MAGIC 0x49435f484f535400ull
#define IC_ABI_VERSION 1u

/* Which application the plugin was loaded into, given to ic_plugin_init:
   a plugin decides what to register by it, and refuses to load by returning
   anything but IC_OK. */
#define IC_HOST_GTK "gtk"
#define IC_HOST_WEB "web"
#define IC_HOST_CONSOLE "console"

#define IC_OK 0
#define IC_ERR_HOST_TOO_OLD 1
#define IC_ERR_HOST_UNKNOWN 2
#define IC_ERR_INIT_FAILED 3
#define IC_ERR_IO 4
/* init answering "not in this application" — not a failure. */
#define IC_ERR_NOT_THIS_HOST 5

#define IC_SIDE_LEFT 0u
#define IC_SIDE_RIGHT 1u

#define IC_ENABLE_ALWAYS 0u
#define IC_ENABLE_ON_FILE (1u << 0)
#define IC_ENABLE_ON_DIR (1u << 1)
#define IC_ENABLE_ON_EMPTY (1u << 2)

#define IC_ACTION_BUTTON 0u
#define IC_ACTION_TOGGLE 1u
#define IC_ACTION_SHOWN (1u << 0)
#define IC_ACTION_ENABLED (1u << 1)
#define IC_ACTION_ON (1u << 2)
#define IC_ACTION_DEFAULT (IC_ACTION_SHOWN | IC_ACTION_ENABLED)

#define IC_SETTING_PLAIN 0u
#define IC_SETTING_SECRET (1u << 0)

#define IC_COLUMN_TEXT 0u
#define IC_COLUMN_CHECK 1u
#define IC_CELL_TICKED "1"

#define IC_TREE_MAGIC 0x49435f5452454500ull
#define IC_DRIVES_MAGIC 0x49435f4452495645ull
#define IC_PINNED_MAGIC 0x49435f50494e4e00ull
#define IC_COLUMNS_MAGIC 0x49435f434f4c5300ull
#define IC_ROWS_MAGIC 0x49435f524f575300ull
#define IC_ABOUT_MAGIC 0x49435f41424f5554ull

typedef void *IcFsHandle;
typedef void *IcShellHandle;
/* The filesystem a file lives on, and an open read or write upon it. Both
   belong to the host: it alone knows whether what is underneath is a disk, a
   server or another archive. */
typedef void *IcFsSource;
typedef void *IcStream;

/* How a stream is to be used. */
#define IC_OPEN_READ 0u
#define IC_OPEN_WRITE 1u   /* truncates, or makes the file */
#define IC_OPEN_UPDATE 2u  /* reads and writes, keeping what is there */

#define IC_SEEK_SET 0u
#define IC_SEEK_CURRENT 1u
#define IC_SEEK_END 2u

/* What a source does cheaply. These say what a thing COSTS, not what it
   permits: a seek on a source without IC_FS_SEEK still works, and the host
   pays for it. Read them to choose an algorithm, not to ask permission. */
#define IC_FS_SEEK (1u << 0)
#define IC_FS_SEQUENTIAL (1u << 1) /* forwards is cheap, backwards is not */
#define IC_FS_WRITE (1u << 2)
#define IC_FS_LIST (1u << 3)
#define IC_FS_LOCAL_PATH (1u << 4)

typedef void (*IcClickFn)(void *user_data, void *parent_window);

typedef struct IcSelectionItem {
    const char *path;
    const char *key;
    int is_dir;
} IcSelectionItem;

typedef struct IcSelection {
    const IcSelectionItem *items;
    uint32_t count;
} IcSelection;

typedef struct IcColumn {
    const char *key;
    const char *title;
    int32_t width;
} IcColumn;

/* A panel of rows. No header of its own, so this one can never grow. */
typedef struct IcTable {
    const IcColumn *columns;
    uint32_t column_count;
    const char *const *cells;
    uint32_t row_count;
    uint32_t key_column;
} IcTable;

typedef IcTable (*IcTableFn)(void *user_data);

/* The same, plus a flag per row saying whether it opens. */
typedef struct IcTree {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t column_count;
    const IcColumn *columns;
    const char *const *cells;
    uint32_t row_count;
    uint32_t key_column;
    const int *enterable;
} IcTree;

typedef IcTree (*IcTreeFn)(const char *path, void *user_data);

/* How ask reports what the user chose. Called once, on the frontend thread. */
typedef void (*IcAnswerFn)(const uint8_t *answer, uint64_t answer_len, void *user_data);

typedef struct IcDrive {
    const char *key;
    const char *name;
    const char *subtitle;
    const char *settings;
    const char *svg;
    int online;
} IcDrive;

typedef struct IcDrives {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcDrive *rows;
} IcDrives;

typedef IcDrives (*IcDrivesFn)(void *user_data);

typedef struct IcPinnedConnection {
    const char *id;
    const char *title;
    const char *svg;
    const char *view;
} IcPinnedConnection;

typedef struct IcPinnedConnections {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcPinnedConnection *rows;
} IcPinnedConnections;

typedef IcPinnedConnections (*IcPinnedConnectionsFn)(void *user_data);

typedef struct IcDirEntry {
    const char *name;
    int is_dir;
    uint64_t size;
    uint64_t modified;
    uint32_t permissions;
    int has_permissions;
} IcDirEntry;

typedef struct IcListing {
    const IcDirEntry *items;
    uint32_t count;
} IcListing;

typedef struct IcBytes {
    const uint8_t *data;
    uint64_t len;
} IcBytes;

/* A column a filesystem adds to the panel's own. */
typedef struct IcFsColumn {
    const char *key;
    const char *title;
    int32_t width;
    uint32_t kind;
} IcFsColumn;

typedef struct IcColumns {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcFsColumn *items;
} IcColumns;

typedef struct IcRow {
    IcDirEntry entry;
    const char *const *extra;
    uint32_t extra_count;
} IcRow;

typedef struct IcRows {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcRow *items;
} IcRows;

typedef void (*IcFsClickFn)(IcFsHandle mount, void *user_data, void *parent);
typedef int (*IcFsCellClickedFn)(IcFsHandle handle, const char *dir, const char *name,
                                 const char *column_key, int ticked);
typedef uint32_t (*IcFsActionStateFn)(IcFsHandle handle, const char *action_id);
typedef IcColumns (*IcFsColumnsFn)(IcFsHandle handle);
typedef IcRows (*IcFsRowsFn)(IcFsHandle handle, const char *path);
/* Opened inside a file of another filesystem: an archive, a torrent. What is
   handed over is the filesystem that file lives on and the path it lives at —
   never its bytes. */
typedef IcFsHandle (*IcFsOpenInFn)(IcFsSource source, const char *path, void *user_data);
typedef void (*IcFsCloseFn)(IcFsHandle handle);
typedef IcListing (*IcFsListFn)(IcFsHandle handle, const char *path);
typedef IcBytes (*IcFsReadFn)(IcFsHandle handle, const char *path);
typedef int (*IcFsFlagFn)(IcFsHandle handle);
typedef const char *(*IcFsErrorFn)(IcFsHandle handle);
typedef int (*IcFsWriteFn)(IcFsHandle handle, const char *path, const uint8_t *bytes,
                           uint64_t len);
typedef int (*IcFsPathFn)(IcFsHandle handle, const char *path);
typedef int (*IcFsRenameFn)(IcFsHandle handle, const char *from, const char *to);
/* `mode` is the Unix permission bits, 07777 at most. */
typedef int (*IcFsPermissionsFn)(IcFsHandle handle, const char *path, uint32_t mode);
typedef IcShellHandle (*IcShellOpenFn)(IcFsHandle handle, const char *cwd, uint32_t rows,
                                       uint32_t cols);
typedef IcBytes (*IcShellReadFn)(IcShellHandle shell);
typedef int (*IcShellWriteFn)(IcShellHandle shell, const uint8_t *bytes, uint64_t len);
typedef int (*IcShellResizeFn)(IcShellHandle shell, uint32_t rows, uint32_t cols);
typedef void (*IcShellCloseFn)(IcShellHandle shell);
typedef int (*IcShellAvailableFn)(IcFsHandle handle);

/* Everything from `write` down may be NULL. */
typedef struct IcFsVTable {
    uint32_t struct_size;
    IcFsOpenInFn open_in;
    IcFsCloseFn close;
    IcFsListFn list;
    IcFsReadFn read;
    IcFsFlagFn is_read_only;
    IcFsErrorFn last_error;
    IcFsWriteFn write;
    IcFsPathFn create_dir;
    IcFsPathFn remove;
    IcFsRenameFn rename;
    IcShellOpenFn shell_open;
    IcShellReadFn shell_read;
    IcShellWriteFn shell_write;
    IcShellResizeFn shell_resize;
    IcShellCloseFn shell_close;
    IcShellAvailableFn shell_available;
    IcFsColumnsFn columns;
    IcFsRowsFn list_rows;
    IcFsActionStateFn action_state;
    IcFsCellClickedFn cell_clicked;
    /* NULL means this filesystem cannot change permissions, and the host offers no chmod on it. */
    IcFsPermissionsFn set_permissions;
} IcFsVTable;

/* Both answer JSON: a view document, and the reply to an event. */
typedef IcBytes (*IcViewDescribeFn)(const uint8_t *ctx, uint64_t ctx_len, void *user_data);
typedef IcBytes (*IcViewEventFn)(const uint8_t *event, uint64_t event_len, void *user_data);
typedef void (*IcViewClosedFn)(uint64_t instance, void *user_data);

typedef struct IcViewVTable {
    uint32_t struct_size;
    IcViewDescribeFn describe;
    IcViewEventFn on_event; /* may be NULL */
    IcViewClosedFn closed;  /* may be NULL */
} IcViewVTable;

/* The "type" of an event handed to on_event. */
#define IC_EVENT_OPENED "opened"
/* Also what a key from the document's "keys" arrives as, with "node" naming that key's node. */
#define IC_EVENT_ACTIVATE "activate"
#define IC_EVENT_CHANGE "change"
#define IC_EVENT_EXPAND "expand"
#define IC_EVENT_COLLAPSE "collapse"
/* A media node played to its end; "node" names it. */
#define IC_EVENT_ENDED "ended"

/* A viewer opened on one file: which window it is, the filesystem the file
   lives on, and where on it. Never the bytes — the plugin reads what it needs
   through fs_open and the calls beside it. */
typedef int (*IcViewerOpenFn)(uint64_t instance, IcFsSource source, const char *path,
                              void *user_data);
/* Pixels the plugin made rather than read: a page of a document, the picture
   inside a camera file. Asked for when the application draws, so a plugin
   holding a thousand-page document holds no pages. `name` is what the document
   called it after `part:`. */
typedef IcBytes (*IcViewerContentFn)(uint64_t instance, const char *name, void *user_data);

/* The describe and event context of a viewer carries "instance", so a plugin
   showing two files at once knows which of them is being asked about. */

/* What a plugin shows when the user asks to look at a file: a view with a file
   behind it. The document it answers with names its pictures —
   `file:<path>` for what is on the filesystem it was opened on,
   `part:<name>` for what the plugin made — and never carries them. */
/* Asked before a viewer's window closes: IC_OK lets it go, anything else keeps
   it open and the window is described again, so a plugin that refuses can say
   why in the same window. */
typedef int (*IcViewerClosingFn)(uint64_t instance, void *user_data);

/* The only drawing interface there is so far. */
#define IC_CANVAS_GL 1u

/* Where a plugin may put pixels of its own. get_proc_address is the host's,
   because finding a GL symbol differs on every system. */
typedef struct IcCanvas {
    uint32_t struct_size;
    uint32_t api;
    void *(*get_proc_address)(void *ctx, const char *name);
    void *proc_ctx;
} IcCanvas;

/* One frame's worth of where to draw. Width and height already carry the scale. */
typedef struct IcFrame {
    uint32_t struct_size;
    int32_t fbo;
    int32_t width;
    int32_t height;
    double scale;
} IcFrame;

typedef int (*IcCanvasReadyFn)(uint64_t instance, const IcCanvas *canvas, void *user_data);
typedef void (*IcCanvasDrawFn)(uint64_t instance, const IcFrame *frame, void *user_data);
typedef void (*IcCanvasGoneFn)(uint64_t instance, void *user_data);

typedef struct IcViewerVTable {
    uint32_t struct_size;
    const IcViewVTable *view;
    IcViewerOpenFn open;
    IcViewClosedFn closed;   /* may be NULL */
    IcViewerContentFn content; /* may be NULL */
    IcViewerClosingFn closing; /* may be NULL */
    /* All three NULL together for a plugin that only describes windows. */
    IcCanvasReadyFn canvas_ready;
    IcCanvasDrawFn canvas_draw;
    IcCanvasGoneFn canvas_gone;
} IcViewerVTable;

typedef IcFsHandle (*IcConnectionOpenFn)(const uint8_t *settings, uint64_t settings_len,
                                         void *user_data);

typedef struct IcConnectionVTable {
    uint32_t struct_size;
    IcConnectionOpenFn open;
    const IcFsVTable *fs;
    IcViewDescribeFn describe; /* may be NULL */
    IcViewEventFn on_event;    /* may be NULL */
} IcConnectionVTable;

/* The application's side. One instance for the process; grows only at the end. */
typedef struct IcHost {
    /* What a plugin checks before it trusts anything below. */
    uint64_t magic;
    uint32_t struct_size;
    uint32_t abi_version;
    const char *(*host_version)(void);

    /* The host itself: what it is, what it says, what it keeps. */
    void (*log_warn)(const char *message);
    const char *(*language)(void);
    int (*register_locales)(const char *language, const uint8_t *catalogue, uint64_t catalogue_len);
    int (*register_plugin_asset)(const char *plugin_id, const char *name, const uint8_t *bytes,
                                 uint64_t bytes_len);
    IcBytes (*settings_read)(const char *plugin_id, const char *key);
    int (*settings_write)(const char *plugin_id, const char *key, const uint8_t *bytes,
                          uint64_t len, uint32_t flags);
    /* One question put to the user; `answered` is called once with
       {"button": "<id>", "text": "<typed>"}. See ask in ic-plugin-api. */
    int (*ask)(const uint8_t *spec, uint64_t spec_len, IcAnswerFn answered, void *user_data);

    /* The chrome a plugin may add to and hide. */
    int (*add_toolbar_button)(const char *id, const char *svg, const char *tooltip, uint32_t side,
                              int32_t priority, uint32_t enable_flags, IcClickFn on_click,
                              void *user_data);
    int (*set_default_toolbar_visible)(const char *extensions, int shown);
    int (*add_header_button)(const char *id, const char *svg, const char *label, const char *tooltip,
                             uint32_t side, int32_t priority, IcClickFn on_click, void *user_data);
    int (*set_header_label)(const char *id, const char *label);
    int (*set_header_icon)(const char *id, const char *svg);
    int (*set_header_visible)(const char *id, int shown);

    /* Panels: what is in them, and what a plugin may put there. */
    IcSelection (*selection)(void);
    int (*register_panel_source)(const char *id, const char *title, const char *svg, IcTableFn rows,
                                 void *user_data);
    int (*open_panel_source)(const char *id);
    int (*close_panel_source)(void);
    int (*register_panel_action)(const char *source_id, const char *action_id, const char *svg,
                                 const char *tooltip, uint32_t enable_flags, IcClickFn on_click,
                                 void *user_data);
    int (*register_panel_tree)(const char *id, const char *title, const char *svg, IcTreeFn rows,
                               void *user_data);

    /* Where the places in the sidebar come from. */
    int (*register_drive_source)(const char *kind, const char *svg, IcDrivesFn rows,
                                 void *user_data);
    int (*drives_changed)(void);
    int (*register_connection_kind)(const char *id, const uint8_t *schema, uint64_t schema_len,
                                    const IcConnectionVTable *vtable, void *user_data);
    int (*register_pinned_connections)(const char *id, IcPinnedConnectionsFn rows,
                                       void *user_data);
    int (*pinned_connections_changed)(void);

    /* Filesystems a plugin brings, and telling the host they moved. */
    int (*register_filesystem)(const char *extensions, const IcFsVTable *vtable, void *user_data);
    int (*register_fs_action)(const char *extensions, const char *action_id, const char *svg,
                              const char *tooltip, uint32_t enable_flags, uint32_t kind,
                              IcFsClickFn on_click, void *user_data);
    int (*fs_invalidate)(const char *extensions);
    /* Says what the plugin changed, so whatever the host remembers about that
       path is thrown away. An empty path means the whole filesystem. */
    int (*fs_changed)(IcFsSource source, const char *path);

    /* Reading a filesystem the host already has, whoever brought it: the only
       slots through which a plugin asks the application for something rather
       than being handed it.
       A source arrives with IcFsVTable::open_in and is good for as long as the
       mount opened upon it; call them from the thread that handed it over,
       while that call is running. */
    uint32_t (*fs_caps)(IcFsSource source);
    IcStream (*fs_open)(IcFsSource source, const char *path, uint32_t mode);
    /* Bytes into the caller's buffer: 0 is the end, -1 an error, and a short
       answer is not the end. */
    int64_t (*fs_read)(IcStream stream, uint8_t *into, uint64_t len);
    /* The new position, or -1. Seeking to the end is how a length is asked
       for, and a source that is not cheap to seek answers anyway. */
    int64_t (*fs_seek)(IcStream stream, int64_t offset, uint32_t whence);
    int64_t (*fs_write)(IcStream stream, const uint8_t *from, uint64_t len);
    int (*fs_truncate)(IcStream stream, uint64_t len);
    void (*fs_close)(IcStream stream);
    /* The rows belong to the host and are good until the next call on this
       thread. */
    IcListing (*fs_list)(IcFsSource source, const char *path);
    /* A real path for something that opens files itself — a player, a
       renderer. The host copies when it must, and the copy lasts as long as
       the source. NULL when it cannot. */
    const char *(*fs_local_path)(IcFsSource source, const char *path);

    /* Documents the host draws on the plugin's behalf. */
    int (*register_view)(const char *id, const char *title, const IcViewVTable *vtable,
                         void *user_data);
    int (*open_view)(const char *id, const uint8_t *arg, uint64_t arg_len);
    int (*view_invalidate)(const char *id);
    /* A plugin with a frame ready says so here and is asked to draw on the
       frontend's own thread. */
    int (*canvas_invalidate)(uint64_t instance);

    /* Offers to show the files named by these extensions — ".png,.jpg,.nef",
       spelled as register_filesystem spells them. By extension and nothing
       else: the application does not read a file to find out what it is.
       `priority` settles it when two plugins offer the same extension, the
       larger winning; the application's own text and hex viewer is always the
       last resort. */
    int (*register_viewer)(const char *viewer_id, const char *extensions, int32_t priority,
                           const IcViewerVTable *vtable, void *user_data);
} IcHost;

typedef struct IcAbout {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t abi_version;
    const char *id;
    const char *name;
    const char *version;
    const char *description;
} IcAbout;

/* Whether a host is one this plugin can talk to at all, and whether it reaches
 * as far as the slot the plugin means to call. `needed` is the offset of that
 * slot plus the size of a pointer — the same arithmetic `needs_up_to` does on
 * the Rust side. */
static inline int ic_host_usable(const IcHost *host, uint32_t needed) {
    if (host == NULL) {
        return 0;
    }
    if (host->magic != IC_HOST_MAGIC || host->abi_version != IC_ABI_VERSION) {
        return 0;
    }
    return host->struct_size >= needed;
}

#define IC_NEEDS_UP_TO(field) ((uint32_t)(offsetof(IcHost, field) + sizeof(void *)))

#ifdef __cplusplus
}
#endif

#endif /* IC_PLUGIN_H */
