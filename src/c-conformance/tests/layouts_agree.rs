//! The C header and the Rust declarations, field by field.
//!
//! Adding a slot means adding it in both places. If only one side is touched,
//! this is what says so — a plugin written in C would otherwise call through
//! the wrong offset, which nothing else in either language would notice.

use ic_c_conformance::{as_c_sees_it, constants_in_the_header, words_in_the_header, Table};
use ic_plugin_api::*;

const HOST: Table = Table::Host;
const FS: Table = Table::Filesystem;
const VIEW: Table = Table::Window;
const CONNECTION: Table = Table::Connection;
const ABOUT: Table = Table::About;
const ENTRY: Table = Table::DirEntry;
const SIZES: Table = Table::Sizes;

fn same(which: Table, what: &str, rust: Vec<(&str, usize)>) {
    let c = as_c_sees_it(which);
    let rust: Vec<(String, usize)> = rust
        .into_iter()
        .map(|(name, at)| (name.to_string(), at))
        .collect();
    assert_eq!(
        c.len(),
        rust.len(),
        "{what}: C names {} fields, Rust {}",
        c.len(),
        rust.len()
    );
    for (seen, wanted) in c.iter().zip(rust.iter()) {
        assert_eq!(
            seen, wanted,
            "{what}: C has {} at {}, Rust has {} at {}",
            seen.0, seen.1, wanted.0, wanted.1
        );
    }
}

#[test]
fn the_host_table_is_laid_out_the_same_in_both_languages() {
    same(
        HOST,
        "IcHost",
        vec![
            ("magic", std::mem::offset_of!(IcHost, magic)),
            ("struct_size", std::mem::offset_of!(IcHost, struct_size)),
            ("abi_version", std::mem::offset_of!(IcHost, abi_version)),
            ("host_version", std::mem::offset_of!(IcHost, host_version)),
            ("log_warn", std::mem::offset_of!(IcHost, log_warn)),
            ("language", std::mem::offset_of!(IcHost, language)),
            (
                "register_locales",
                std::mem::offset_of!(IcHost, register_locales),
            ),
            (
                "register_plugin_asset",
                std::mem::offset_of!(IcHost, register_plugin_asset),
            ),
            ("settings_read", std::mem::offset_of!(IcHost, settings_read)),
            (
                "settings_write",
                std::mem::offset_of!(IcHost, settings_write),
            ),
            ("ask", std::mem::offset_of!(IcHost, ask)),
            (
                "add_toolbar_button",
                std::mem::offset_of!(IcHost, add_toolbar_button),
            ),
            (
                "set_default_toolbar_visible",
                std::mem::offset_of!(IcHost, set_default_toolbar_visible),
            ),
            (
                "add_header_button",
                std::mem::offset_of!(IcHost, add_header_button),
            ),
            (
                "set_header_label",
                std::mem::offset_of!(IcHost, set_header_label),
            ),
            (
                "set_header_icon",
                std::mem::offset_of!(IcHost, set_header_icon),
            ),
            (
                "set_header_visible",
                std::mem::offset_of!(IcHost, set_header_visible),
            ),
            ("selection", std::mem::offset_of!(IcHost, selection)),
            (
                "register_panel_source",
                std::mem::offset_of!(IcHost, register_panel_source),
            ),
            (
                "open_panel_source",
                std::mem::offset_of!(IcHost, open_panel_source),
            ),
            (
                "close_panel_source",
                std::mem::offset_of!(IcHost, close_panel_source),
            ),
            (
                "register_panel_action",
                std::mem::offset_of!(IcHost, register_panel_action),
            ),
            (
                "register_panel_tree",
                std::mem::offset_of!(IcHost, register_panel_tree),
            ),
            (
                "register_drive_source",
                std::mem::offset_of!(IcHost, register_drive_source),
            ),
            (
                "drives_changed",
                std::mem::offset_of!(IcHost, drives_changed),
            ),
            (
                "register_connection_kind",
                std::mem::offset_of!(IcHost, register_connection_kind),
            ),
            (
                "register_pinned_connections",
                std::mem::offset_of!(IcHost, register_pinned_connections),
            ),
            (
                "pinned_connections_changed",
                std::mem::offset_of!(IcHost, pinned_connections_changed),
            ),
            (
                "register_filesystem",
                std::mem::offset_of!(IcHost, register_filesystem),
            ),
            (
                "register_fs_action",
                std::mem::offset_of!(IcHost, register_fs_action),
            ),
            ("fs_invalidate", std::mem::offset_of!(IcHost, fs_invalidate)),
            ("fs_changed", std::mem::offset_of!(IcHost, fs_changed)),
            ("fs_caps", std::mem::offset_of!(IcHost, fs_caps)),
            ("fs_open", std::mem::offset_of!(IcHost, fs_open)),
            ("fs_read", std::mem::offset_of!(IcHost, fs_read)),
            ("fs_seek", std::mem::offset_of!(IcHost, fs_seek)),
            ("fs_write", std::mem::offset_of!(IcHost, fs_write)),
            ("fs_truncate", std::mem::offset_of!(IcHost, fs_truncate)),
            ("fs_close", std::mem::offset_of!(IcHost, fs_close)),
            ("fs_list", std::mem::offset_of!(IcHost, fs_list)),
            ("fs_local_path", std::mem::offset_of!(IcHost, fs_local_path)),
            ("register_view", std::mem::offset_of!(IcHost, register_view)),
            ("open_view", std::mem::offset_of!(IcHost, open_view)),
            (
                "view_invalidate",
                std::mem::offset_of!(IcHost, view_invalidate),
            ),
            (
                "canvas_invalidate",
                std::mem::offset_of!(IcHost, canvas_invalidate),
            ),
            (
                "register_viewer",
                std::mem::offset_of!(IcHost, register_viewer),
            ),
        ],
    );
}

