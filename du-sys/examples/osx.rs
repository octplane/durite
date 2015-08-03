extern crate du_sys as du;

use std::ffi::CString;
use du::{StatFs, statfs64, objc_getClass, sel_getUid, objc_msgSend, CFShow};


#[cfg(not(any(target_os="macos")))]
fn lets_get_kraken(mount_point:String) {}

#[cfg(any(target_os="macos"))]
fn lets_get_kraken(mount_point: String) {
    let mut st: StatFs = Default::default();
    let mp = CString::new(mount_point.into_bytes()).unwrap();
    unsafe {
     let o = statfs64(mp.as_ptr(), &mut st);
     println!("bfree {:?}", st.f_bfree);
     println!("bavail {:?}", st.f_bavail);
     println!("fs_type_name {:?}", st.fstypename());
     println!("f_mntfromname {:?}", st.mntfromname());
     println!("f_mntonname {:?}", st.mntonname());
     println!("ffree {:?}",  st.f_ffree);

     let ns = CString::new("NSFileManager").unwrap();
     let dm_method = CString::new("defaultManager").unwrap();
     let file_manager_class = objc_getClass(ns.as_ptr());
     let default_manager_selector = sel_getUid(dm_method.as_ptr());
     let default_manager = objc_msgSend(file_manager_class, default_manager_selector, std::ptr::null());

     let mounted_method = CString::new("mountedVolumeURLsIncludingResourceValuesForKeys:options:").unwrap();
     let mounted_method_selector = sel_getUid(mounted_method.as_ptr());
     let array = objc_msgSend(default_manager, mounted_method_selector, std::ptr::null());

     println!("{:?}", file_manager_class);
     println!("{:?}", default_manager_selector);
     println!("{:?}", default_manager);
     CFShow(array);
    }

}


fn main() {
    lets_get_kraken("/".to_string());
}
