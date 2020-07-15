#[inline]
pub fn one_loop(grid_cols: &i32, grid_rows: &i32, grid_size_width: &i32, grid_size_height: &i32) {
  let grid_count = grid_cols * grid_rows;
  for grid_index in 0..grid_count {
    let grid_row = grid_index / grid_rows;
    let grid_col = grid_index % grid_cols;
    let _x = grid_col * grid_size_width;
    let _y = grid_row * grid_size_height;
  }
}

#[inline]
pub fn two_loop(grid_cols: &i32, grid_rows: &i32, grid_size_width: &i32, grid_size_height: &i32) {
  for grid_row in 0..*grid_rows {
    let _y = grid_row * grid_size_height;
    for grid_col in 0..*grid_cols {
      let _x = grid_col * grid_size_width;
    }
  }
}
