/* Ice Commander plugin ABI for C; layouts match ic-plugin-api byte for byte. Exports:
 * ic_plugin_init (required), ic_plugin_shutdown, ic_plugin_about, ic_plugin_name, ic_plugin_version. */

#ifndef IC_PLUGIN_H
#define IC_PLUGIN_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define IC_HOST_MAGIC 0x49435f484f535400ull
#define IC_ABI_VERSION 1u

/* The `kind` passed to ic_plugin_init; answer anything but IC_OK to decline loading. */
#define IC_HOST_GTK "gtk"
#define IC_HOST_WEB "web"
#define IC_HOST_CONSOLE "console"
#define IC_HOST_ANDROID "android"

#define IC_OK 0
#define IC_ERR_HOST_TOO_OLD 1
#define IC_ERR_HOST_UNKNOWN 2
#define IC_ERR_INIT_FAILED 3
/* A failed filesystem call; the reason is in last_error. */
#define IC_ERR_IO 4
/* init declining this host. Not reported as a failure. */
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
/* Host-owned and opaque to the plugin. */
typedef void *IcFsSource;
/* Host-owned. */
typedef void *IcStream;

/* Ask for no more than needed: a read-only source refuses IC_OPEN_WRITE. */
#define IC_OPEN_READ 0u
#define IC_OPEN_WRITE 1u   /* truncates, or makes the file */
#define IC_OPEN_UPDATE 2u  /* reads and writes, keeping what is there */

#define IC_SIZE_UNKNOWN 0

#define IC_SEEK_SET 0u
#define IC_SEEK_CURRENT 1u
#define IC_SEEK_END 2u

/* Costs, not permissions: seeking without IC_FS_SEEK still works, the host pays for it. */
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

/* No header, so it can never grow. */
typedef struct IcTable {
    const IcColumn *columns;
    uint32_t column_count;
    const char *const *cells;
    uint32_t row_count;
    uint32_t key_column;
} IcTable;

typedef IcTable (*IcTableFn)(void *user_data);

/* Like IcTable plus `enterable`; unlike it, can grow via struct_size. */
typedef struct IcTree {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t column_count;
    const IcColumn *columns;
    const char *const *cells;
    uint32_t row_count;
    uint32_t key_column;
    /* Non-zero where the row opens; NULL means none do. */
    const int *enterable;
} IcTree;

/* `path` is empty at the root, otherwise the row keys joined by `/`. */
typedef IcTree (*IcTreeFn)(const char *path, void *user_data);

/* Called once, on the frontend thread. */
typedef void (*IcAnswerFn)(const uint8_t *answer, uint64_t answer_len, void *user_data);

typedef struct IcDrive {
    /* Must stay the same across restarts: favourites hang off it. */
    const char *key;
    const char *name;
    const char *subtitle;
    /* Passed to the connection kind's open when picked. */
    const char *settings;
    /* NULL or empty uses the icon the source was registered with. */
    const char *svg;
    int online;
} IcDrive;

typedef struct IcDrives {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcDrive *rows;
} IcDrives;

/* Called on the frontend thread on every redraw; answer from memory, no I/O. */
typedef IcDrives (*IcDrivesFn)(void *user_data);

/* The user can neither rename nor delete it. */
typedef struct IcPinnedConnection {
    const char *id;
    const char *title;
    /* Empty uses the application's own icon. */
    const char *svg;
    /* A register_view id; its describe gets this entry's `id`. */
    const char *view;
} IcPinnedConnection;

typedef struct IcPinnedConnections {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcPinnedConnection *rows;
} IcPinnedConnections;

/* Called on the frontend thread on every redraw. */
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

/* Pointers a plugin answers with stay its own, valid until its next call on the same handle. */
typedef struct IcBytes {
    const uint8_t *data;
    uint64_t len;
} IcBytes;

typedef struct IcScreen {
    uint32_t width_dp;
    uint32_t height_dp;
    /* Unreliable on Android; decide by width_dp. */
    float scale;
} IcScreen;

