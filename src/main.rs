#![feature(iter_advance_by)]
#![feature(iterator_try_collect)]
#![feature(never_type)]

mod parse;
mod primitives;

use crate::primitives::MetaElement;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::slice::Iter;

#[derive(Debug)]
enum RuntimeError<'a> {
  CompoundListAsMacroError(&'a MetaElement),
  UnknownDef(&'a MetaElement),
}

type MetaDef = (MetaElement, MetaElement, MetaElement);
type StackFrame = RefCell<Vec<MetaElement>>;

struct MetaMachine {
  stack: Vec<Rc<StackFrame>>,
  frame: Weak<StackFrame>,
  code: Vec<MetaElement>,
  defs: Vec<MetaDef>,
}

impl MetaMachine {
  fn new() -> Self {
    let init = MetaElement::parse("
      (start (macro form body)
        (.INDEX 0 0)
        (.INDEX 0 1)
        (.CONTEXT)
        (.RETURN 0)
        (.INDEX 0 1)
        (.CONTEXT)
        (.RETURN 1 -1)
        (.DEFINE))")
      .unwrap();

    let stack = vec![Rc::new(RefCell::new(vec![init]))];
    let initframe = Rc::downgrade(&stack[0]);
    let initial_def = (MetaElement::parse("start").unwrap(),
		       MetaElement::parse("(def-form def-body)").unwrap(),
		       MetaElement::parse("((.DEFINE))").unwrap());

    MetaMachine {
      stack : stack,
      frame: initframe,
      code: vec![],
      defs : vec![initial_def],
    }
  }

  fn run(&mut self) -> Result<(), RuntimeError> {
    // let exec = &self.frame.upgrade().unwrap()
    //   .borrow().iter().rev().nth(0).unwrap();
    // self.code = exec;

    // while self.code.len() != 0
    // match exec {
    //   MetaElement::Expr(_) => {
    // 	// let macro_symbol = exec.as_list().unwrap().into_iter().nth(0).unwrap();
    // 	// let macro_name = macro_symbol.as_str()
    // 	//   .ok_or(RuntimeError::CompoundListAsMacroError(&macro_symbol))?;
    // 	// let macro_def = self.get_def(macro_symbol)
    // 	//   .ok_or(RuntimeError::UnknownDef(&macro_symbol))?;

    // 	// let nargs = macro_def.1.as_list().unwrap().len();
    // 	// let (rem_stack, args) = {
    // 	//   let rev_stack = self.frame.upgrade().unwrap()
    // 	//     .borrow().iter().rev().skip(1);
    // 	//   let args = rev_stack.take(nargs).collect::<Vec<_>>();
    // 	//   let rem_stack = rev_stack.rev().collect::<Vec<_>>();
    // 	//   (rem_stack, args)
    // 	// };
    // 	// self.frame.upgrade().unwrap().replace(rem_stack);
    // 	()
    //   },
    //   MetaElement::Instr(instr) => {
    // 	()
    //   },
    // };

    // println!("{}", exec);
    Ok(())
  }

  fn get_defs(&self) -> &Vec<MetaDef> {
    &self.defs
  }

  // fn get_def(&self, sym: &MetaElement) -> Option<MetaDef> {
  //   self.defs.iter().find(|def| {
  //     def.0 == sym
  //   })
  // }

  fn print_def(def: &MetaDef) {
    let def_string = format!("{}", def.2);
    println!("{} {}\n{}", def.0, def.1, def_string);
  }
}

fn main() {
  let mut meta = MetaMachine::new();
  meta.run().expect("Runtime error");
  // for def in meta.get_defs() {
  //   MetaMachine::print_def(def);
  // }
  // for scope in meta.stack {
  //   for arg in scope {
  //     println!("{}", arg.as_list().unwrap().len());
  //     println!("{}", &arg);
  //   }
  // }
}
