use std::convert::TryFrom;
use std::str::Chars;
use std::fmt::{self, Display};
use std::ops::Deref;

use crate::primitives::MetaElement;

#[derive(Debug, Clone)]
pub enum SymParseError {
  SymItemExtraInput(String),
  SymItemEOF(String),
  SymListEOF(String),
  SymAtomNoTerminal(String),
  SymAtomEOF(String),
  InvalidStartOfInput,
}

#[derive(Debug, Clone)]
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
      Some(atom.as_str())
    }
    else {
      None
    }
  }

  // pub fn as_early_list(&self) -> Option<&SymList> {
  //   if let Self::SymList(list) = self {
  //     // let early_ptr : *const Vec<_> = &list.items;
  //     // Some(unsafe {
  //     // 	NonNull::new(early_ptr as *mut Vec<SymItem>).unwrap().as_ref()
  //     // })
  //     Some(&list)
  //   }
  //   else {
  //     None
  //   }
  // }

  pub fn as_list(&self) -> Option<&SymList> {
    if let Self::SymList(list) = self {
      Some(&list)
    }
    else {
      None
    }
  }

  pub fn index_early(&self, idx: usize) -> Option<&SymItem> {
    let list = self.as_list()?;
    list[idx].into_inner_early()
  }

  pub fn index(&self, idx: usize) -> Option<&MetaElement> {
    let list = self.as_list()?;
    list[idx].into_inner()
  }
}

impl Display for SymItem {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      SymItem::SymAtom(data) => {
	fmt.write_str(data.as_str())
      },
      SymItem::SymList(data) => {
	if data.len() == 0 {
	  fmt.write_str("()")
	}
	else {
	  let inner_fmts = data.iter().map(|x| { format!("{}", &x) }).collect::<Vec<String>>();
	  let inner_string = format!("{}{}",
				     inner_fmts[0],
				     inner_fmts[1..].iter().fold("".to_string(), |acc, x| { format!("{} {}", acc, x) }));
	  fmt.write_str(format!("({})", inner_string).as_str())
	}
      },
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

    result
  }
}

// SymList container type changes depending on the stage of parsing. Early on, they
// may only hold other SymItems. After parsing at the MetaElement level, they change to
// supporting MetaElements
// union SymListItem {
//   pre : ManuallyDrop<SymItem>,
//   full : ManuallyDrop<MetaElement>,
// }
#[derive(Debug, Clone)]
pub enum SymListItem {
  Early(SymItem),
  Full(MetaElement),
}

impl SymListItem {
  pub fn into_inner_early(&self) -> Option<&SymItem> {
    if let SymListItem::Early(x) = self { Some(x) }
    else { None }
  }

  pub fn into_inner(&self) -> Option<&MetaElement> {
    if let SymListItem::Full(x) = self { Some(x) }
    else { None }
  }

  pub fn update_to_late_stage(&self, contents: MetaElement) {
    let item_ptr : *const SymListItem = self;
    let item_mut_ptr = item_ptr as *mut SymListItem;
    unsafe {
      *item_mut_ptr = SymListItem::Full(contents);
    }
  }
}

// impl SymListItem {
//   pub fn as_early_phase(&self) -> Option<&SymItem> {
//     if let SymListItem::Early(x) = self { Some(&x) }
//     else { None }
//   }

//   pub fn as_late_phase(&self) -> Option<&MetaElement> {
//     if let SymListItem::Full(x) = self { Some(&x) }
//     else { None }
//   }
// }

impl Display for SymListItem {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      SymListItem::Early(symitem) => fmt.write_str(format!("{}", &symitem).as_str())?,
      SymListItem::Full(meta) => fmt.write_str(format!("{}", &meta).as_str())?
    }
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct SymList {
  items : Vec<SymListItem>,
}

impl Deref for SymList {
  type Target = Vec<SymListItem>;

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

// impl Index<usize> for SymList {
//   type Output = SymItem;

//   fn index(&self, idx: I) -> &Self::Output {
//     if let SymListItem::Early(symitem) = self.items[idx] {
//       &symitem
//     }
//     else {
//       panic!("Invalid index access into SymList.");
//     }
//   }
// }

// impl Iterator for SymList {
//   type Item = SymItem;

//   fn index(&self, idx: I) -> &Self::Output {
//     if let SymListItem::Early(symitem) = self.items[idx] {
//       &symitem
//     }
//     else {
//       panic!("Invalid index access into SymList.");
//     }
//   }
// }

// impl Index<I> for SymList {
//   type Output = Symitem;

//   fn index(&self, idx: I) -> &Self::Output {
//     if let SymListItem::Early(symitem) = self.items[idx] {
//       &symitem
//     }
//     else {
//       panic!("Invalid index access into SymList.");
//     }
//   }
// }

// fn get_ptr<T>(x : &T) -> *const u8 {
//   let ptr : *const T = x;
//   NonNull::new(ptr as *mut T).unwrap().as_ptr() as *const u8
// }

// fn get_dist<T>(x : &T, y : &T) -> usize {
//   get_ptr(x) as usize - get_ptr(y) as usize
// }

impl SymList {
  fn new(chars : &mut Chars) -> Result<Self, SymParseError> {
    let list_eof_error = SymParseError::SymListEOF("EOF when building SymList.".to_string());
    let mut list_items = Vec::new();

    while chars.as_str().get(0..1).ok_or(list_eof_error.clone())? != ")" {
      list_items.push(SymListItem::Early(SymItem::try_from(chars.by_ref())?));

      // let test = [SymItem::parse("a").unwrap(),
      // 		  SymItem::parse("b").unwrap()];
      // let test1 = [SymListItem::Early(SymItem::parse("a").unwrap()),
      // 	       	   SymListItem::Early(SymItem::parse("b").unwrap())];
      // let test2 = [SymListItem::Early(SymItem::parse("a").unwrap()),
      // 	       	   SymListItem::Early(SymItem::parse("b").unwrap())];
      // println!("0x{:#x}", get_dist(&test[1], &test[0]));
      // println!("0x{:#x}", get_dist(&test1[1], &test1[0]));
      // println!("0x{:#x}", std::mem::size_of::<SymItem>());
      // println!("0x{:#x}", std::mem::size_of::<SymListItem>());
      // println!("{}", std::mem::align_of::<SymItem>());
      // println!("{}", std::mem::align_of::<SymListItem>());
      // panic!("hi");

      let next_elem_start = chars.as_str().find(|c| { c != ' ' && c != '\n' }).ok_or(list_eof_error.clone())?;
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

#[derive(Debug, Clone)]
pub struct SymAtom {
  symbol : String,
}

impl Deref for SymAtom {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.symbol
  }
}

impl SymAtom {
  fn new(chars : &mut Chars) -> Result<Self, SymParseError> {
    let sym_end = chars.as_str().find(|c: char| { c == '(' || c == ')' || c == ' ' }).unwrap_or(chars.as_str().len());
    Ok(SymAtom {
      symbol : chars.take(sym_end).collect::<String>(),
    })
  }
}
