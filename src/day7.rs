#[path = "read_input.rs"]
mod read_input;

#[derive(Debug)]
struct Directory {
  name: String,
  files: Vec<[String; 2]>,
  internal_directories: Vec<Directory>,
}

impl Directory {
  fn get_size(&self) -> usize {
    let file_size = self.get_files_size_sum();
    let directories_size = self.get_directories_size();
    return file_size + directories_size;
  }

  fn get_files_size_sum(&self) -> usize {
    let mut total_size = 0;
    for files in self.files.iter() {
      total_size += files[0].parse::<usize>().unwrap();
    }
    return total_size
  }

  fn get_directories_size(&self) -> usize {
    let mut total_size = 0;
    for directory in self.internal_directories.iter() {
      total_size += directory.get_size();
    }
    return total_size;
  }
}

fn parse_input() -> Directory {
  let input = read_input::get_input("inputs/day7.txt");

  let lines: Vec<&str> = input.split("\n").collect();
  let mut root: Directory = Directory {
    name: "/".to_string(),
    files: vec![],
    internal_directories: vec![],
  };
  let mut current_directory: &mut Directory = &mut root;
  let mut current_path: Vec<&str> = vec![];
  for &item in lines.iter() {
    let mut item_chars = item.chars();
    if item_chars.nth(0).unwrap() == '$' {
      let commands: Vec<&str> = item.split(" ").collect();
      let command = commands[1];
      if command == "cd" {
        let arg = commands[2];
        if arg == "/" {
          continue;
        }
        if arg == ".." {
          current_path.pop();
          current_directory = &mut root;
          for &path in current_path.iter() {
            let next_dir = current_directory
              .internal_directories
              .iter_mut()
              .find(|dir| dir.name == path);
            if next_dir.is_none() {
              panic!("No such directory - {path}");
            }
            current_directory = next_dir.unwrap();
          }
          continue;
        }
        current_path.push(arg);
        let next_dir = current_directory
          .internal_directories
          .iter_mut()
          .find(|dir| dir.name == arg);
        if next_dir.is_none() {
          panic!("No such directory - {arg}");
        }
        current_directory = next_dir.unwrap();
      }
    } else {
      let location: Vec<&str> = item.split(" ").collect();
      if location[0] == "dir" {
        let new_dir = Directory {
          name: location[1].to_string(),
          files: vec![],
          internal_directories: vec![],
        };
        current_directory.internal_directories.push(new_dir);
      } else {
        current_directory
          .files
          .push([location[0].to_string(), location[1].to_string()]);
      }
    }
  }

  return root;
}

pub fn first_task() {
  let root = parse_input();

  let mut directories: Vec<&Directory> = vec![&root];
  let mut total_sum = 0;

  while directories.len() > 0 {
    let current_dir = directories[0];
    let current_dir_size = current_dir.get_size();
    if current_dir_size < 100_000 {
      total_sum += current_dir_size;
    }
    for dir in current_dir.internal_directories.iter() {
      directories.push(dir);
    }
    directories.remove(0);
  }

  println!("{total_sum}");
}

const TOTAL_SIZE: usize = 70_000_000;
const SIZE_NEEDED: usize = 30_000_000;

pub fn second_task() {
  let root = parse_input();

  let root_size = root.get_size();
  let size_to_delete = SIZE_NEEDED - (TOTAL_SIZE - root_size);

  let mut directories: Vec<&Directory> = vec![&root];

  let mut best_difference = TOTAL_SIZE + 1;
  while directories.len() > 0 {
    let current_dir = directories[0];
    let current_dir_size = current_dir.get_size();
    if current_dir_size >= size_to_delete && current_dir_size < best_difference {
      best_difference = current_dir_size
    }

    for dir in current_dir.internal_directories.iter() {
      directories.push(dir);
    }
    directories.remove(0);
  }

  println!("{best_difference}");
}
