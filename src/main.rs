use std::thread;
use std::io::{stdout, Write};
use std::time::Duration;
use chrono::Local;
use termion::{terminal_size, clear, cursor, color};

const FULL: [bool; 3] = [true, true, true];
const LEFT: [bool; 3] = [true, false, false];
const CENTER: [bool; 3] = [false, true, false];
const RIGHT: [bool; 3] = [false, false, true];
const OUTER: [bool; 3] = [true, false, true];
const NONE: [bool; 3] = [false, false, false];

fn main() {
  let color = color::Blue;
  print!("{}{}", cursor::Hide, color::Fg(color));

  loop {
    let size = terminal_size().unwrap();
    let (x, y) = ((size.0 - 64) / 2, (size.1 + 5) / 2);
    let time = Local::now();
    print!("{}{}", clear::All, cursor::Goto(x, y - 5));
    for (i, c) in format!("{}", time.format("%H:%M:%S")).chars().enumerate() {
      for (col, blocks) in match c {
        '0' => [FULL, OUTER, OUTER, OUTER, FULL],
        '1' => [CENTER, CENTER, CENTER, CENTER, CENTER],
        '2' => [FULL, RIGHT, FULL, LEFT, FULL],
        '3' => [FULL, RIGHT, FULL, RIGHT, FULL],
        '4' => [OUTER, OUTER, FULL, RIGHT, RIGHT],
        '5' => [FULL, LEFT, FULL, RIGHT, FULL],
        '6' => [FULL, LEFT, FULL, OUTER, FULL],
        '7' => [FULL, RIGHT, RIGHT, RIGHT, RIGHT],
        '8' => [FULL, OUTER, FULL, OUTER, FULL],
        '9' => [FULL, OUTER, FULL, RIGHT, RIGHT],
        ':' => [NONE, CENTER, NONE, CENTER, NONE],
        _ => [NONE, NONE, NONE, NONE, NONE],
      }
      .iter()
      .enumerate()
      {
        print!("{}", cursor::Goto(x + 8 * i as u16, y - 5 + col as u16));
        for block in blocks {
          if *block {
            print!("{}  ", color::Bg(color));
          } else {
            print!("{}  ", color::Bg(color::Reset));
          }
        }
      }
    }
    print!(
      "{}{}{}",
      color::Bg(color::Reset),
      cursor::Goto(x, y + 1),
      time.format("%d %B %Y")
    );
    stdout().flush();
    thread::sleep(Duration::from_secs(1));
  }
}
