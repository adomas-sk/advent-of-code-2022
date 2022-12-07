#[path = "read_input.rs"]
mod read_input;

pub fn first_task() {
  let input = read_input::get_input("inputs/day6.txt");

  let mut lines: Vec<&str> = input.split("").collect();
  lines.pop();
  lines.remove(0);

  // This also worked on first task
  // for (index, &character) in lines.iter().enumerate() {
  //   if index < 3 {
  //     continue;
  //   }
  //   if lines[index - 1] == character
  //     || lines[index - 2] == character
  //     || lines[index - 3] == character
  //     || lines[index - 1] == lines[index - 2]
  //     || lines[index - 1] == lines[index - 3]
  //     || lines[index - 3] == lines[index - 2]
  //   {
  //     continue;
  //   }
  //   println!("{:?}", index + 1);
  //   break;
  // }

  let mut last_characters: Vec<&str> = vec![];
  for (index, character) in lines.iter().enumerate() {
    if index < 3 {
      last_characters.push(character);
      continue;
    }
    last_characters.push(character);
    let mut deduped_characters = last_characters.to_vec();
    deduped_characters.sort();
    deduped_characters.dedup();
    if deduped_characters.len() == 4 {
      println!("{:?}", index + 1);
      break;
    }
    last_characters.remove(0);
  }
}

pub fn second_task() {
  let input = read_input::get_input("inputs/day6.txt");

  let mut lines: Vec<&str> = input.split("").collect();
  lines.pop();
  lines.remove(0);

  let mut last_characters: Vec<&str> = vec![];
  for (index, character) in lines.iter().enumerate() {
    if index < 13 {
      last_characters.push(character);
      continue;
    }
    last_characters.push(character);
    let mut deduped_characters = last_characters.to_vec();
    deduped_characters.sort();
    deduped_characters.dedup();
    if deduped_characters.len() == 14 {
      println!("{:?}", index + 1);
      break;
    }
    last_characters.remove(0);
  }
}
