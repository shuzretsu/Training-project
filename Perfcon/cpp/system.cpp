#include <iostream>
#include <vector>
#include <numeric> 
#include <algorithm> 

class PerformanceMonitor {
public:
    PerformanceMonitor() {}

    // Gather system data > CPU usage, Memory, I/O, etc.
    void gatherData() {
        // Simulate gathering data from system sensors
        cpuUsage.push_back(rand() % 100);   // Random CPU usage between 0-100%
        memoryUsage.push_back(rand() % 100); // Random memory usage
    }

    // Calculate average CPU usage 
    double calculateAverageCPU() {
        return std::accumulate(cpuUsage.begin(), cpuUsage.end(), 0.0) / cpuUsage.size();
    }

    // Calculate peak memory usage
    int calculatePeakMemory() {
        return *std::max_element(memoryUsage.begin(), memoryUsage.end());
    }

    void displayPerformanceReport() {
        std::cout << "Average CPU Usage: " << calculateAverageCPU() << "%" << std::endl;
        std::cout << "Peak Memory Usage: " << calculatePeakMemory() << "%" << std::endl;
    }

    // Export data to Python ML
    std::vector<int> exportMemoryData() {
        return memoryUsage;
    }

private:
    std::vector<int> cpuUsage;
    std::vector<int> memoryUsage;
};