#[test]
fn the_filesystem_table_is_laid_out_the_same() {
    same(
        FS,
        "IcFsVTable",
        vec![
            ("struct_size", std::mem::offset_of!(IcFsVTable, struct_size)),
            ("open_in", std::mem::offset_of!(IcFsVTable, open_in)),
            ("close", std::mem::offset_of!(IcFsVTable, close)),
            ("list", std::mem::offset_of!(IcFsVTable, list)),
            ("read", std::mem::offset_of!(IcFsVTable, read)),
            (
                "is_read_only",
                std::mem::offset_of!(IcFsVTable, is_read_only),
            ),
            ("last_error", std::mem::offset_of!(IcFsVTable, last_error)),
            ("write", std::mem::offset_of!(IcFsVTable, write)),
            ("create_dir", std::mem::offset_of!(IcFsVTable, create_dir)),
            ("remove", std::mem::offset_of!(IcFsVTable, remove)),
            ("rename", std::mem::offset_of!(IcFsVTable, rename)),
            ("shell_open", std::mem::offset_of!(IcFsVTable, shell_open)),
            ("shell_read", std::mem::offset_of!(IcFsVTable, shell_read)),
            ("shell_write", std::mem::offset_of!(IcFsVTable, shell_write)),
            (
                "shell_resize",
                std::mem::offset_of!(IcFsVTable, shell_resize),
            ),
            ("shell_close", std::mem::offset_of!(IcFsVTable, shell_close)),
            (
                "shell_available",
                std::mem::offset_of!(IcFsVTable, shell_available),
            ),
            ("columns", std::mem::offset_of!(IcFsVTable, columns)),
            ("list_rows", std::mem::offset_of!(IcFsVTable, list_rows)),
            (
                "action_state",
                std::mem::offset_of!(IcFsVTable, action_state),
            ),
            (
                "cell_clicked",
                std::mem::offset_of!(IcFsVTable, cell_clicked),
            ),
            (
                "set_permissions",
                std::mem::offset_of!(IcFsVTable, set_permissions),
            ),
        ],
    );
}

