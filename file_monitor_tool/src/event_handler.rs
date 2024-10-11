use std::thread;
use std::sync::mpsc::Sender;

pub fn start_monitoring_thread(inotify_fd: i32, sender: Sender<String>) {
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
            let bytes_read = unsafe { libc::read(inotify_fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };
            if bytes_read > 0 {
                sender.send("File event detected!".to_string()).unwrap();
            }
        }
    });
}
