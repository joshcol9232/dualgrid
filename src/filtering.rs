use crate::core::cell::{num_vertices, Cell};

pub fn filter_by_max_index_rad<const R: usize, const I: usize>(
    cells: Vec<Cell<R, I>>,
    max_index_rad: usize,
) -> Vec<Cell<R, I>>
where 
    [(); num_vertices(R)]:
{
    cells.into_iter()
         .filter(|cell| cell.indices.cast::<f32>().norm_squared() < max_index_rad.pow(2) as f32)
         .collect()
}

