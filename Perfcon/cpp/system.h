#ifndef SYSTEM_H
#define SYSTEM_H

#include <vector>

// PerformanceMonitor class declaration
class PerformanceMonitor {
public:
    PerformanceMonitor();
    void gatherData();

    // Calculate average CPU usage ove time
    double calculateAverageCPU();

    // Calculate peak memory usage over time
    int calculatePeakMemory();

    // Display report of system performance
    void displayPerformanceReport();

    // Export memory usage data python ml
    std::vector<int> exportMemoryData();

private:
    std::vector<int> cpuUsage;   // Stores CPU usage data
    std::vector<int> memoryUsage;  // Stores memory usage data
};

#endif // SYSTEM_H
