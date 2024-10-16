### PerfCon (Performance Control System)

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
