use std::os::raw::c_double;
use std::ptr;
use std::io::{self, Write};

extern "C" {
    // This function for using CUDA internally via Fortran
    fn fortran_matrix_solver(n: i32, matrix_ptr: *const c_double, b_ptr: *const c_double, x_ptr: *mut c_double);
}

fn solve_matrix(matrix: Vec<Vec<f64>>, b: Vec<f64>) -> Vec<f64> {
    let n = matrix.len() as i32;
    let mut x = vec![0.0; n as usize];

    // Flatten thw matrix and vector b for Fortran FFI
    let matrix_flat: Vec<f64> = matrix.into_iter().flatten().collect();
    let b_flat: Vec<f64> = b.into_iter().collect();

    // Call Fortran solver, that use CUDA for matrix-vector multiplication
    unsafe {
        fortran_matrix_solver(n, matrix_flat.as_ptr(), b_flat.as_ptr(), x.as_mut_ptr());
    }

    x
}

fn main() {
    println!("Hybrid Matrix Solver with GPU Support");

    // Get matrix size 
    let mut size_input = String::new();
    print!("Enter the size of the matrix (n for an n x n matrix): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut size_input).unwrap();
    let n: usize = size_input.trim().parse().expect("Invalid size");

    // Get matrix entries
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut input = String::new();
            print!("Enter matrix element ({}, {}): ", i + 1, j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            matrix[i][j] = input.trim().parse().expect("Invalid input");
        }
    }

    // Get vector b
    let mut b = vec![0.0; n];
    for i in 0..n {
        let mut input = String::new();
        print!("Enter element b[{}]: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        b[i] = input.trim().parse().expect("Invalid input");
    }

    // Solve the system using the Fortran solver
    let result = solve_matrix(matrix, b);
    println!("Solution: x = {:?}", result);
}
