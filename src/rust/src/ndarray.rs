use ndarray::array;
use ndarray::Array;
use ndarray::Dimension;
use ndarray::IntoDimension;
use ndarray::ShapeBuilder;
use savvy::savvy_err;
use savvy::OwnedRealSexp;
use savvy::{r_println, savvy, RealSexp};

/// @export
#[savvy]
fn ndarray_input(x: RealSexp) -> savvy::Result<()> {
    // In R, dim is i32, so you need to convert it to usize first.
    let dim_i32 = x.get_dim().ok_or(savvy_err!("no dimension found"))?;
    let dim: Vec<usize> = dim_i32.iter().map(|i| *i as usize).collect();

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

    let (vec, offset) = a_column_major.into_raw_vec_and_offset();
    let offset = offset.unwrap_or(0);
    let vec_view = &vec[offset..(offset + dim.0 * dim.1)];
    let mut out = OwnedRealSexp::try_from(vec_view)?;
    // It seems there's no handy way to convert an arbitrary dimension of tuple
    // to a slice without unsafe.
    match dim.into_dimension().as_array_view().as_slice() {
        Some(d) => out.set_dim(d)?,
        None => return Err(savvy_err!("Failed to get dimension as slice")),
    }

    out.into()
}
