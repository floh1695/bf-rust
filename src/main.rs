mod command;
mod machine;

use command::Command;
use machine::Machine;

fn main() {
  let program = program_selector("triangle");
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
    _ => vec![],
  }
}
