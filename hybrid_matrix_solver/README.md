
# Hybrid Matrix Solver with GPU Support

I created a project that implements a **hybrid matrix solver** that leverages **Rust** for user interaction and I/O, and **Fortran with GPU support** for efficient matrix computations. The solver uses the **Conjugate Gradient method** for solving large, sparse, symmetric positive-definite linear systems (`Ax = b`).

## What it does?

- Input for matrix dimensions and elements via Rust.
- GPU-accelerated matrix-vector multiplication using **OpenACC** in Fortran, with CUDA support.
- Modular design with easy-to-use FFI bindings between Rust and Fortran.

---

## Prerequisites

Before compiling and running this project, ensure you have the following software installed:

1. **Rust**: The Rust programming language and Cargo (the Rust package manager).
   
   Install Rust and Cargo using [rustup](https://rustup.rs/):
   
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Fortran**: A Fortran compiler. For example, you can use the GNU Fortran compiler `gfortran` (commonly included with GCC).

   Install `gfortran` on Linux:

   - **Ubuntu/Debian**:
     ```bash
     sudo apt-get install gfortran
     ```

3. **NVIDIA HPC SDK**: Fortran code uses **OpenACC** for GPU acceleration. You will need the **NVIDIA HPC SDK** to compile the OpenACC directives for GPU offloading.

   Download in the [NVIDIA HPC SDK](https://developer.nvidia.com/hpc-sdk) and follow their instructions for installation. 👍


4. **CUDA Toolkit**: If you're using an NVIDIA GPU, you need the CUDA toolkit installed for GPU computations. Follow the [installation guide](https://developer.nvidia.com/cuda-downloads).

---

## Compilation Instructions

### 1. **Compiling the Fortran Code**

Compile the Fortran code with OpenACC for GPU support:

```bash
pgfortran -acc -Minfo=accel -c fortran/matrix_solver.f90 -o matrix_solver.o
```

NOTES : If **GPU support is not required** you can compile the Fortran code using `gfortran` (without OpenACC):

```bash
gfortran -c fortran/matrix_solver.f90 -o matrix_solver.o
```

### 2. **Compiling the Rust Code**

Next, compile the Rust code and link it to the Fortran object file:

```bash
rustc src/main.rs -L . -l matrix_solver -o hybrid_solver
```

### 3. **Running the Program**

After compilation, you can run the program:

```bash
./hybrid_solver
```

---

## HOW TO USE IT?

Once the program is compiled and running, follow ths steps to input the matrix dimensions and elements:

1. **Matrix Size**: The program will first prompt you to input the matrix size (for an `n x n` matrix).

   Example:
   ```
   Enter the size of the matrix (n for an n x n matrix): 3
   ```

2. **Matrix Elements**: Then you will input the matrix elements row by row.

   Example:
   ```
   Enter matrix element (1, 1): 4.0
   Enter matrix element (1, 2): 1.0
   Enter matrix element (1, 3): 0.0
   ...etc...
   ```

3. **Vector b**: Input the elements of vector `b`.

   Example:
   ```
   Enter element b[1]: 1.0
   Enter element b[2]: 2.0
   Enter element b[3]: 3.0
   ```

4. **Solution**: The program will compute the solution `x` for the system `Ax = b` and display the result.

   Example output:
   ```
   Solution: x = [0.0909, 0.6363, 1.5]
   ```

---

## GPU Support (OpenACC)

The **matrix-vector multiplication** in this program is accelerated on the GPU using **OpenACC** directives. Ensure you have the correct compiler and CUDA drivers installed to enable the feature. The matrix size and computational intensity will benefit most from GPU offloading, especially for large-scale systems.

You can view **GPU activity** and verify that GPU acceleration is being utilized by running nvidia-smi

```bash
nvidia-smi
```

Make sure the `pgfortran` compiler used the `-acc` flag and compiled the matrix multiplication for the GPU.

---

## Troubleshooting

If you encounter issues, they may be associated with:

1. **OpenACC**: If you encounter problems with offloading to the GPU, check that your compiler is properly installd and supports OpenACC. You maybe need to recompile with additional debugging options (`-Minfo=accel`).

2. **Compilation Errors**: Ensure all dependencies like Rust, Fortran, PGI/NVIDIA compilers are properly installed and in configured paths.

3. **Performance**: If performance is not as expected, ensure your matrix size is large enough to benefit from GPU acceleration.

---
---
---
---

## Notes for me :
Future Enhancements (maybe)

- **File Input Support**: Extend the program to read matrix and vector inputs from a file.
- **Different Solvers**: Implement additional matrix solvers (e.g., LU decomposition or Cholesky).
- **Parallelism**: Add multi-threading support on the CPU side via OpenMP or Rust’s async/await for even greater performance.