/* A mount's own column; IcColumn is frozen. */
typedef struct IcFsColumn {
    const char *key;
    const char *title;
    int32_t width;
    /* IC_COLUMN_TEXT or IC_COLUMN_CHECK. */
    uint32_t kind;
} IcFsColumn;

/* Added after the panel's own columns, never replacing them. */
typedef struct IcColumns {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcFsColumn *items;
} IcColumns;

typedef struct IcRow {
    IcDirEntry entry;
    /* One string per declared column, in order; a NULL array or cell reads as empty. */
    const char *const *extra;
    /* Bounds every read of `extra`. */
    uint32_t extra_count;
} IcRow;

typedef struct IcRows {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t count;
    const IcRow *items;
} IcRows;

/* `mount` is the one the press happened in; NULL if the panel has left it since. */
typedef void (*IcFsClickFn)(IcFsHandle mount, void *user_data, void *parent);
/* `dir` and `name` as list spelled them. The host lists again afterwards. */
typedef int (*IcFsCellClickedFn)(IcFsHandle handle, const char *dir, const char *name,
                                 const char *column_key, int ticked);
/* Mask of IC_ACTION_SHOWN, IC_ACTION_ENABLED, IC_ACTION_ON. Called on the frontend
   thread on every toolbar refresh; answer from memory, no I/O. */
typedef uint32_t (*IcFsActionStateFn)(IcFsHandle handle, const char *action_id);
typedef IcColumns (*IcFsColumnsFn)(IcFsHandle handle);
typedef IcRows (*IcFsRowsFn)(IcFsHandle handle, const char *path);
/* Opens a mount inside a file on another filesystem: gets its source and path, never its bytes. */
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
/* One host thread drives all five. shell_read must not block: empty when idle,
   NULL `data` once the shell has ended. */
typedef IcShellHandle (*IcShellOpenFn)(IcFsHandle handle, const char *cwd, uint32_t rows,
                                       uint32_t cols);
typedef IcBytes (*IcShellReadFn)(IcShellHandle shell);
typedef int (*IcShellWriteFn)(IcShellHandle shell, const uint8_t *bytes, uint64_t len);
typedef int (*IcShellResizeFn)(IcShellHandle shell, uint32_t rows, uint32_t cols);
typedef void (*IcShellCloseFn)(IcShellHandle shell);
/* Per mount; absent means every mount has a shell iff shell_open is set. */
typedef int (*IcShellAvailableFn)(IcFsHandle handle);

/* The plugin's stream; IcStream is the host's. */
typedef void *IcFsStream;

/* One connection and one transfer. `size` is the byte count, 0 when not known. */
typedef IcFsStream (*IcFsStreamOpenFn)(IcFsHandle handle, const char *path, uint32_t mode,
                                       uint64_t size);
/* 0 is the end of the file, -1 an error; short of `len` is neither. */
typedef int64_t (*IcFsStreamReadFn)(IcFsStream stream, uint8_t *into, uint64_t len);
typedef int64_t (*IcFsStreamWriteFn)(IcFsStream stream, const uint8_t *from, uint64_t len);
/* A write may only fail here, on the flush. */
typedef int (*IcFsStreamCloseFn)(IcFsStream stream);
/* Replaces stream_close when the host abandons a write; the destination keeps what it had. */
typedef void (*IcFsStreamAbortFn)(IcFsStream stream);
/* Copies without the bytes crossing; anything but IC_OK falls back to an ordinary copy. */
typedef int (*IcFsCopyWithinFn)(IcFsHandle handle, const char *from, const char *to);
/* The file's real path on disk, for a mount over local files; NULL when none. */
typedef const char *(*IcFsOnDiskFn)(IcFsHandle handle, const char *path);

