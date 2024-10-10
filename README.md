# Training Project

This repository will contains various simple projects I make to enhance and explore different programming

## Projects List

### 1. PerfCon (Performance Control System)

**Description**:  
PerfCon is a hybrid system that combines the efficiency of C++ for performance monitoring with the power of Python for machine learning analysis. It tracks system resource usage (CPU and memory) and predicts future trends using machine learning models.

- **Languages**: C++, Python
- **Features**: 
  - Real-time monitors CPU and memory usage.
  - Provides report on average CPU and peak memory usage.
  - Exports data to Python for machine learning-based trend predictions.
  
**Usage**:  
Navigate to the `PerfCon` directory and follow the build instructions.

1. Build the C++ program:
   ```bash
   cd PerfCon/cpp
   mkdir build && cd build
   cmake ..
   make
   ```
   Install Library
   ```pip install -r PerfCon/py/requirements.txt```
   Run
   ```./PerfCon```