#[test]
fn the_window_and_connection_tables_are_laid_out_the_same() {
    same(
        VIEW,
        "IcViewVTable",
        vec![
            (
                "struct_size",
                std::mem::offset_of!(IcViewVTable, struct_size),
            ),
            ("describe", std::mem::offset_of!(IcViewVTable, describe)),
            ("on_event", std::mem::offset_of!(IcViewVTable, on_event)),
            ("closed", std::mem::offset_of!(IcViewVTable, closed)),
        ],
    );
    same(
        CONNECTION,
        "IcConnectionVTable",
        vec![
            (
                "struct_size",
                std::mem::offset_of!(IcConnectionVTable, struct_size),
            ),
            ("open", std::mem::offset_of!(IcConnectionVTable, open)),
            ("fs", std::mem::offset_of!(IcConnectionVTable, fs)),
            (
                "describe",
                std::mem::offset_of!(IcConnectionVTable, describe),
            ),
            (
                "on_event",
                std::mem::offset_of!(IcConnectionVTable, on_event),
            ),
        ],
    );
}

/// The table a plugin hands over to show a file, which is a view with a file
/// behind it.
#[test]
fn the_viewer_table_is_laid_out_the_same() {
    same(
        Table::Viewer,
        "IcViewerVTable",
        vec![
            (
                "struct_size",
                std::mem::offset_of!(IcViewerVTable, struct_size),
            ),
            ("view", std::mem::offset_of!(IcViewerVTable, view)),
            ("open", std::mem::offset_of!(IcViewerVTable, open)),
            ("closed", std::mem::offset_of!(IcViewerVTable, closed)),
            ("content", std::mem::offset_of!(IcViewerVTable, content)),
            ("closing", std::mem::offset_of!(IcViewerVTable, closing)),
            (
                "canvas_ready",
                std::mem::offset_of!(IcViewerVTable, canvas_ready),
            ),
            (
                "canvas_draw",
                std::mem::offset_of!(IcViewerVTable, canvas_draw),
            ),
            (
                "canvas_gone",
                std::mem::offset_of!(IcViewerVTable, canvas_gone),
            ),
        ],
    );
}

#[test]
fn what_a_plugin_says_about_itself_is_laid_out_the_same() {
    same(
        ABOUT,
        "IcAbout",
        vec![
            ("magic", std::mem::offset_of!(IcAbout, magic)),
            ("struct_size", std::mem::offset_of!(IcAbout, struct_size)),
            ("abi_version", std::mem::offset_of!(IcAbout, abi_version)),
            ("id", std::mem::offset_of!(IcAbout, id)),
            ("name", std::mem::offset_of!(IcAbout, name)),
            ("version", std::mem::offset_of!(IcAbout, version)),
            ("description", std::mem::offset_of!(IcAbout, description)),
        ],
    );
    same(
        ENTRY,
        "IcDirEntry",
        vec![
            ("name", std::mem::offset_of!(IcDirEntry, name)),
            ("is_dir", std::mem::offset_of!(IcDirEntry, is_dir)),
            ("size", std::mem::offset_of!(IcDirEntry, size)),
            ("modified", std::mem::offset_of!(IcDirEntry, modified)),
            ("permissions", std::mem::offset_of!(IcDirEntry, permissions)),
            (
                "has_permissions",
                std::mem::offset_of!(IcDirEntry, has_permissions),
            ),
        ],
    );
}

