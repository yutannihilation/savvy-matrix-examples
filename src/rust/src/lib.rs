use savvy::{savvy, IntegerSexp};

/// @export
#[savvy]
fn nalgebra_input(x: IntegerSexp) -> savvy::Result<()> {
    Ok(())
}

/// @export
#[savvy]
fn nalgebra_output() -> savvy::Result<savvy::Sexp> {
    ().try_into()
}

/// @export
#[savvy]
fn ndarray_input(x: IntegerSexp) -> savvy::Result<()> {
    Ok(())
}

/// @export
#[savvy]
fn ndarray_output() -> savvy::Result<savvy::Sexp> {
    ().try_into()
}

/// @export
#[savvy]
fn glam_input(x: IntegerSexp) -> savvy::Result<()> {
    Ok(())
}

/// @export
#[savvy]
fn glam_output() -> savvy::Result<savvy::Sexp> {
    ().try_into()
}
