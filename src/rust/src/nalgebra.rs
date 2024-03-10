use nalgebra::DMatrix;
use savvy::{r_println, savvy, OwnedRealSexp, RealSexp};

/// @export
#[savvy]
fn nalgebra_input(x: RealSexp) -> savvy::Result<()> {
    let dim = x.get_dim().ok_or("no dimension found")?;

    if dim.len() != 2 {
        return Err("Input must be matrix!".into());
    }

    let m = DMatrix::from_vec(dim[0] as _, dim[1] as _, x.to_vec());

    r_println!("{m:?}");

    Ok(())
}

/// @export
#[savvy]
fn nalgebra_output() -> savvy::Result<savvy::Sexp> {
    let m = DMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let shape = m.shape();
    let dim = &[shape.0, shape.1];

    let mut out = OwnedRealSexp::try_from(m.as_slice())?;
    out.set_dim(dim)?;

    out.into()
}
