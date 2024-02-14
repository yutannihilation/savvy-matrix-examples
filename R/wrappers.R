#' @useDynLib savvyMatrixExamples, .registration = TRUE
#' @keywords internal
NULL

#' @export
nalgebra_input <- function(x) {
  invisible(.Call(nalgebra_input__impl, x))
}

#' @export
nalgebra_output <- function() {
  .Call(nalgebra_output__impl)
}

#' @export
ndarray_input <- function(x) {
  invisible(.Call(ndarray_input__impl, x))
}

#' @export
ndarray_output <- function() {
  .Call(ndarray_output__impl)
}

#' @export
glam_input <- function(x) {
  invisible(.Call(glam_input__impl, x))
}

#' @export
glam_output <- function() {
  .Call(glam_output__impl)
}