/* Copied by the host; what its pointers point at must stay alive. Slots from `write` down may be NULL. */
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
    /* Asked once per listing, before the rows. */
    IcFsColumnsFn columns;
    /* list with the cells for `columns`; preferred when both are set. */
    IcFsRowsFn list_rows;
    /* NULL gives IC_ACTION_DEFAULT for every button. */
    IcFsActionStateFn action_state;
    IcFsCellClickedFn cell_clicked;
    /* NULL: no chmod is offered on this filesystem. */
    IcFsPermissionsFn set_permissions;
    /* NULL stream_open: files move whole. Called on the thread that opened the stream;
       the host closes what it opens. */
    IcFsStreamOpenFn stream_open;
    IcFsStreamReadFn stream_read;
    IcFsStreamWriteFn stream_write;
    IcFsStreamCloseFn stream_close;
    IcFsCopyWithinFn copy_within;
    IcFsOnDiskFn on_disk;
    IcFsStreamAbortFn stream_abort;
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
/* Also sent for a key from the document's "keys"; "node" names its node. */
#define IC_EVENT_ACTIVATE "activate"
#define IC_EVENT_CHANGE "change"
#define IC_EVENT_EXPAND "expand"
#define IC_EVENT_COLLAPSE "collapse"
/* A media node played to its end; "node" names it. */
#define IC_EVENT_ENDED "ended"

/* Gets the file's source and path, never its bytes: read through fs_open. */
typedef int (*IcViewerOpenFn)(uint64_t instance, IcFsSource source, const char *path,
                              void *user_data);
/* Bytes for a `part:<name>` reference; the plugin's until it is asked again. */
typedef IcBytes (*IcViewerContentFn)(uint64_t instance, const char *name, void *user_data);

/* IC_OK lets the window close. */
typedef int (*IcViewerClosingFn)(uint64_t instance, void *user_data);

/* The only `api` so far; a plugin that does not know `api` draws nothing. */
#define IC_CANVAS_GL 1u

/* Handed over once the host has a drawing surface; resolve GL symbols via get_proc_address. */
typedef struct IcCanvas {
    uint32_t struct_size;
    uint32_t api;
    void *(*get_proc_address)(void *ctx, const char *name);
    void *proc_ctx;
} IcCanvas;

/* Width and height are in pixels, scale already applied. */
typedef struct IcFrame {
    uint32_t struct_size;
    int32_t fbo;
    int32_t width;
    int32_t height;
    double scale;
} IcFrame;

/* The context is current. Answer IC_OK to be drawn. */
typedef int (*IcCanvasReadyFn)(uint64_t instance, const IcCanvas *canvas, void *user_data);
/* Frontend thread only, between canvas_ready and canvas_gone. */
typedef void (*IcCanvasDrawFn)(uint64_t instance, const IcFrame *frame, void *user_data);
/* Free what canvas_ready made; the context is still current. */
typedef void (*IcCanvasGoneFn)(uint64_t instance, void *user_data);

/* A view over one file; its context carries "instance". Pictures are named
   `file:<path>` (on the viewer's source) or `part:<name>` (from content). */
typedef struct IcViewerVTable {
    uint32_t struct_size;
    const IcViewVTable *view;
    IcViewerOpenFn open;
    IcViewClosedFn closed;   /* may be NULL; the source must not be used after this */
    IcViewerContentFn content; /* may be NULL */
    /* May be NULL. Anything but IC_OK keeps the window open and describes it again; closed
       follows a real close. */
    IcViewerClosingFn closing;
    /* The three canvas slots are set or NULL together. */
    IcCanvasReadyFn canvas_ready;
    IcCanvasDrawFn canvas_draw;
    IcCanvasGoneFn canvas_gone;
} IcViewerVTable;

typedef int (*IcPlayerOpenFn)(IcFsSource source, const char *path, void *user_data);

/* `of` is 0 while the length is not known. */
typedef struct IcPlayedSoFar {
    double at;
    double of;
    int paused;
    int ended;
} IcPlayedSoFar;

typedef IcPlayedSoFar (*IcPlayerWhereFn)(void *user_data);
typedef int (*IcPlayerPauseFn)(int paused, void *user_data);
typedef int (*IcPlayerSeekFn)(double to, void *user_data);
typedef int (*IcPlayerVolumeFn)(double volume, void *user_data);
typedef void (*IcPlayerStopFn)(void *user_data);

