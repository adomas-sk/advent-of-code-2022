#[path = "read_input.rs"]
mod read_input;

fn parse_input() -> Vec<Vec<String>> {
  let input = read_input::get_input("inputs/day8.txt");
  let rows: Vec<&str> = input.split("\n").collect();
  let grid: Vec<Vec<String>> = rows
    .iter()
    .map(|row| {
      let mut columns: Vec<&str> = row.split("").collect();
      columns.pop();
      columns.remove(0);
      return columns.iter().map(|cell| cell.to_string()).collect();
    })
    .collect();
  return grid;
}

pub fn first_task() {
  let grid: Vec<Vec<String>> = parse_input();

  let mut visible_trees = 0;
  for (y, row) in grid.iter().enumerate() {
    for (x, tree) in row.iter().enumerate() {
      if x == 0 || y == 0 || y == grid.len() - 1 || x == row.len() - 1 {
        visible_trees += 1;
        continue;
      }
      let size = tree.parse::<usize>().unwrap();
      // TOP
      let has_top_cover = (0..y).find(|&top| {
        let neighbor_tree_size = grid[top][x].parse::<usize>().unwrap();
        return size <= neighbor_tree_size;
      });
      if has_top_cover.is_none() {
        visible_trees += 1;
        continue;
      }
      // BOTTOM
      let has_bottom_cover = (y + 1..grid.len()).find(|&bottom| {
        let neighbor_tree_size = grid[bottom][x].parse::<usize>().unwrap();
        return size <= neighbor_tree_size;
      });
      if has_bottom_cover.is_none() {
        visible_trees += 1;
        continue;
      }
      // LEFT
      let has_left_cover = (0..x).find(|&left| {
        let neighbor_tree_size = grid[y][left].parse::<usize>().unwrap();
        return size <= neighbor_tree_size;
      });
      if has_left_cover.is_none() {
        visible_trees += 1;
        continue;
      }
      // RIGHT
      let has_right_cover = (x + 1..row.len()).find(|&right| {
        let neighbor_tree_size = grid[y][right].parse::<usize>().unwrap();
        return size <= neighbor_tree_size;
      });
      if has_right_cover.is_none() {
        visible_trees += 1;
        continue;
      }
    }
  }
  println!("{visible_trees}");
}

pub fn second_task() {
  let grid: Vec<Vec<String>> = parse_input();

  let mut best_visibility = 0;
  for (y, row) in grid.iter().enumerate() {
    for (x, tree) in row.iter().enumerate() {
      let size = tree.parse::<usize>().unwrap();

      // TOP
      let mut visible_at_top = 0;
      for new_y in 0..y {
        let neighbor_tree_size = grid[(y - 1) - new_y][x].parse::<usize>().unwrap();
        visible_at_top += 1;
        if neighbor_tree_size >= size {
          break;
        }
      }
      // BOTTOM
      let mut visible_at_bottom = 0;
      for new_y in y + 1..grid.len() {
        let neighbor_tree_size = grid[new_y][x].parse::<usize>().unwrap();
        visible_at_bottom += 1;
        if neighbor_tree_size >= size {
          break;
        }
      }
      // LEFT
      let mut visible_at_left = 0;
      for new_x in 0..x {
        let neighbor_tree_size = grid[y][(x - 1) - new_x].parse::<usize>().unwrap();
        visible_at_left += 1;
        if neighbor_tree_size >= size {
          break;
        }
      }
      // RIGHT
      let mut visible_at_right = 0;
      for new_x in x + 1..row.len() {
        let neighbor_tree_size = grid[y][new_x].parse::<usize>().unwrap();
        visible_at_right += 1;
        if neighbor_tree_size >= size {
          break;
        }
      }
      let total_visibility =
        visible_at_top * visible_at_bottom * visible_at_left * visible_at_right;
      if best_visibility < total_visibility {
        best_visibility = total_visibility;
      }
    }
  }
  println!("{best_visibility}");
}
