mod config;
mod event_handler;
mod logger;
mod hasher;
mod queue;
mod watcher;

fn main() {
    // Load config
    let config = config::load_config();
    
    // nitialize file watcher
    let inotify_fd = watcher::init_inotify();

    // watch directory
    watcher::add_watch(inotify_fd, &config.monitor_dir);
    
    // event processing
    let (tx, rx) = std::sync::mpsc::channel();
    
    event_handler::start_monitoring_thread(inotify_fd, tx);
    logger::start_logging_thread(rx);
}
