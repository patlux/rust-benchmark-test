use grid;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let grid_cols = 3;
  let grid_rows = 3;
  let image_width = 1920;
  let image_height = 1080;

  let grid_size_width = image_width / grid_cols;
  let grid_size_height = image_height / grid_rows;

  let s = String::new();
  let cmd: &str = &args.get(1).unwrap_or(&s);
  match cmd {
    "--one" => grid::one_loop(&grid_cols, &grid_rows, &grid_size_width, &grid_size_height),
    "--two" => grid::two_loop(&grid_cols, &grid_rows, &grid_size_width, &grid_size_height),
    _ => println!("Please pass one of the followings arguments: --one or --two"),
  }
}
