mod command;
mod machine;

use command::Command;
use machine::Machine;

fn main() {
  let program = program_selector("storage");
  let mut machine = Machine::new();
  machine.interpret(&program);
}

fn program_selector(code: &str) -> Vec<Command> {
  match code {
    "countup" => vec![
      Command::In(),
      Command::Loop(vec![
        Command::Down(1),
        Command::Right(1),
        Command::Up(1),
        Command::Out(),
        Command::Left(1),
      ]),
    ],
    "countdown" => vec![
      Command::Empty(),
      Command::In(),
      Command::Loop(vec![
        Command::Out(),
        Command::Down(1),
      ]),
    ],
    "triangle" => vec![
      Command::In(),
      Command::Loop(vec![
        Command::Loop(vec![
          Command::Down(1),
          Command::Right(1),
          Command::Up(1),
          Command::Right(1),
          Command::Up(1),
          Command::Left(2)
        ]),
        Command::Right(1),
        Command::Down(1),
        Command::Loop(vec![
          Command::Down(1),
          Command::Left(1),
          Command::Up(1),
          Command::Right(1),
        ]),
        Command::Right(1),
        Command::Loop(vec![
          Command::Down(1),
          Command::Right(1),
          Command::Up(1),
          Command::Left(1),
        ]),
        Command::Left(2),
      ]),
      Command::Right(3),
      Command::Out(),
    ],
    "storage" => vec![ // 2(A + B) - C // Unfinished
      Command::In(), // define A #0
      Command::Right(1),
      Command::In(), // define B #1
      Command::Right(2),
      Command::In(), // define C #4
      Command::Left(3),
      add_block(0, 1, 2),
      Command::Right(2),
      Command::Out(),
    ],
    _ => vec![],
  }
}

fn add_block(a: usize, b: usize, c: usize) -> Command {
  let b_c_delta = c - b;

  Command::Block(vec![
    Command::Right(a),
    Command::Store(),
    Command::Left(a),
    Command::Right(c),
    Command::Load(),
    Command::Left(c),
    Command::Right(b),
    Command::Store(),
    Command::Loop(vec![
      Command::Down(1),
      Command::Right(b_c_delta),
      Command::Up(1),
      Command::Left(b_c_delta),
    ]),
    Command::Load(),
    Command::Left(b)
  ])
}
