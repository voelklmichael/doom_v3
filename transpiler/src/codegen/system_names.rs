//! A small, corpus-specific table of C identifiers that this codebase never
//! declares itself - they come from system headers (`<X11/Xlib.h>`,
//! `<X11/extensions/XShm.h>`, `<stdio.h>`, `<netinet/in.h>`, `<netdb.h>`,
//! `<sys/time.h>`, `<signal.h>`, `<strings.h>`) that this transpiler doesn't
//! parse. Rather than build general system-header modeling, this maps the
//! exact, finite set of such names this corpus actually references (all in
//! `i_video.c`/`i_net.c`/`i_sound.c`/`w_wad.c`/`z_zone.c`/`doomstat.h`/
//! `p_local.h`, confirmed via a real `cargo build -p doom_rs` run) to their
//! real, ABI-correct definitions - the `libc`/`x11` crates for anything with
//! a real external ABI, or a literal value for the two cases
//! (`IPPORT_USERRESERVED`, `MININT`/`MAXINT`) that are well-known constants
//! with no corresponding symbol in the `libc` crate for this target.
//!
//! `IPPORT_USERRESERVED` is a real gap in the `libc` crate itself on
//! Linux/glibc targets (present for some other Unix targets, e.g. hurd, but
//! not linux-gnu - confirmed by a real test compile) - its value (5000) is
//! fixed by the BSD sockets API this corpus's own comment quotes, so a
//! literal is exact, not a guess. `MININT`/`MAXINT` (`doomtype.h`) are
//! themselves conditionally `#include <values.h>` on `LINUX` rather than
//! `#define`d directly - `values.h`'s own `MAXINT`/`MININT` are just the
//! 32-bit int extremes, so `std::ffi::c_int::MAX`/`MIN` are exact.

/// Maps a `Type::Named` leaf's text to a fully-qualified Rust path, for
/// names this corpus's own parser never declares (no typedef/struct/enum of
/// this name exists anywhere in `linuxdoom-1.10/`). Checked by
/// `types::classify_named` before falling back to "pass the name through
/// verbatim, assuming it's one of *this* corpus's own types".
pub fn system_type(name: &str) -> Option<&'static str> {
    Some(match name {
        "FILE" => "libc::FILE",
        "sockaddr_in" => "libc::sockaddr_in",
        "Display" => "x11::xlib::Display",
        "Window" => "x11::xlib::Window",
        "Colormap" => "x11::xlib::Colormap",
        "Visual" => "x11::xlib::Visual",
        "GC" => "x11::xlib::GC",
        "XEvent" => "x11::xlib::XEvent",
        "XVisualInfo" => "x11::xlib::XVisualInfo",
        "XImage" => "x11::xlib::XImage",
        "XShmSegmentInfo" => "x11::xshm::XShmSegmentInfo",
        "XColor" => "x11::xlib::XColor",
        "Cursor" => "x11::xlib::Cursor",
        _ => return None,
    })
}

/// Maps a bare value-position identifier (a macro/expression's own
/// `Expr::Ident`, including a call's callee - see
/// `codegen::expr::render_ident`) to a Rust expression fragment. Also
/// consulted by `codegen::macros::has_unresolved_ident` so an object-like
/// macro whose body references one of these (`i_sound.c`'s `itimer =
/// ITIMER_REAL`/`sig = SIGALRM`, `i_net.c`'s `DOOMPORT =
/// IPPORT_USERRESERVED + 0x1d`, `p_local.h`'s `ONFLOORZ = MININT`/
/// `ONCEILINGZ = MAXINT`) isn't mistaken for dead code referencing a
/// genuinely undefined name. Deliberately excludes callable names (see
/// `system_function`) - those still render correctly through this same
/// `render_ident` path when actually *called*, but a bare, uncalled
/// function reference isn't a scalar value, so keeping the two tables
/// separate lets `codegen::macros::emit_define_object` tell "resolves, and
/// is a value" apart from "resolves, but is a function" (see
/// `is_bare_function_reference`).
pub fn system_value(name: &str) -> Option<&'static str> {
    Some(match name {
        "ITIMER_REAL" => "libc::ITIMER_REAL",
        "SIGALRM" => "libc::SIGALRM",
        "IPPORT_USERRESERVED" => "5000",
        "MININT" => "std::ffi::c_int::MIN",
        "MAXINT" => "std::ffi::c_int::MAX",
        _ => return None,
    })
}

/// Maps a callable system-header name to its Rust path - checked by
/// `render_ident` too (so a *call*, e.g. a hypothetical
/// `strcasecmp(a, b)`, still renders correctly), but tracked separately
/// from `system_value` since a bare, uncalled reference to one of these
/// (`w_wad.c`'s real `#define strcmpi strcasecmp` - a function-alias
/// macro, not a scalar constant) can't become a scalar-typed Rust `const`
/// the way `system_value`'s names can - `codegen::macros::
/// emit_define_object` degrades that shape instead of guessing a
/// translation for it.
pub fn system_function(name: &str) -> Option<&'static str> {
    Some(match name {
        "strcasecmp" => "libc::strcasecmp",
        _ => return None,
    })
}

#[cfg(test)]
mod tests;
