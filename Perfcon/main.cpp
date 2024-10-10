#include "system.h"
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>  // passing C++ data to Python

namespace py = pybind11;

int main() {
    PerformanceMonitor monitor;

    // Simulate multiple cycles(data collection)
    for (int i = 0; i < 10; ++i) {
        monitor.gatherData();
    }

    // Display gathered report
    monitor.displayPerformanceReport();

    // Export memory usage data to Python for ml processig
    std::vector<int> memoryData = monitor.exportMemoryData();

    // Initialize Python interpreter
    py::scoped_interpreter guard{};

    // Import and call Python ml module
    py::module_ ml = py::module_::import("ml");
    ml.attr("analyze_memory")(memoryData);

    return 0;
}
