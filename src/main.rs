#![feature(iter_advance_by)]

mod parse;

use crate::parse::SymItem;

enum MacroInstruction {
  Expand,
  Eval,
  Copy,
  Frame,
  Return,
  Define,
}

struct MetaMachine {
  
}

fn main() {
  println!("{}", SymItem::parse("(a b c)").unwrap());
}
