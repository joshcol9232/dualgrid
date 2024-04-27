use crate::core::GridSpace;
use crate::core::cell::{Cell, num_vertices};

use std::io::Write;
use std::path::Path;
use std::fs::File;

const FACTORIAL: [usize; 16] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
    3628800,
    39916800,
    479001600,
    6227020800,
    87178291200,
    1307674368000
];

pub const fn factorial(n: usize) -> usize {
    FACTORIAL[n]
}

pub  fn random_offsets<const I: usize>() -> [f32; I] {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut out = [0.0; I];
    for o in out.iter_mut() {
        *o = rng.gen::<f32>();
    }
    out
}

pub fn write_to_file<P, const R: usize, const I: usize>(path: P, cells: &Vec<Cell<R, I>>) -> std::io::Result<()>
where
    P: AsRef<Path>,
    [(); num_vertices(R)]:
{
    let mut out_file = File::create(path)?;
    for cell in cells {
        let st = cell.numpy_format();
        out_file.write_all(st.as_bytes())?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

}

