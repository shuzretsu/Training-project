use std::fs::OpenOptions;
use std::sync::mpsc::Receiver;
use std::thread;
use std::io::Write;

pub fn start_logging_thread(receiver: Receiver<String>) {
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            log_event(&event);
        }
    });
}

fn log_event(event: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("file_monitor.log")
        .unwrap();

    writeln!(file, "{}", event).unwrap();
}
