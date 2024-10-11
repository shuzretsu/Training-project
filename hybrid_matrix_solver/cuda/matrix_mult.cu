// Matrix Multiplication CUDA

#include <cuda_runtime.h>

extern "C" void matrix_vector_mult_cuda(float* A, float* x, float* b, int n);

__global__ void matvec_kernel(float* A, float* x, float* b, int n) {
    int row = blockIdx.x * blockDim.x + threadIdx.x;
    if (row < n) {
        float sum = 0.0f;
        for (int j = 0; j < n; ++j) {
            sum += A[row * n + j] * x[j];
        }
        b[row] = sum;
    }
}

extern "C" void matrix_vector_mult_cuda(float* A, float* x, float* b, int n) {
    float *d_A, *d_x, *d_b;
    
    // Allocate memory on the device (GPU)
    cudaMalloc((void**)&d_A, n * n * sizeof(float));
    cudaMalloc((void**)&d_x, n * sizeof(float));
    cudaMalloc((void**)&d_b, n * sizeof(float));

    // Copy data from host (CPU) to device (GPU)
    cudaMemcpy(d_A, A, n * n * sizeof(float), cudaMemcpyHostToDevice);
    cudaMemcpy(d_x, x, n * sizeof(float), cudaMemcpyHostToDevice);

    // Launch the kernel with n threads whic one for each row of matrix A
    int blockSize = 256; // we can adjust thee block size according to your GPU architecture(search on google about that)
    int gridSize = (n + blockSize - 1) / blockSize;
    matvec_kernel<<<gridSize, blockSize>>>(d_A, d_x, d_b, n);

    // Copy result from (GPU) to host (CPU)
    cudaMemcpy(b, d_b, n * sizeof(float), cudaMemcpyDeviceToHost);

    // Free the device memory
    cudaFree(d_A);
    cudaFree(d_x);
    cudaFree(d_b);
}
