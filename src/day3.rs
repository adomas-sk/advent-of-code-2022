use std::collections::HashMap;

#[path = "read_input.rs"]
mod read_input;

fn generate_letters() -> Vec<char> {
  let lower_case_chars: Vec<char> = (b'a'..=b'z').map(char::from).collect();
  let upper_case_chars: Vec<char> = (b'A'..=b'Z').map(char::from).collect();
  let letters = [lower_case_chars, upper_case_chars].concat();
  return letters;
}

pub fn first_task() {
  let input = read_input::get_input("inputs/day3.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let mut sum = 0;
  for &items in lines.iter() {
    let (first_compartment, second_compartment) =
      items.split_at(items.len() / 2);

    let mut fc_items: Vec<&str> = first_compartment.split("").collect();
    fc_items.pop();
    fc_items.swap_remove(0);

    let mut sc_items: Vec<&str> = second_compartment.split("").collect();
    sc_items.pop();
    sc_items.swap_remove(0);

    let mut item = "".to_string();
    'outer: for &fc_item in fc_items.iter() {
      for &sc_item in sc_items.iter() {
        if fc_item == sc_item {
          item = fc_item.to_string();
          break 'outer;
        }
      }
    }
    let index = generate_letters()
      .iter()
      .position(|letter| letter.to_string() == item)
      .unwrap();
    sum += index + 1;
  }
  println!("{sum}");
}

pub fn second_task() {
  let input = read_input::get_input("inputs/day3.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let groups: Vec<&[&str]> = lines.chunks(3).collect();

  let mut sum = 0;
  for group in groups.iter() {
    let mut items_in_bags: HashMap<&str, i32> = HashMap::new();

    for bag in group.iter() {
      let mut items: Vec<&str> = bag.split("").collect();
      items.pop();
      items.swap_remove(0);

      let mut seen_items: Vec<&str> = vec![];
      for item in items.iter() {
        if !seen_items.contains(item) {
          seen_items.push(item);
          let count = items_in_bags.entry(item).or_insert(0);
          *count += 1;
        }
      }
    }
    for (&key, &val) in items_in_bags.iter() {
      if val == 3 {
        let index = generate_letters()
          .iter()
          .position(|letter| letter.to_string() == key)
          .unwrap();
        sum += index + 1;
        break;
      }
    }
  }
  println!("{sum}");
}
