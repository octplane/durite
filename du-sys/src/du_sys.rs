extern crate libc;
use libc::*;
use std::ptr;
use std::default::Default;
use std::ffi::CStr;

use std::ffi::CString;

use std::collections::hash_map::HashMap;
use std::fmt;

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


#[repr(C)]
pub struct StatFs {
    pub f_bsize:     uint32_t,
    pub f_iosize:    int32_t,
    pub f_blocks:    uint64_t,
    pub f_bfree:     uint64_t,
    pub f_bavail:    uint64_t,
    pub f_files:     uint64_t,
    pub f_ffree:     uint64_t,
    pub f_fsid:      [uint32_t; 2],
    pub f_owner:     uid_t,
    pub f_type:      uint32_t,
    pub f_flags:     uint32_t,
    pub f_fssubtype: uint32_t,
    pub f_fstypename:   [c_char; 16],
    pub f_mntonname:    [c_char; 1024],
    pub f_mntfromname:  [c_char; 1024],
    pub f_reserved:     [uint32_t; 8],
}

impl fmt::Debug for StatFs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

// -        println!("bfree {:?}", st.f_bfree);
// -        println!("bavail {:?}", st.f_bavail);
// -        println!("fs_type_name {:?}", st.fstypename());
// -        println!("f_mntfromname {:?}", st.mntfromname());
// -        println!("f_mntonname {:?}", st.mntonname());
// -        println!("ffree {:?}",  st.f_ffree);

        write!(f, "on {:?} as {:?} [{:?}/{:?}]", self.mntonname(), self.fstypename(), self.f_bfree, self.f_blocks)
    }
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
            f_fssubtype: 0,
            f_fstypename: [0; 16],
            f_mntonname:  [0; 1024],
            f_mntfromname:[0; 1024],
            f_reserved: [0; 8]
        }
    }
}

impl StatFs {
    pub fn fstypename(&self) -> String {
        let v = unsafe { CStr::from_ptr(&self.f_fstypename as *const i8).to_bytes().to_vec() };
        String::from_utf8(v).unwrap()
    }

    pub fn mntonname(&self) -> String {
        let v = unsafe { CStr::from_ptr(&self.f_mntonname as *const i8).to_bytes().to_vec() };
        String::from_utf8(v).unwrap()
        // let foo: [u8; 1024] = unsafe { transmute(self.f_mntfromname)};
        // String::from_utf8_lossy(&foo).to_string()
    }
    pub fn  mntfromname(&self) -> String {
        let v = unsafe { CStr::from_ptr(&self.f_mntfromname as *const i8).to_bytes().to_vec() };
        String::from_utf8(v).unwrap()
    }
}

extern {
    pub fn statfs64(path: *const c_char, stafs: *mut StatFs) -> size_t;
    // statfs(const char *path, struct statfs *buf);
}


pub type CFIndex = uint64_t;
pub type ClassPointer = *mut libc::c_void;
pub type SEL = *mut libc::c_void;
pub type NSArray = ClassPointer;
pub type CFArrayRef = ClassPointer;

#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex
}


#[link(name = "CoreServices", kind = "framework")]
#[link(name = "objc")]
extern {
    pub fn objc_getClass (name: *const c_char) -> ClassPointer;
    pub fn objc_msgSend(id: ClassPointer, sel: SEL, parm: *const c_char) -> ClassPointer;
    pub fn sel_getUid (selector: *const c_char) -> SEL;
    pub fn CFShow(source: ClassPointer);
    pub fn CFArrayGetValues (theArray: CFArrayRef, range: CFRange , values: *const *const libc::c_char );
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    //(NSArray *)propertyKeys options:(NSVolumeEnumerationOptions)options
    pub fn mountedVolumeURLsIncludingResourceValuesForKeys(propertyKeys: *const c_char, options: *const c_char) -> NSArray;

}

pub fn call_selector_on(object: ClassPointer, selector: &str) -> ClassPointer {
    unsafe {
        let csel = CString::new(selector).unwrap();
        let selector = sel_getUid (csel.as_ptr());
        let r = objc_msgSend(object, selector, std::ptr::null()); // => ok
        r
    }
}

pub fn mounted_volume_stats() -> HashMap<String, StatFs> {

    let mut ret = HashMap::new();
    for v in mounted_volume_urls() {
        if v.starts_with("file:///") {
            let mount_point: String = v.chars().skip(7).collect();
            let mut st: StatFs = Default::default();
            let mp = CString::new(mount_point.clone().into_bytes()).unwrap();
            unsafe {
                let r = statfs64(mp.as_ptr(), &mut st);
                if r==0 {
                    ret.insert(mount_point, st);
                }
            }
        }
    }
    ret
}

pub fn mounted_volume_urls() -> Vec<String> {
    let mut ret: Vec<String> = vec!();
    let ns = CString::new("NSFileManager").unwrap();
    unsafe {
        let file_manager_class = objc_getClass(ns.as_ptr());
        let default_manager = call_selector_on(file_manager_class, "defaultManager");

        let array = call_selector_on(default_manager, "mountedVolumeURLsIncludingResourceValuesForKeys:options:");
        let size: uint64_t = CFArrayGetCount(array);
        let range = CFRange{location: 0, length: size };
        let values: Vec<*const c_char> = vec![ptr::null(); size as usize];
        CFArrayGetValues(array, range, values.as_ptr());
        for p in values {
            let nstring = call_selector_on(p as ClassPointer, "absoluteString");
            let cstring = call_selector_on(nstring as ClassPointer, "UTF8String");
            let cstr = CStr::from_ptr(cstring as *const c_char);
            let str = std::str::from_utf8(cstr.to_bytes()).unwrap().to_string();
            ret.push(str);
        }
    }
    ret
}