#[test]
fn every_struct_is_the_same_size_in_both_languages() {
    let sizes = as_c_sees_it(SIZES);
    let mine: Vec<(&str, usize)> = vec![
        ("IcHost", std::mem::size_of::<IcHost>()),
        ("IcFsVTable", std::mem::size_of::<IcFsVTable>()),
        ("IcViewVTable", std::mem::size_of::<IcViewVTable>()),
        (
            "IcConnectionVTable",
            std::mem::size_of::<IcConnectionVTable>(),
        ),
        ("IcAbout", std::mem::size_of::<IcAbout>()),
        ("IcDirEntry", std::mem::size_of::<IcDirEntry>()),
        ("IcListing", std::mem::size_of::<IcListing>()),
        ("IcBytes", std::mem::size_of::<IcBytes>()),
        ("IcSelectionItem", std::mem::size_of::<IcSelectionItem>()),
        ("IcSelection", std::mem::size_of::<IcSelection>()),
        ("IcColumn", std::mem::size_of::<IcColumn>()),
        ("IcTable", std::mem::size_of::<IcTable>()),
        ("IcTree", std::mem::size_of::<IcTree>()),
        ("IcDrive", std::mem::size_of::<IcDrive>()),
        ("IcDrives", std::mem::size_of::<IcDrives>()),
        (
            "IcPinnedConnection",
            std::mem::size_of::<IcPinnedConnection>(),
        ),
        (
            "IcPinnedConnections",
            std::mem::size_of::<IcPinnedConnections>(),
        ),
        ("IcFsColumn", std::mem::size_of::<IcFsColumn>()),
        ("IcColumns", std::mem::size_of::<IcColumns>()),
        ("IcRow", std::mem::size_of::<IcRow>()),
        ("IcRows", std::mem::size_of::<IcRows>()),
        ("IcViewerVTable", std::mem::size_of::<IcViewerVTable>()),
        ("IcCanvas", std::mem::size_of::<IcCanvas>()),
        ("IcFrame", std::mem::size_of::<IcFrame>()),
    ];
    assert_eq!(sizes.len(), mine.len());
    for (seen, wanted) in sizes.iter().zip(mine.iter()) {
        assert_eq!(seen.0, wanted.0);
        assert_eq!(
            seen.1, wanted.1,
            "{}: C says {} bytes, Rust {}",
            seen.0, seen.1, wanted.1
        );
    }
}

/// An optional slot is a null pointer in C. Rust writes it as `Option<fn>`,
/// which is the same word only because a function pointer can never be null —
/// the guarantee the whole header rests on.
#[test]
fn an_absent_slot_is_a_null_pointer() {
    assert_eq!(
        std::mem::size_of::<Option<IcFsWriteFn>>(),
        std::mem::size_of::<*const std::os::raw::c_void>()
    );
    let absent: Option<IcFsWriteFn> = None;
    let as_word: usize = unsafe { std::mem::transmute(absent) };
    assert_eq!(as_word, 0);
}

#[test]
fn the_constants_in_the_header_are_the_constants_in_the_crate() {
    let mine: [(&str, u64); 8] = [
        ("IC_HOST_MAGIC", IC_HOST_MAGIC),
        ("IC_ABI_VERSION", IC_ABI_VERSION as u64),
        ("IC_ABOUT_MAGIC", IC_ABOUT_MAGIC),
        ("IC_TREE_MAGIC", IC_TREE_MAGIC),
        ("IC_DRIVES_MAGIC", IC_DRIVES_MAGIC),
        ("IC_PINNED_MAGIC", IC_PINNED_MAGIC),
        ("IC_COLUMNS_MAGIC", IC_COLUMNS_MAGIC),
        ("IC_ROWS_MAGIC", IC_ROWS_MAGIC),
    ];
    assert_eq!(constants_in_the_header(), mine);
}

#[test]
fn the_words_in_the_header_are_the_words_in_the_crate() {
    let mine = [
        IC_HOST_GTK,
        IC_HOST_WEB,
        IC_HOST_CONSOLE,
        IC_CELL_TICKED,
        IC_EVENT_OPENED,
        IC_EVENT_ACTIVATE,
        IC_EVENT_CHANGE,
        IC_EVENT_EXPAND,
        IC_EVENT_COLLAPSE,
        IC_EVENT_ENDED,
    ];
    assert_eq!(words_in_the_header(), mine);
}
