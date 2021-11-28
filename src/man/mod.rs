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
            "../man/cat",

            "man",
            "../man/man",

            "grep",
            "../man/grep",

            "can",
            "../man/fs_handle/can",

            "cp",
            "../man/fs_handle/cp",

            "cpdir",
            "../man/fs_handle/cpdir",

            "ls",
            "../man/fs_handle/ls",

            "mkdir",
            "../man/fs_handle/mkdir",

            "rm",
            "../man/fs_handle/rm",

            "rmdir",
            "../man/fs_handle/rmdir",

            "touch",
            "../man/fs_handle/touch"
        );
        hmp
    };
}
