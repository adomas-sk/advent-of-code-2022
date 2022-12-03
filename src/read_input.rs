use std::fs;

pub fn get_input(filename: &str) -> String {
  let contents = fs::read_to_string(filename)
    .expect("Should have been able to read the file");
  return contents;
}
