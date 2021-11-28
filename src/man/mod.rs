use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

type ManualMap = FxHashMap<&'static str, &'static str>;

macro_rules! impl_man {
    ($hmp:tt, $($key:tt, $cmd:tt), *) => {
        $(
            $hmp.insert($key, $cmd);
        )*
    }
}

lazy_static! {
    pub static ref MANUALS: ManualMap = {
        let mut hmp: ManualMap = ManualMap::default();
        impl_man!(
            hmp,
            "cat",
            "/opt/man/cat",

            "man",
            "/opt/man/man",

            "grep",
            "/opt/man/grep",

            "can",
            "/opt/man/fs_handle/can",

            "cp",
            "/opt/man/fs_handle/cp",

            "cpdir",
            "/opt/man/fs_handle/cpdir",

            "ls",
            "/opt/man/fs_handle/ls",

            "mkdir",
            "/opt/man/fs_handle/mkdir",

            "rm",
            "/opt/man/fs_handle/rm",

            "rmdir",
            "/opt/man/fs_handle/rmdir",

            "touch",
            "/opt/man/fs_handle/touch"
        );
        hmp
    };
}
