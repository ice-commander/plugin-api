/* Every layout a plugin written in C depends on, as C sees it. The Rust tests
 * beside this file ask for each one and compare it with `offset_of!`. */

#include "ic_plugin.h"

typedef struct {
    const char *name;
    size_t value;
} IcNamed;

#define FIELD(type, field) {#field, offsetof(type, field)}

static const IcNamed HOST_FIELDS[] = {
    FIELD(IcHost, magic),
    FIELD(IcHost, struct_size),
    FIELD(IcHost, abi_version),
    FIELD(IcHost, host_version),
    FIELD(IcHost, log_warn),
    FIELD(IcHost, language),
    FIELD(IcHost, register_locales),
    FIELD(IcHost, register_plugin_asset),
    FIELD(IcHost, settings_read),
    FIELD(IcHost, settings_write),
    FIELD(IcHost, ask),
    FIELD(IcHost, add_toolbar_button),
    FIELD(IcHost, set_default_toolbar_visible),
    FIELD(IcHost, add_header_button),
    FIELD(IcHost, set_header_label),
    FIELD(IcHost, set_header_icon),
    FIELD(IcHost, set_header_visible),
    FIELD(IcHost, selection),
    FIELD(IcHost, register_panel_source),
    FIELD(IcHost, open_panel_source),
    FIELD(IcHost, close_panel_source),
    FIELD(IcHost, register_panel_action),
    FIELD(IcHost, register_panel_tree),
    FIELD(IcHost, register_drive_source),
    FIELD(IcHost, drives_changed),
    FIELD(IcHost, register_connection_kind),
    FIELD(IcHost, register_pinned_connections),
    FIELD(IcHost, pinned_connections_changed),
    FIELD(IcHost, register_filesystem),
    FIELD(IcHost, register_fs_action),
    FIELD(IcHost, fs_invalidate),
    FIELD(IcHost, fs_changed),
    FIELD(IcHost, fs_caps),
    FIELD(IcHost, fs_open),
    FIELD(IcHost, fs_read),
    FIELD(IcHost, fs_seek),
    FIELD(IcHost, fs_write),
    FIELD(IcHost, fs_truncate),
    FIELD(IcHost, fs_close),
    FIELD(IcHost, fs_list),
    FIELD(IcHost, fs_local_path),
    FIELD(IcHost, register_view),
    FIELD(IcHost, open_view),
    FIELD(IcHost, view_invalidate),
    FIELD(IcHost, canvas_invalidate),
    FIELD(IcHost, register_viewer),
};

static const IcNamed FS_FIELDS[] = {
    FIELD(IcFsVTable, struct_size),   FIELD(IcFsVTable, open_in),
    FIELD(IcFsVTable, close),         FIELD(IcFsVTable, list),
    FIELD(IcFsVTable, read),          FIELD(IcFsVTable, is_read_only),
    FIELD(IcFsVTable, last_error),    FIELD(IcFsVTable, write),
    FIELD(IcFsVTable, create_dir),    FIELD(IcFsVTable, remove),
    FIELD(IcFsVTable, rename),        FIELD(IcFsVTable, shell_open),
    FIELD(IcFsVTable, shell_read),    FIELD(IcFsVTable, shell_write),
    FIELD(IcFsVTable, shell_resize),  FIELD(IcFsVTable, shell_close),
    FIELD(IcFsVTable, shell_available), FIELD(IcFsVTable, columns),
    FIELD(IcFsVTable, list_rows),     FIELD(IcFsVTable, action_state),
    FIELD(IcFsVTable, cell_clicked),  FIELD(IcFsVTable, set_permissions),
};

static const IcNamed VIEWER_FIELDS[] = {
    FIELD(IcViewerVTable, struct_size), FIELD(IcViewerVTable, view),
    FIELD(IcViewerVTable, open),        FIELD(IcViewerVTable, closed),
    FIELD(IcViewerVTable, content),     FIELD(IcViewerVTable, closing),
    FIELD(IcViewerVTable, canvas_ready), FIELD(IcViewerVTable, canvas_draw),
    FIELD(IcViewerVTable, canvas_gone),
};

static const IcNamed VIEW_FIELDS[] = {
    FIELD(IcViewVTable, struct_size),
    FIELD(IcViewVTable, describe),
    FIELD(IcViewVTable, on_event),
    FIELD(IcViewVTable, closed),
};

static const IcNamed CONNECTION_FIELDS[] = {
    FIELD(IcConnectionVTable, struct_size), FIELD(IcConnectionVTable, open),
    FIELD(IcConnectionVTable, fs),          FIELD(IcConnectionVTable, describe),
    FIELD(IcConnectionVTable, on_event),
};

static const IcNamed ABOUT_FIELDS[] = {
    FIELD(IcAbout, magic), FIELD(IcAbout, struct_size), FIELD(IcAbout, abi_version),
    FIELD(IcAbout, id),    FIELD(IcAbout, name),        FIELD(IcAbout, version),
    FIELD(IcAbout, description),
};

static const IcNamed ENTRY_FIELDS[] = {
    FIELD(IcDirEntry, name),        FIELD(IcDirEntry, is_dir),
    FIELD(IcDirEntry, size),        FIELD(IcDirEntry, modified),
    FIELD(IcDirEntry, permissions), FIELD(IcDirEntry, has_permissions),
};

