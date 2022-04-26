use regex::Regex;
use std::convert::TryFrom;
use std::str::Chars;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SymParseError {
  SymItemExtraInput(String),
  SymItemEOF(String),
  SymListEOF(String),
  SymAtomNoTerminal(String),
  SymAtomEOF(String),
  InvalidStartOfInput,
}

#[derive(Debug)]
pub enum SymItem {
  SymList(SymList),
  SymAtom(SymAtom),
}

impl SymItem {
  pub const Nil : SymItem = SymItem::SymList(SymList { items: vec![] } );
  
  pub fn parse(string: &str) -> Option<Self> {
    let mut chars = string.trim().chars();
    let result = Self::try_from(chars.by_ref());
    let end_of_input_check = if chars.as_str().len() != 0 {
      Err(SymParseError::SymItemExtraInput("Extra input after SymItem.".to_string()))
    }
    else {
      Ok(())
    };

    match result.and_then(|ok| end_of_input_check.and(Ok(ok))) {
      Ok(x) => Some(x),
      Err(e) => {
	println!("Parse failed: {:?}", e);
	None
      },
    }
  }

  pub fn is_atom(&self) -> bool {
    match self {
      Self::SymAtom(_) => true,
      _ => false,
    }
  }

  pub fn is_list(&self) -> bool {
    match self {
      Self::SymList(_) => true,
      _ => false,
    }
  }

  pub fn as_str(&self) -> Option<&str> {
    if let Self::SymAtom(atom) = self {
      Some(atom.symbol.as_str())
    }
    else {
      None
    }
  }

  pub fn as_list(&self) -> Option<&Vec<SymItem>> {
    if let Self::SymList(list) = self {
      Some(&list.items)
    }
    else {
      None
    }
  }
}

impl Display for SymItem {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      SymItem::SymList(data) => fmt.write_str(format!("{}", &data).as_str()),
      SymItem::SymAtom(data) => fmt.write_str(format!("{}", &data).as_str()),
    }
  }
}

impl<'chars> TryFrom<&mut Chars<'chars>> for SymItem {
  type Error = SymParseError;

  fn try_from(chars : &mut Chars<'chars>) -> Result<Self, Self::Error> {
    let first_char = chars.clone()
      .peekable().nth(0)
      .ok_or(SymParseError::SymItemEOF("Empty input when building SymItem.".to_string()))?;
    let result = {
      if first_char == '('  {
	let end_of_list_error = Err(SymParseError::SymListEOF("Unexpected EOF after start of SymList.".to_string()));
	Ok(SymItem::SymList(SymList::new(chars.advance_by(1).or(end_of_list_error).and(Ok(chars.by_ref()))?)?))
      }
      else if first_char == ')' {
	Err(SymParseError::InvalidStartOfInput)
      }
      else {
	Ok(SymItem::SymAtom(SymAtom::new(chars.by_ref())?))
      }
    };

    // if chars.as_str().len() != 0 {
    //   Err(SymParseError::SymItemExtraInput("Extra input after SymItem.".to_string()))?;
    // }

    // Ok(result)
    result
  }
}

#[derive(Debug)]
struct SymList {
  items : Vec<SymItem>,
}

impl SymList {
  fn new(chars : &mut Chars) -> Result<Self, SymParseError> {
    let list_eof_error = SymParseError::SymListEOF("EOF when building SymList.".to_string());
    let mut list_items = Vec::new();
    let mut chars_str = chars.as_str();

    while chars.as_str().get(0..1).ok_or(list_eof_error.clone())? != ")" {
      list_items.push(SymItem::try_from(chars.by_ref())?);

      let next_elem_start = chars.as_str().find(|c| { c != ' ' }).ok_or(list_eof_error.clone())?;
      if next_elem_start > 0 {
	chars.nth(next_elem_start - 1).unwrap();
      }
    }
    chars.next();

    Ok(SymList {
      items : list_items,
    })
  }
}

impl Display for SymList {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    if self.items.len() == 0 {
      fmt.write_str("()");
    }
    else {
      let inner_fmts = self.items.iter().map(|x| { format!("{}", &x) }).collect::<Vec<String>>();
      let inner_string = format!("{}{}",
				 inner_fmts[0],
				 inner_fmts[1..].iter().fold("".to_string(), |acc, x| { format!("{} {}", acc, x) }));
      fmt.write_str(format!("({})", inner_string).as_str());
    }

    Ok(())
  }
}

#[derive(Debug)]
struct SymAtom {
  symbol : String,
}

impl SymAtom {
  fn new(chars : &mut Chars) -> Result<Self, SymParseError> {
    let sym_end = chars.as_str().find(|c: char| { c == '(' || c == ')' || c == ' ' }).unwrap_or(chars.as_str().len());
    Ok(SymAtom {
      symbol : chars.take(sym_end).collect::<String>(),
    })
  }
}

impl Display for SymAtom {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fmt.write_str(&self.symbol);
    Ok(())
  }
}
