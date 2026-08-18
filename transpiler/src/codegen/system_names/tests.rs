use super::{system_function, system_type, system_value};

#[test]
fn known_system_types_resolve() {
    assert_eq!(system_type("FILE"), Some("libc::FILE"));
    assert_eq!(system_type("Display"), Some("x11::xlib::Display"));
    assert_eq!(
        system_type("XShmSegmentInfo"),
        Some("x11::xshm::XShmSegmentInfo")
    );
}

#[test]
fn unknown_type_is_none() {
    assert_eq!(system_type("mobj_t"), None);
    assert_eq!(system_type("int"), None);
}

#[test]
fn known_system_values_resolve() {
    assert_eq!(system_value("SIGALRM"), Some("libc::SIGALRM"));
    assert_eq!(system_value("IPPORT_USERRESERVED"), Some("5000"));
    assert_eq!(system_value("MININT"), Some("std::ffi::c_int::MIN"));
    assert_eq!(system_value("MAXINT"), Some("std::ffi::c_int::MAX"));
}

#[test]
fn unknown_value_is_none() {
    assert_eq!(system_value("snd_MaxVolume"), None);
}

#[test]
fn strcasecmp_is_a_function_not_a_value() {
    // w_wad.c's real `#define strcmpi strcasecmp` needs this distinction -
    // see `codegen::macros::is_bare_function_reference`.
    assert_eq!(system_value("strcasecmp"), None);
    assert_eq!(system_function("strcasecmp"), Some("libc::strcasecmp"));
}
