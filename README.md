# Ice Commander plugin API

The boundary between Ice Commander and its plugins, kept on its own so that writing a
plugin means depending on this and nothing else.

**This is a concept, not a finished product.** Nothing has been released yet, the ABI is
still being broken when a better shape turns up, and the repository is published so the
idea can be read and argued with rather than because it is ready to build against.

## The idea

A plugin is one shared library — `.so`, `.dll` or `.dylib` — that the application loads at
startup. It never draws anything itself. It *describes* what it offers as JSON, and
whichever frontend is running builds that: the desktop application, a terminal, or React in
a browser. The same library file therefore works in all three, and a plugin author does not
write a line of interface code for any of them.

What a plugin can offer: a filesystem you walk into as if it were a folder, a kind of
connection with a form to fill in, drives of its own beside the disks, windows, and a
viewer for files of an extension it claims.

## What is here

A plugin depends on one crate and nothing else:

| crate | what it is |
|---|---|
| `ic-plugin-api` | the C boundary itself: the table of functions a plugin is handed, the tables it hands back, and the rules both sides keep. No dependencies of its own, and it compiles into the plugin's own library |
| `include/ic_plugin.h` | the same boundary as a C header, for plugins not written in Rust |

Beside them: an example plugin in Rust, one in C, and a test that holds the header and the
Rust declarations to the same layout, field by field. The C example is the plainest
statement of what this boundary is — it depends on no crate at all, only on the header.

Nothing else lives here. The application's own side of the boundary, and the checker that
loads a plugin and reports what it registered, are part of Ice Commander rather than of the
contract, and a plugin needs neither to be built.

The current boundary is ABI version 1. The application refuses a plugin that declares another
ABI version rather than calling into it. Besides filesystems, connection kinds, toolbar windows,
panels and viewers, it carries:

- `IcFsVTable::set_permissions`: a filesystem that can chmod says so by filling it in;
- an `ended` event (`IC_EVENT_ENDED`) when a `media` node in a document plays to its end;
- `keys` in a document, keyboard shortcuts that reach the plugin as `activate` of a node
  without any button on screen: `"keys": [{ "accel": "Left", "node": "back" }]`.

## Writing a plugin

There is a separate SDK with worked examples — a filesystem, a connection kind, a window
from a toolbar button, a panel with columns of its own, a viewer — each one a small crate
that does a single thing and explains itself. Start there rather than here: this repository
is the contract, the SDK is how it is used.

## Building

```
./test.sh         # builds the C example, then runs the whole workspace
./build.sh        # the example plugins, into bin/
./build-c.sh      # the C example on its own, needs a C compiler
```

## Contributing

Patches carry a `Signed-off-by` line, per the Developer Certificate of Origin in `DCO`.
`CONTRIBUTING.md` says what that means and how to add it.

## Licence

MIT or Apache-2.0, at your option. See `LICENSE-MIT` and `LICENSE-APACHE`.
