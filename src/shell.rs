pub unsafe fn spawn() {
    let pid = unsafe { libc::fork() };

    if pid == 0 {
        let shell = std::ffi::CString::new("/bin/sh").unwrap();
        let args = [shell.as_ptr(), std::ptr::null()];

        unsafe {
            libc::execv(shell.as_ptr(), args.as_ptr());
        }
        std::process::exit(1)
    } else if pid == -1 {
        eprintln!("[NullInit] fork failed!");
    }
}
