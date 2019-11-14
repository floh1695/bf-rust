use std::io;

use crate::command::Command;

pub struct Machine {
  pointer: usize,
  memory: Vec<i32>,
}

impl Machine {
  pub fn new() -> Machine {
    Machine {
      pointer: 0,
      memory: Vec::new(),
    }
  }

  pub fn interpret(&mut self, commands: &[Command]) {
    for command in commands {
      self.interpret_single(command);
    }
  }

  fn interpret_single(&mut self, command: &Command) {
    match command {
      Command::Empty() =>
        {},
      Command::Right(r) =>
        self.pointer += r,
      Command::Left(l) =>
        self.pointer -= l,
      Command::Up(u) => {
        self.ensure_memory();
        self.memory[self.pointer] += u;
      },
      Command::Down(d) => {
        let command = &Command::Up(-d);
        self.interpret_single(command);
      },
      Command::Out() => {
        println!("{}", self.current_value());
      },
      Command::In() => {
        let mut input = String::new();
        io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line");

        let input: i32 = input
          .trim()
          .parse()
          .expect("Please type a number");

        self.ensure_memory();
        self.memory[self.pointer] = input;
      },
      Command::Loop(cs) => {
        while self.memory[self.pointer] != 0 {
          self.interpret(&cs);
        }
      },
    }
  }

  fn current_value(&mut self) -> i32 {
    self.ensure_memory();
    self.memory[self.pointer]
  }

  fn ensure_memory(&mut self) {
    while self.memory.len() <= self.pointer {
      self.memory.push(0);
    }
  }
}
