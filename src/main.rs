mod mount;
mod shell;

fn main() {
    unsafe {
        // file system
        mount::mount_fs("proc", "/proc", "proc", 0);
        mount::mount_fs("sysfs", "/sys", "sysfs", 0);
        mount::mount_fs("devtmpfs", "/dev", "devtmpfs", 0);

        // console
        let console = std::ffi::CString::new("/dev/console").unwrap();
        let fd = libc::open(console.as_ptr(), libc::O_RDWR);

        libc::dup2(fd, 0); // stdin
        libc::dup2(fd, 1); // stdout
        libc::dup2(fd, 2); // stderr

        // shell
        shell::spawn();
    }

    loop {
        let mut status = 0;
        let pid = unsafe { libc::waitpid(-1, &mut status, 0) };

        if pid > 0 {
        } else if pid == -1 {
        }
    }
}
