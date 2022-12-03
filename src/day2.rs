#[path = "read_input.rs"]
mod read_input;

pub fn first_task() {
  let input = read_input::get_input("inputs/day2.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let mut score = 0;

  for &game in lines.iter() {
    let game_split: Vec<&str> = game.split(" ").collect();
    let [opponent_play, my_play] = [game_split[0], game_split[1]];
    match my_play {
      "X" => {
        score += 1;
        match opponent_play {
          "A" => score += 3,
          "B" => score += 0,
          "C" => score += 6,
          _ => todo!(),
        }
      }
      "Y" => {
        score += 2;
        match opponent_play {
          "A" => score += 6,
          "B" => score += 3,
          "C" => score += 0,
          _ => todo!(),
        }
      }
      "Z" => {
        score += 3;
        match opponent_play {
          "A" => score += 0,
          "B" => score += 6,
          "C" => score += 3,
          _ => todo!(),
        }
      }
      _ => todo!(),
    }
  }
  println!("{score}");
}

pub fn second_task() {
  let input = read_input::get_input("inputs/day2.txt");

  let lines: Vec<&str> = input.split("\n").collect();

  let mut score = 0;

  for &game in lines.iter() {
    let game_split: Vec<&str> = game.split(" ").collect();
    let [opponent_play, outcome] = [game_split[0], game_split[1]];

    let my_play = match outcome {
      "X" => match opponent_play {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => todo!(),
      },

      "Y" => match opponent_play {
        "A" => "A",
        "B" => "B",
        "C" => "C",
        _ => todo!(),
      },

      "Z" => match opponent_play {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => todo!(),
      },

      _ => todo!(),
    };
    match my_play {
      "A" => {
        score += 1;
        match opponent_play {
          "A" => score += 3,
          "B" => score += 0,
          "C" => score += 6,
          _ => todo!(),
        }
      }
      "B" => {
        score += 2;
        match opponent_play {
          "A" => score += 6,
          "B" => score += 3,
          "C" => score += 0,
          _ => todo!(),
        }
      }
      "C" => {
        score += 3;
        match opponent_play {
          "A" => score += 0,
          "B" => score += 6,
          "C" => score += 3,
          _ => todo!(),
        }
      }
      _ => todo!(),
    }
  }

  println!("{score}");
}
