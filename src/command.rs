pub enum Command {
  Empty(),
  Right(usize),
  Left(usize),
  Up(i32),
  Down(i32),
  Out(),
  In(),
  Loop(Vec<Command>),
  Block(Vec<Command>),
  Store(),
  Load(),
}
