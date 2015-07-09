extern crate libc;
use libc::*;
use std::default::Default;

// #define MFSTYPENAMELEN  16  length of fs type name including null
// #define MAXPATHLEN      1024
// #define MNAMELEN        MAXPATHLEN
   // struct statfs { /* when _DARWIN_FEATURE_64_BIT_INODE is defined */
   //       uint32_t    f_bsize;        /* fundamental file system block size */
   //       int32_t     f_iosize;       /* optimal transfer block size */
   //       uint64_t    f_blocks;       /* total data blocks in file system */
   //       uint64_t    f_bfree;        /* free blocks in fs */
   //       uint64_t    f_bavail;       /* free blocks avail to non-superuser */
   //       uint64_t    f_files;        /* total file nodes in file system */
   //       uint64_t    f_ffree;        /* free file nodes in fs */
   //       fsid_t      f_fsid;         /* file system id */
   //       uid_t       f_owner;        /* user that mounted the filesystem */
   //       uint32_t    f_type;         /* type of filesystem */
   //       uint32_t    f_flags;        /* copy of mount exported flags */
   //       uint32_t    f_fssubtype;    /* fs sub-type (flavor) */
   //       char        f_fstypename[MFSTYPENAMELEN];   /* fs type name */
   //       char        f_mntonname[MAXPATHLEN];        /* directory on which mounted */
   //       char        f_mntfromname[MAXPATHLEN];      /* mounted filesystem */
   //       uint32_t    f_reserved[8];  /* For future use */
   //   };


pub type Uid = uint32_t;


#[repr(C)]
pub struct StatFs {
    pub f_bsize: uint32_t,
    pub f_iosize: int32_t,
    pub f_blocks: uint64_t,
    pub f_bfree:  uint64_t,
    pub f_bavail: uint64_t,
    pub f_files:  uint64_t,
    pub f_ffree:  uint64_t,
    pub f_fsid:   [uint32_t; 2],
    pub f_owner:  Uid,
    pub f_type:   uint32_t,
    pub f_flags:  uint32_t,
    pub f_fstypename:   [c_char; 16],
    pub f_mntonname:    [c_char; 1024],
    pub f_mntfromname:  [c_char; 1024],
    pub f_reserved:     [uint32_t; 8],
}

impl Default for StatFs {
    fn default() -> StatFs {
        StatFs{
            f_bsize: 0,
            f_iosize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_fsid: [0; 2],
            f_owner: 0,
            f_type: 0,
            f_flags: 0,
            f_fstypename: [0; 16],
            f_mntonname:  [0; 1024],
            f_mntfromname:[0; 1024],
            f_reserved: [0; 8]
        }
    }
}

extern {
    pub fn statfs(path: *const c_char, stafs: *mut StatFs) -> size_t;
    // statfs(const char *path, struct statfs *buf);
}
