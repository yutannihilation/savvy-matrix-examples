use glam::{dmat3, dvec3, DMat3};
use savvy::{r_println, savvy, OwnedRealSexp, RealSexp};

/// @export
#[savvy]
fn glam_input(x: RealSexp) -> savvy::Result<()> {
    let dim = match x.get_dim() {
        Some(dim) => dim,
        None => {
            return Err("no dimension found!".into());
        }
    };

    if dim.as_slice() != [3, 3] {
        return Err("Input must be 3x3 matrix!".into());
    }

    // As we already check the dimension, this must not fail
    let x_array: &[f64; 9] = x.as_slice().try_into().unwrap();

    let m = DMat3::from_cols_array(x_array);

    r_println!("{m:?}");

    Ok(())
}

/// @export
#[savvy]
fn glam_output() -> savvy::Result<savvy::Sexp> {
    let m = dmat3(
        dvec3(1.0, 2.0, 3.0),
        dvec3(4.0, 5.0, 6.0),
        dvec3(7.0, 8.0, 9.0),
    );

    let mut out = OwnedRealSexp::try_from(m.to_cols_array().as_slice())?;
    out.set_dim(&[3, 3])?;

    out.into()
}
