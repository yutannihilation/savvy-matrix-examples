use ndarray::array;
use ndarray::Array;
use ndarray::Dimension;
use ndarray::IntoDimension;
use ndarray::ShapeBuilder;
use savvy::OwnedRealSexp;
use savvy::{r_println, savvy, RealSexp};

/// @export
#[savvy]
fn ndarray_input(x: RealSexp) -> savvy::Result<()> {
    let dim = match x.get_dim() {
        Some(dim) => dim,
        None => {
            return Err("no dimension found!".into());
        }
    };

    // f() changes the order from row-major (C-style convention) to column-major (Fortran-style convention).
    let a = Array::from_shape_vec(dim.f(), x.to_vec());

    r_println!("{a:?}");

    Ok(())
}

/// @export
#[savvy]
fn ndarray_output() -> savvy::Result<savvy::Sexp> {
    // By default, memory layout is row major.
    let a_row_major = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let a_column_major = a_row_major.reversed_axes();
    let dim = a_column_major.dim();

    let mut out = OwnedRealSexp::try_from(a_column_major.into_raw_vec())?;
    // It seems there's no handy way to convert an arbitrary dimension of tuple
    // to a slice without unsafe.
    match dim.into_dimension().as_array_view().as_slice() {
        Some(d) => out.set_dim(d)?,
        None => return Err("Failed to get dimension as slice".into()),
    }

    out.into()
}
