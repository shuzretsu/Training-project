# Training Project

This repository will contains various simple projects I make to enhance and explore different programming

## Projects List

### 1. PerfCon (Performance Control System)

**Description**:  
PerfCon is a hybrid system that combines the efficiency of C++ for performance monitoring with the power of Python for machine learning analysis. It tracks system resource usage (CPU and memory) and predicts future trends using machine learning models.
- **What it does?**: 
  - Real-time monitors CPU and memory usage.
  - Provides report on average CPU and peak memory usage.
  - Exports data to Python for machine learning-based trend predictions.
  
**Usage**:  
Navigate to the `PerfCon` directory and follow the build instructions.

1. **Build the C++ program**:
   ```bash
   cd PerfCon/cpp
   mkdir build && cd build
   cmake ..
   make
   ```
2. **Install Library**:
   ```pip install -r PerfCon/py/requirements.txt```
3. **Run**:
   ```./PerfCon```


### 2. Real-time file integrity monitoring 

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
