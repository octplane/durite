extern crate du_sys as du;
extern crate libc;

#[cfg(not(any(target_os="macos")))]
fn lets_get_kraken() {}

#[cfg(any(target_os="macos"))]
fn lets_get_kraken() {
    println!("{:?}", du::mounted_volume_stats());
}


fn main() {
    lets_get_kraken();
}
