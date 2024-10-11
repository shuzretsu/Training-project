### Real-time file integrity monitoring 

**Description**:  
It is a real-time file integrity monitoring tool built in Rust. It detects unauthorized changes in specified directories, logs events, and ensures system security with minimal overhead.

- **What it does?**:
  - Monitors directories for file modifications, creations, and deletions.
  - For fficient logging of detected events.

**Usage**:  
Follow the steps below to set up and run the tool.

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/SentinelRust.git
   cd SentinelRust
   ```
2. **Build project**:

    ```bash
    cargo build --release
    ```
3. **Set directory to monitor**:

    ```bash
    export MONITOR_DIR=/path/to/monitor
    ```
4. **Run**:

    ```bash
    cargo run --release
    ```
