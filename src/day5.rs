#[path = "read_input.rs"]
mod read_input;

fn parse_input() -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
  let input = read_input::get_input("inputs/day5.txt");

  let parts: Vec<&str> = input.split("\n\n").collect();
  let initial_state = parts[0];
  let instructions = parts[1];

  // Parse initial state
  let rows: Vec<&str> = initial_state.split("\n").collect();
  let stacks = (rows[0].len() + 1) / 4;

  let mut columns: Vec<Vec<char>> = vec![];
  for _ in 0..stacks {
    columns.push(vec![]);
  }
  for (index, &item) in rows.iter().enumerate() {
    if index == rows.len() - 1 {
      break;
    }
    let item_string = item.to_string();
    for index in 0..stacks {
      let container = item_string.chars().nth(index * 4 + 1).unwrap();
      if container != ' ' {
        columns[index].push(container);
      }
    }
  }

  for column in columns.iter_mut() {
    column.reverse();
  }

  // Parse instructions
  let instruction_list: Vec<&str> = instructions.split("\n").collect();
  let parsed_instructions: Vec<[usize; 3]> = instruction_list
    .iter()
    .map(|instruction_row| {
      let instruction_parts: Vec<&str> = instruction_row.split(" ").collect();
      let count = instruction_parts[1].parse::<usize>().unwrap();
      let from = instruction_parts[3].parse::<usize>().unwrap();
      let to = instruction_parts[5].parse::<usize>().unwrap();
      return [count, from, to];
    })
    .collect();
  return (columns, parsed_instructions);
}

pub fn first_task() {
  let (mut columns, parsed_instructions) = parse_input();

  parsed_instructions.iter().for_each(|instruction| {
    let count = instruction[0];
    let from = instruction[1];
    let to = instruction[2];
    for _ in 0..count {
      let temp = columns[from - 1].pop().unwrap();
      columns[to - 1].push(temp);
    }
  });

  columns.iter().for_each(|stack| {
    let last_char = stack[stack.len() - 1];
    print!("{last_char}");
  });
  println!("");
}

pub fn second_task() {
  let (mut columns, parsed_instructions) = parse_input();

  parsed_instructions.iter().for_each(|instruction| {
    let count = instruction[0];
    let from = instruction[1];
    let to = instruction[2];
    let mut temp: Vec<char> = vec![];
    for _ in 0..count {
      let single_temp = columns[from - 1].pop().unwrap();
      temp.push(single_temp);
    }
    temp.reverse();
    columns[to - 1].append(&mut temp);
  });

  columns.iter().for_each(|stack| {
    let last_char = stack[stack.len() - 1];
    print!("{last_char}");
  });
  println!("");
}
