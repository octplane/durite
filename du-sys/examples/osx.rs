extern crate du_sys as du;

use std::ffi::CString;
use du::{StatFs, statfs64};


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
    }

}


fn main() {
    lets_get_kraken("/".to_string());
}