/* Only the sound: the list, the titles and the artwork stay with the application. */
typedef struct IcPlayerVTable {
    uint32_t struct_size;
    IcPlayerOpenFn open;
    IcPlayerWhereFn played_so_far;
    IcPlayerPauseFn pause;
    IcPlayerStopFn stop;
    IcPlayerSeekFn seek;     /* may be NULL */
    IcPlayerVolumeFn volume; /* may be NULL */
} IcPlayerVTable;

/* JSON of strings; the host adds "ic.connect_timeout_secs" and "ic.request_timeout_secs". */
typedef IcFsHandle (*IcConnectionOpenFn)(const uint8_t *settings, uint64_t settings_len,
                                         void *user_data);

typedef struct IcConnectionVTable {
    uint32_t struct_size;
    IcConnectionOpenFn open;
    const IcFsVTable *fs;
    /* May run off the frontend thread: only log_warn is safe to call back. */
    IcViewDescribeFn describe; /* may be NULL */
    IcViewEventFn on_event;    /* may be NULL */
} IcConnectionVTable;

/* One table per process, outliving every plugin; keep the pointer, never copy the table (it may be
   longer than this header). Slots run on the init thread unless they say otherwise. Grows only at the end. */
typedef struct IcHost {
    /* What a plugin checks before it trusts anything below. */
    uint64_t magic;
    uint32_t struct_size;
    uint32_t abi_version;
    const char *(*host_version)(void);

    /* The host itself: what it is, what it says, what it keeps. */
    void (*log_warn)(const char *message);
    /* Two-letter code of the UI language; valid for the process. */
    const char *(*language)(void);
    int (*register_locales)(const char *language, const uint8_t *catalogue, uint64_t catalogue_len);
    /* Referred to as `asset:<plugin_id>/<name>`. */
    int (*register_plugin_asset)(const char *plugin_id, const char *name, const uint8_t *bytes,
                                 uint64_t bytes_len);
    /* Empty when unset. Valid until the next call on this thread. */
    IcBytes (*settings_read)(const char *plugin_id, const char *key);
    /* A NULL `bytes` deletes the key. `flags`: IC_SETTING_PLAIN or IC_SETTING_SECRET. */
    int (*settings_write)(const char *plugin_id, const char *key, const uint8_t *bytes,
                          uint64_t len, uint32_t flags);
    /* `spec` JSON as documented on ask in ic-plugin-api. `answered` gets
       {"button", "text"?, "choice"?} once; dismissal reports the first button. */
    int (*ask)(const uint8_t *spec, uint64_t spec_len, IcAnswerFn answered, void *user_data);

    /* The chrome a plugin may add to and hide. */
    int (*add_toolbar_button)(const char *id, const char *svg, const char *tooltip, uint32_t side,
                              int32_t priority, uint32_t enable_flags, IcClickFn on_click,
                              void *user_data);
    /* Inside mounts of `extensions` (spelled as in register_filesystem). Default: shown. */
    int (*set_default_toolbar_visible)(const char *extensions, int shown);
    int (*add_header_button)(const char *id, const char *svg, const char *label, const char *tooltip,
                             uint32_t side, int32_t priority, IcClickFn on_click, void *user_data);
    /* An empty label leaves only the icon. Callable from any thread. */
    int (*set_header_label)(const char *id, const char *label);
    /* Callable from any thread. */
    int (*set_header_icon)(const char *id, const char *svg);
    /* Callable from any thread. */
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
    /* Extra entries in the drives list; picking one calls open of connection kind `kind`. */
    int (*register_drive_source)(const char *kind, const char *svg, IcDrivesFn rows,
                                 void *user_data);
    /* Callable from any thread. */
    int (*drives_changed)(void);
    /* An empty `schema`: the kind only mounts and is never offered for creation. */
    int (*register_connection_kind)(const char *id, const uint8_t *schema, uint64_t schema_len,
                                    const IcConnectionVTable *vtable, void *user_data);
    /* Registering the same `id` again replaces the entries. */
    int (*register_pinned_connections)(const char *id, IcPinnedConnectionsFn rows,
                                       void *user_data);
    /* Callable from any thread. */
    int (*pinned_connections_changed)(void);

    /* Filesystems a plugin brings, and telling the host they moved. */
    /* The vtable runs on a worker thread; from there only log_warn is safe to call back. */
    int (*register_filesystem)(const char *extensions, const IcFsVTable *vtable, void *user_data);
    /* Shown only inside mounts of `extensions` (spelled as in register_filesystem). */
    int (*register_fs_action)(const char *extensions, const char *action_id, const char *svg,
                              const char *tooltip, uint32_t enable_flags, uint32_t kind,
                              IcFsClickFn on_click, void *user_data);
    /* Callable from any thread. */
    int (*fs_invalidate)(const char *extensions);
    /* Drops what the host caches for `path` on `source`; an empty path means all of it. */
    int (*fs_changed)(IcFsSource source, const char *path);

    /* A source comes from IcFsVTable::open_in, lives as long as that mount, and may only
       be used on the thread of that call while it runs. */
    /* IC_FS_* bits: a cost hint, not a permission. */
    uint32_t (*fs_caps)(IcFsSource source);
    /* `mode` is IC_OPEN_*; `size` the byte count, 0 when not known. NULL on failure. */
    IcStream (*fs_open)(IcFsSource source, const char *path, uint32_t mode, uint64_t size);
    /* 0 is the end, -1 an error; a short read is neither. */
    int64_t (*fs_read)(IcStream stream, uint8_t *into, uint64_t len);
    /* The new position or -1; `whence` is IC_SEEK_*. Works without IC_FS_SEEK, slower. */
    int64_t (*fs_seek)(IcStream stream, int64_t offset, uint32_t whence);
    /* Bytes taken, or -1. */
    int64_t (*fs_write)(IcStream stream, const uint8_t *from, uint64_t len);
    int (*fs_truncate)(IcStream stream, uint64_t len);
    /* Written data is flushed by the time this returns. */
    void (*fs_close)(IcStream stream);
    /* Host-owned rows, valid until the next call on this thread. */
    IcListing (*fs_list)(IcFsSource source, const char *path);
    /* A real local path, copied if needed and kept as long as the source; NULL if impossible. */
    const char *(*fs_local_path)(IcFsSource source, const char *path);

    /* Documents the host draws on the plugin's behalf. */
    int (*register_view)(const char *id, const char *title, const IcViewVTable *vtable,
                         void *user_data);
    int (*open_view)(const char *id, const uint8_t *arg, uint64_t arg_len);
    int (*view_invalidate)(const char *id);
    /* Asks for canvas_draw on the frontend thread; never draw from a thread of your own. */
    int (*canvas_invalidate)(uint64_t instance);

    /* `extensions` like ".png,.jpg", matched by name only. The larger `priority` wins;
       the built-in text and hex viewer is always the fallback. */
    int (*register_viewer)(const char *viewer_id, const char *extensions, int32_t priority,
                           const IcViewerVTable *vtable, void *user_data);

    IcScreen (*screen)(void);

    /* Same extension list and priority rule as register_viewer. */
    int (*register_player)(const char *player_id, const char *extensions, int32_t priority,
                           const IcPlayerVTable *vtable, void *user_data);

    /* Platform-specific; IC_ERR_NOT_THIS_HOST where nothing handles `name`. Callable from any thread. */
    int (*platform_call)(const char *plugin_id, const char *name, const char *value);
} IcHost;

typedef struct IcAbout {
    uint64_t magic;
    uint32_t struct_size;
    uint32_t abi_version;
    /* Stable identity, not the file name. */
    const char *id;
    /* Plain Latin display name, not a translation key. */
    const char *name;
    const char *version;
    const char *description;
} IcAbout;

/* `needed` is IC_NEEDS_UP_TO(slot); check it before calling that slot. */
static inline int ic_host_usable(const IcHost *host, uint32_t needed) {
    if (host == NULL) {
        return 0;
    }
    if (host->magic != IC_HOST_MAGIC || host->abi_version < IC_ABI_VERSION) {
        return 0;
    }
    return host->struct_size >= needed;
}

#define IC_NEEDS_UP_TO(field) ((uint32_t)(offsetof(IcHost, field) + sizeof(void *)))

#ifdef __cplusplus
}
#endif

#endif /* IC_PLUGIN_H */
