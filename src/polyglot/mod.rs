mod java;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub const PATH_SEPARATOR: char = ';';
    } else {
        pub const PATH_SEPARATOR: char = ':';
    }
}

pub fn inject_polyglot() {
    if java::is_available() {
        java::init_env();
    }
}
