#[path = "read_input.rs"]
mod read_input;

pub fn first_task() {
  let input = read_input::get_input("inputs/day4.txt");

  let pairs: Vec<&str> = input.split("\n").collect();

  let mut score = 0;

  for &pair in pairs.iter() {
    let elfs: Vec<&str> = pair.split(",").collect();
    let first_elf_assigment: Vec<&str> = elfs[0].split("-").collect();
    let second_elf_assigment: Vec<&str> = elfs[1].split("-").collect();

    let fe_start = first_elf_assigment[0].parse::<i32>().unwrap();
    let fe_end = first_elf_assigment[1].parse::<i32>().unwrap();
    let se_start = second_elf_assigment[0].parse::<i32>().unwrap();
    let se_end = second_elf_assigment[1].parse::<i32>().unwrap();

    if (se_start >= fe_start && se_end <= fe_end)
      || (se_start <= fe_start && se_end >= fe_end)
    {
      score += 1;
    }
  }
  println!("{score}");
}

pub fn second_task() {
  let input = read_input::get_input("inputs/day4.txt");

  let pairs: Vec<&str> = input.split("\n").collect();

  let mut score = 0;

  for &pair in pairs.iter() {
    let elfs: Vec<&str> = pair.split(",").collect();
    let first_elf_assigment: Vec<&str> = elfs[0].split("-").collect();
    let second_elf_assigment: Vec<&str> = elfs[1].split("-").collect();

    let fe_start = first_elf_assigment[0].parse::<i32>().unwrap();
    let fe_end = first_elf_assigment[1].parse::<i32>().unwrap();
    let se_start = second_elf_assigment[0].parse::<i32>().unwrap();
    let se_end = second_elf_assigment[1].parse::<i32>().unwrap();

    'outer: for number1 in fe_start..(fe_end + 1) {
      for number2 in se_start..(se_end + 1) {
        if (number1 == number2) {
          score += 1;
          break 'outer;
        }
      }
    }
  }
  println!("{score}");
}
