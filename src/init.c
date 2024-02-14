
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP nalgebra_input__impl(SEXP x) {
    SEXP res = nalgebra_input(x);
    return handle_result(res);
}

SEXP nalgebra_output__impl(void) {
    SEXP res = nalgebra_output();
    return handle_result(res);
}

SEXP ndarray_input__impl(SEXP x) {
    SEXP res = ndarray_input(x);
    return handle_result(res);
}

SEXP ndarray_output__impl(void) {
    SEXP res = ndarray_output();
    return handle_result(res);
}

SEXP glam_input__impl(SEXP x) {
    SEXP res = glam_input(x);
    return handle_result(res);
}

SEXP glam_output__impl(void) {
    SEXP res = glam_output();
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"nalgebra_input__impl", (DL_FUNC) &nalgebra_input__impl, 1},
    {"nalgebra_output__impl", (DL_FUNC) &nalgebra_output__impl, 0},
    {"ndarray_input__impl", (DL_FUNC) &ndarray_input__impl, 1},
    {"ndarray_output__impl", (DL_FUNC) &ndarray_output__impl, 0},
    {"glam_input__impl", (DL_FUNC) &glam_input__impl, 1},
    {"glam_output__impl", (DL_FUNC) &glam_output__impl, 0},
    {NULL, NULL, 0}
};

void R_init_savvyMatrixExamples(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
