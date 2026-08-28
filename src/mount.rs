use std::ffi::CString;
use std::ptr;

pub unsafe fn mount_fs(source: &str, target: &str, fstype: &str, flags: libc::c_ulong) -> i32 {
    let src = CString::new(source).unwrap();
    let tgt = CString::new(target).unwrap();
    let fst = CString::new(fstype).unwrap();

    unsafe { libc::mount(src.as_ptr(), tgt.as_ptr(), fst.as_ptr(), flags, ptr::null()) }
}
