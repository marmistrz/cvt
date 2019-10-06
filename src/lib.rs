cfg_if::cfg_if! {
    if #[cfg(target_os = "vxworks")] {
        mod vxworks;
        pub use self::vxworks::{cvt, cvt_r};
    } else if #[cfg(unix)] {
        mod unix;
        pub use self::unix::{cvt, cvt_r};
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::cvt;
    } else {
        compile_error!("cvt doesn't compile for this platform yet");
    }
}
