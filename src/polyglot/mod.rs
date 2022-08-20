pub mod java;
pub mod node;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub const PATH_SEPARATOR: char = ';';
    } else {
        pub const PATH_SEPARATOR: char = ':';
    }
}

pub fn inject_languages() {
    if java::is_available() {
        java::init_env();
    }
    if node::is_available() {
        node::init_env();
    }
}
