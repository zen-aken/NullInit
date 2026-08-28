mod mount;
mod shell;

fn main() {
    unsafe {
        mount::mount_fs("proc", "/proc", "proc", 0);
        mount::mount_fs("sysfs", "/sys", "sysfs", 0);
        mount::mount_fs("devtmpfs", "/dev", "devtmpfs", 0);

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