static const IcNamed SIZES[] = {
    {"IcHost", sizeof(IcHost)},
    {"IcFsVTable", sizeof(IcFsVTable)},
    {"IcViewVTable", sizeof(IcViewVTable)},
    {"IcConnectionVTable", sizeof(IcConnectionVTable)},
    {"IcAbout", sizeof(IcAbout)},
    {"IcDirEntry", sizeof(IcDirEntry)},
    {"IcListing", sizeof(IcListing)},
    {"IcBytes", sizeof(IcBytes)},
    {"IcSelectionItem", sizeof(IcSelectionItem)},
    {"IcSelection", sizeof(IcSelection)},
    {"IcColumn", sizeof(IcColumn)},
    {"IcTable", sizeof(IcTable)},
    {"IcTree", sizeof(IcTree)},
    {"IcDrive", sizeof(IcDrive)},
    {"IcDrives", sizeof(IcDrives)},
    {"IcPinnedConnection", sizeof(IcPinnedConnection)},
    {"IcPinnedConnections", sizeof(IcPinnedConnections)},
    {"IcFsColumn", sizeof(IcFsColumn)},
    {"IcColumns", sizeof(IcColumns)},
    {"IcRow", sizeof(IcRow)},
    {"IcRows", sizeof(IcRows)},
    {"IcViewerVTable", sizeof(IcViewerVTable)},
    {"IcCanvas", sizeof(IcCanvas)},
    {"IcFrame", sizeof(IcFrame)},
};

typedef enum {
    IC_TABLE_HOST = 0,
    IC_TABLE_FS = 1,
    IC_TABLE_VIEW = 2,
    IC_TABLE_CONNECTION = 3,
    IC_TABLE_ABOUT = 4,
    IC_TABLE_ENTRY = 5,
    IC_TABLE_SIZES = 6,
    IC_TABLE_VIEWER = 7
} IcWhichTable;

static const IcNamed *table_of(int which, size_t *count) {
    switch (which) {
    case IC_TABLE_HOST:
        *count = sizeof(HOST_FIELDS) / sizeof(HOST_FIELDS[0]);
        return HOST_FIELDS;
    case IC_TABLE_FS:
        *count = sizeof(FS_FIELDS) / sizeof(FS_FIELDS[0]);
        return FS_FIELDS;
    case IC_TABLE_VIEW:
        *count = sizeof(VIEW_FIELDS) / sizeof(VIEW_FIELDS[0]);
        return VIEW_FIELDS;
    case IC_TABLE_CONNECTION:
        *count = sizeof(CONNECTION_FIELDS) / sizeof(CONNECTION_FIELDS[0]);
        return CONNECTION_FIELDS;
    case IC_TABLE_ABOUT:
        *count = sizeof(ABOUT_FIELDS) / sizeof(ABOUT_FIELDS[0]);
        return ABOUT_FIELDS;
    case IC_TABLE_ENTRY:
        *count = sizeof(ENTRY_FIELDS) / sizeof(ENTRY_FIELDS[0]);
        return ENTRY_FIELDS;
    case IC_TABLE_SIZES:
        *count = sizeof(SIZES) / sizeof(SIZES[0]);
        return SIZES;
    case IC_TABLE_VIEWER:
        *count = sizeof(VIEWER_FIELDS) / sizeof(VIEWER_FIELDS[0]);
        return VIEWER_FIELDS;
    default:
        *count = 0;
        return NULL;
    }
}

size_t ic_conformance_count(int which) {
    size_t count = 0;
    table_of(which, &count);
    return count;
}

const char *ic_conformance_name(int which, size_t at) {
    size_t count = 0;
    const IcNamed *table = table_of(which, &count);
    return (table != NULL && at < count) ? table[at].name : NULL;
}

size_t ic_conformance_value(int which, size_t at) {
    size_t count = 0;
    const IcNamed *table = table_of(which, &count);
    return (table != NULL && at < count) ? table[at].value : (size_t)-1;
}

/* What the header itself says, so the constants are compared too. */
uint64_t ic_conformance_host_magic(void) { return IC_HOST_MAGIC; }
uint32_t ic_conformance_abi_version(void) { return IC_ABI_VERSION; }
uint64_t ic_conformance_about_magic(void) { return IC_ABOUT_MAGIC; }
uint64_t ic_conformance_tree_magic(void) { return IC_TREE_MAGIC; }
uint64_t ic_conformance_drives_magic(void) { return IC_DRIVES_MAGIC; }
uint64_t ic_conformance_pinned_magic(void) { return IC_PINNED_MAGIC; }
uint64_t ic_conformance_columns_magic(void) { return IC_COLUMNS_MAGIC; }
uint64_t ic_conformance_rows_magic(void) { return IC_ROWS_MAGIC; }

static const IcNamed WORDS[] = {
    {IC_HOST_GTK, 0},       {IC_HOST_WEB, 0},         {IC_HOST_CONSOLE, 0},
    {IC_CELL_TICKED, 0},    {IC_EVENT_OPENED, 0},     {IC_EVENT_ACTIVATE, 0},
    {IC_EVENT_CHANGE, 0},   {IC_EVENT_EXPAND, 0},     {IC_EVENT_COLLAPSE, 0},
    {IC_EVENT_ENDED, 0},
};

size_t ic_conformance_word_count(void) { return sizeof(WORDS) / sizeof(WORDS[0]); }

const char *ic_conformance_word(size_t at) {
    return at < ic_conformance_word_count() ? WORDS[at].name : NULL;
}
