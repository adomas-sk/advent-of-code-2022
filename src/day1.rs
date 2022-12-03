#[path = "read_input.rs"]
mod read_input;

pub fn first_task() {
  let input = read_input::get_input("inputs/day1.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let mut elfs: Vec<i32> = vec![0];
  for (_i, &item) in lines.iter().enumerate() {
    if item == "" {
      elfs.push(0);
    } else {
      let last_elf = elfs.last_mut().unwrap();
      *last_elf = *last_elf + &item.parse::<i32>().unwrap();
    }
  }
  let elf_with_max_callories = elfs.iter().max().unwrap();
  println!("{elf_with_max_callories}");
}

pub fn second_task() {
  let input = read_input::get_input("inputs/day1.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let mut elfs: Vec<i32> = vec![0];
  for (_i, &item) in lines.iter().enumerate() {
    if item == "" {
      elfs.push(0);
    } else {
      let last_elf = elfs.last_mut().unwrap();
      *last_elf = *last_elf + &item.parse::<i32>().unwrap();
    }
  }
  let chongus_one = elfs.iter().max().unwrap().clone();
  let chongus_one_index = elfs.iter().position(|x| *x == chongus_one).unwrap();
  elfs.swap_remove(chongus_one_index);

  let chongus_two = elfs.iter().max().unwrap().clone();
  let chongus_two_index = elfs.iter().position(|x| *x == chongus_two).unwrap();
  elfs.swap_remove(chongus_two_index);

  let chongus_three = elfs.iter().max().unwrap().clone();
  let chongus_three_index =
    elfs.iter().position(|x| *x == chongus_three).unwrap();
  elfs.swap_remove(chongus_three_index);

  let top_three_elfs_callories = chongus_one + chongus_two + chongus_three;
  println!("{top_three_elfs_callories}");
}
