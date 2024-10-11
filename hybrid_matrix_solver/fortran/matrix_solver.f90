module matrix_solver
  use iso_c_binding, only: c_float, c_int
  implicit none

  interface
    subroutine matrix_vector_mult_cuda(A, x, b, n) bind(C, name="matrix_vector_mult_cuda")
      use iso_c_binding
      real(c_float), dimension(:), intent(in) :: A
      real(c_float), dimension(:), intent(in) :: x
      real(c_float), dimension(:), intent(out) :: b
      integer(c_int), value :: n
    end subroutine matrix_vector_mult_cuda
  end interface

contains

  subroutine conjugate_gradient(A, b, x, n, tol, max_iter)
    implicit none
    integer, intent(in) :: n, max_iter
    real(c_float), intent(in) :: tol
    real(c_float), dimension(n, n), intent(in) :: A
    real(c_float), dimension(n), intent(in) :: b
    real(c_float), dimension(n), intent(out) :: x

    ! Local variables
    real(c_float) :: alpha, beta, rsold, rsnew
    real(c_float), dimension(n) :: r, p, Ap
    integer :: i, iter

    ! Initialize
    x = 0.0
    call matrix_vector_mult_cuda(A, x, Ap, n)
    r = b - Ap
    p = r
    rsold = dot_product(r, r)

    ! Iterative CG method
    do iter = 1, max_iter
      call matrix_vector_mult_cuda(A, p, Ap, n)
      alpha = rsold / dot_product(p, Ap)
      x = x + alpha * p
      r = r - alpha * Ap
      rsnew = dot_product(r, r)
      if (sqrt(rsnew) < tol) exit
      beta = rsnew / rsold
      p = r + beta * p
      rsold = rsnew
    end do

  end subroutine conjugate_gradient
end module matrix_solver

