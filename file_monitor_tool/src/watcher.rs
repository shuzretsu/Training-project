use libc::{inotify_init1, inotify_add_watch, inotify_event, IN_MODIFY, IN_CREATE, IN_DELETE};
use std::os::unix::io::AsRawFd;
use std::io::Read;

pub fn init_inotify() -> i32 {
    unsafe {
        inotify_init1(0)
    }
}

pub fn add_watch(inotify_fd: i32, path: &str) -> i32 {
    unsafe {
        inotify_add_watch(inotify_fd, path.as_ptr() as *const i8, IN_MODIFY | IN_CREATE | IN_DELETE)
    }
}

pub fn monitor_directory(inotify_fd: i32) {
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = unsafe { libc::read(inotify_fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };
        if bytes_read > 0 {
            let event = unsafe { &*(buffer.as_ptr() as *const inotify_event) };
            println!("Event detected: {:?}", event);
        }
    }
}
