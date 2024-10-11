// include/matrix_mult.h
#ifndef MATRIX_MULT_H
#define MATRIX_MULT_H

// Declaration of the CUDA matrix-vector multiplication function
extern "C" void matrix_vector_mult_cuda(float* A, float* x, float* b, int n);

#endif // MATRIX_MULT_H
