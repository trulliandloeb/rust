// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! FreeBSD-specific raw type definitions

#![stable(feature = "raw_ext", since = "1.1.0")]

#[stable(feature = "raw_ext", since = "1.1.0")] pub type off_t = i64;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type dev_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type ino_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type mode_t = u16;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type nlink_t = u16;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type blksize_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type blkcnt_t = i64;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type fflags_t = u32;

#[unstable(feature = "pthread_t", issue = "29791")] pub type pthread_t = usize;

#[doc(inline)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub use self::arch::{stat, time_t};

#[cfg(target_arch = "x86")]
mod arch {
    use super::{off_t, dev_t, ino_t, mode_t, nlink_t, blksize_t, blkcnt_t, fflags_t};
    use os::raw::c_long;
    use os::unix::raw::{uid_t, gid_t};

    #[stable(feature = "raw_ext", since = "1.1.0")] pub type time_t = i32;

    #[repr(C)]
    #[derive(Clone)]
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub struct stat {
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_dev: dev_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ino: ino_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mode: mode_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_nlink: nlink_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_uid: uid_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_gid: gid_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_rdev: dev_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_size: off_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blocks: blkcnt_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blksize: blksize_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_flags: fflags_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_gen: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_lspare: i32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub __unused: [u8; 8],
    }
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use super::{off_t, dev_t, ino_t, mode_t, nlink_t, blksize_t, blkcnt_t, fflags_t};
    use os::raw::c_long;
    use os::unix::raw::{uid_t, gid_t};

    #[stable(feature = "raw_ext", since = "1.1.0")] pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub struct stat {
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_dev: dev_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ino: ino_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mode: mode_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_nlink: nlink_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_uid: uid_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_gid: gid_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_rdev: dev_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_size: off_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blocks: blkcnt_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blksize: blksize_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_flags: fflags_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_gen: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_lspare: i32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtime: time_t,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtime_nsec: c_long,
    }
}

