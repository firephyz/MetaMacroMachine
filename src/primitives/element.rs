use super::minst::{MacroInstruction, MinstSymItemError};
use crate::parse::{SymItem, SymList, SymAtom};

use std::vec;
use std::fmt::{self, Display, Debug};
use std::ops::Deref;

#[derive(Debug)]
pub enum MetaElementError<'m> {
  ParseError,
  InvalidInstr(&'m SymItem),
  UnknownMinstError(MinstSymItemError<'m>),
}

impl<'m> From<MinstSymItemError<'m>> for MetaElementError<'m> {
  fn from(err : MinstSymItemError<'m>) -> Self {
    match err {
      MinstSymItemError::InvalidInstr(sym) => MetaElementError::InvalidInstr(sym),
      _ => MetaElementError::UnknownMinstError(err),
    }
  }
}

#[derive(Debug, Clone)]
pub enum MetaElement {
  Instr(MacroInstruction),
  Expr(SymItem),
}

impl MetaElement {
  pub fn parse(string : &str) -> Option<Self> {
    let expr = SymItem::parse(string).ok_or(MetaElementError::ParseError)
      .map(|val| { Some(val) })
      .unwrap_or_else(|err| { println!("SymItem parse failed {:?}", &err); None })?;

    MetaElement::try_from(&expr)
      .map(|val| { Some(val) })
      .unwrap_or_else(|err| { println!("MetaElement conversion failed: {:?}", &err); None })
  }

  pub fn as_str(&self) -> Option<&str> {
    if let MetaElement::Expr(symitem) = self {
      if let SymItem::SymAtom(atom) = symitem {
	Some(atom.as_str())
      }
      else { None }
    }
    else { None }
  }

  pub fn as_list<'a>(&'a self) -> Option<MetaElementListOperator<'a>> {
    if let MetaElement::Expr(expr) = self {
      if let SymItem::SymList(list) = expr {
	Some(MetaElementListOperator::new(&list))
      }
      else { None }
    }
    else { None }
  }
}

impl<'m> TryFrom<&'m SymItem> for MetaElement {
  type Error = MetaElementError<'m>;
  fn try_from(sym : &'m SymItem) -> Result<Self, Self::Error> {
    if sym.is_atom() {
      Ok(MetaElement::Expr(sym.clone()))
    }
    else {
      if sym.as_list().unwrap().len() >= 1 {
	let car = sym.index_early(0).unwrap();
	// Try making a machine instruction
	let try_inst = {
	  if car.is_atom() {
	    MacroInstruction::try_from(sym)
	  }
	  else {
	    Err(MinstSymItemError::NotAnInstruction(&sym))
	  }
	};

	match try_inst {
	  Ok(inst) => return Ok(MetaElement::Instr(inst)),
	  Err(ref err) => {
	    match err {
	      MinstSymItemError::NotAnInstruction(_) => (),
	      _ => { try_inst?; () },
	    }
	  }
	}

	// Machine instruction didn't work out, recurse on list
	for item in sym.as_list().unwrap().iter() {
	  item.update_to_late_stage(MetaElement::try_from(item.into_inner_early().unwrap())?);
        }
	Ok(MetaElement::Expr(sym.clone()))
      }
      else {
	Ok(MetaElement::Expr(sym.clone()))
      }
    }
  }
}
    
impl Display for MetaElement {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      MetaElement::Expr(sym) => fmt.write_str(format!("{}", &sym).as_str()),
      MetaElement::Instr(inst) => fmt.write_str(format!("{}", &inst).as_str()),
    }
  }
}

pub struct MetaElementListOperator<'a> {
  list: &'a SymList,
}

impl<'a> MetaElementListOperator<'a> {
  fn new(list: &'a SymList) -> Self {
    MetaElementListOperator {
      list: list,
    }
  }
}

impl<'a> IntoIterator for MetaElementListOperator<'a> {
  type Item = &'a MetaElement;
  type IntoIter = vec::IntoIter<Self::Item>;

  fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
    let x = self.list;
    x.iter().map(|list_item| {
      list_item.into_inner().unwrap()
    }).collect::<Vec<_>>().into_iter()
  }
}

impl<'a> Deref for MetaElementListOperator<'a> {
  type Target = SymList;

  fn deref(&self) -> &Self::Target {
    &self.list
  }
}

// impl<'a> Index<usize> for MetaElementListOperator<'a> {
//   type Output = MetaElement;
//   fn index(&self, idx: usize) -> &MetaElement {
//     self.list[idx].into_inner().expect("Encountered an early SymListItem when indexing a MetaElement.")
//   }
// }
