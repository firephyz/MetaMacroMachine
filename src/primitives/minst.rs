use std::convert::TryFrom;
use std::fmt::{self, Display};

use crate::parse::SymItem;

#[derive(Debug)]
enum EncodingError {
  InvalidArg(i32),
}

#[derive(Debug)]
enum DecodingError {
  InvalidInstEncoding(u32),
  InvalidInstWithArgs(u32),
  InvalidContextRange(i32, i32),
}

#[derive(Debug, Clone)]
pub enum MinstSymItemError<'a> {
  NotAnInstruction(&'a SymItem),
  InvalidInstr(&'a SymItem),
  InvalidArgs(&'a SymItem),
}

#[derive(Debug, Clone)]
pub enum MacroInstruction {
  Define,
  Expand,
  Index{frame: i32, narg: Option<i32>},
  Context{range: Option<(i32, i32)>},
  Return{range: Option<ReturnInstData>},
}

#[derive(Debug, Clone)]
pub enum ReturnInstData {
    Arg(i32),
    Range(i32, i32),
}

impl TryFrom<MInstEncoding> for MacroInstruction {
  type Error = DecodingError;

  fn try_from(inst : MInstEncoding) -> Result<Self, Self::Error> {
    let word = inst.inst;
    let inst_encoding = word & 0x7;
    let inst_variation = (word & 0x80000000) != 0;
    let (inst_arg0, inst_arg1) = ((word >> 3 & 0x3FFF) as i32, (word >> (14 + 3) & 0x3FFF) as i32);

    let check_null_args = || {
      let arg_bits = (word >> 3) & 0xFFFFFFF;
      if !(inst_variation && arg_bits == 0) {
	Err(DecodingError::InvalidInstWithArgs(word))
      }
      else {
	Ok(())
      }
    };

    // Functions to map function arguments to packaged MacroInstruction arguments
    let arg_map_index = |arg: i32| -> Result<Option<i32>, DecodingError> {
      if inst_variation { Ok(Some(arg)) } else { Ok(None) }
    };
    let arg_map_context = |arg0: i32, arg1: i32| -> Result<Option<(i32, i32)>, DecodingError> {
      if inst_variation {
	if arg0 == arg1 {
	  Err(DecodingError::InvalidContextRange(arg0, arg1))
	}
	else {
	  Ok(Some((arg0, arg1)))
	}
      }
      else {
	Ok(None)
      }
    };
    let arg_map_return = |start: i32, end: i32| -> Result<Option<ReturnInstData>, DecodingError> {
      if inst_variation {
	if start == end {
	  Ok(Some(ReturnInstData::Arg(start)))
	}
	else {
	  Ok(Some(ReturnInstData::Range(start, end)))
	}
      }
      else {
	Ok(None)
      }
    };    

    // Construct the instruction
    check_null_args()?;
    match inst_encoding {
      0 => Ok(MacroInstruction::Define),
      1 => Ok(MacroInstruction::Expand),
      2 => Ok(MacroInstruction::Index{frame: inst_arg0, narg: arg_map_index(inst_arg1)?}),
      3 => Ok(MacroInstruction::Context{range: arg_map_context(inst_arg0, inst_arg1)?}),
      4 => Ok(MacroInstruction::Return{range: arg_map_return(inst_arg0, inst_arg1)?}),
      _ => Err(DecodingError::InvalidInstEncoding(inst_encoding)),
    }
  }
}

impl<'m> TryFrom<&'m SymItem> for MacroInstruction {
  type Error = MinstSymItemError<'m>;

  fn try_from(sym : &'m SymItem) -> Result<Self, Self::Error> {
    let inst_name = sym.index_early(0).unwrap().as_str().unwrap();

    // Check for machine instruction dot
    if inst_name.chars().nth(0).unwrap() != '.' {
      Err(MinstSymItemError::NotAnInstruction(sym.index_early(0).unwrap()))?
    }

    // Convert arguments into integers
    let args_as_integers = sym.as_list().unwrap().
      iter().skip(1)
      .map(|sym| {
	let inner_sym = sym.into_inner_early().unwrap();
	inner_sym.as_str().unwrap()
	  .parse::<i32>().or(Err(MinstSymItemError::InvalidArgs(inner_sym)))
      }).try_collect::<Vec<i32>>()?;
    let num_args = args_as_integers.len();

    // Dispatch create MacroInstructions based off of the first symbol name
    let test = Err(MinstSymItemError::InvalidInstr(sym));
    match inst_name {
      ".DEFINE" => {
	if num_args != 0 { Err(MinstSymItemError::InvalidInstr(sym)) }
	else { Ok(MacroInstruction::Define) }
      },
      ".EXPAND" => test,
      ".CONTEXT" => {
	match num_args {
	  0 => Ok(MacroInstruction::Context{range: None}),
	  2 => Ok(MacroInstruction::Context{range: Some((args_as_integers[0], args_as_integers[1]))}),
	  _ => Err(MinstSymItemError::InvalidInstr(sym)),
	}
      },
      ".INDEX" => {
	match num_args {
	  0 => Err(MinstSymItemError::InvalidArgs(sym)),
	  1 => Ok(MacroInstruction::Index{frame: args_as_integers[0], narg: None}),
	  2 => Ok(MacroInstruction::Index{frame: args_as_integers[0], narg: Some(args_as_integers[1])}),
	  _ => Err(MinstSymItemError::InvalidInstr(sym)),
	}
      },
      ".RETURN" => {
	match num_args {
	  0 => Ok(MacroInstruction::Return{range: None}),
	  1 => Ok(MacroInstruction::Return{range: Some(ReturnInstData::Arg(args_as_integers[0]))}),
	  2 => {
	    if args_as_integers[0] == args_as_integers[1] {
	      Err(MinstSymItemError::InvalidInstr(sym))
	    }
	    else {
	      Ok(MacroInstruction::Return{range: Some(ReturnInstData::Range(args_as_integers[0], args_as_integers[1]))})
	    }
	  }
	  _ => Err(MinstSymItemError::InvalidInstr(sym)),
	}
      },
      _ => Err(MinstSymItemError::InvalidInstr(sym)),
    }
  }
}

impl From<&MacroInstruction> for u32 {
  fn from(inst: &MacroInstruction) -> u32 {
    match inst {
      MacroInstruction::Define => 0,
      MacroInstruction::Expand => 1,
      MacroInstruction::Index{frame: _, narg: _} => 2,
      MacroInstruction::Context{range: _} => 3,
      MacroInstruction::Return{range: _} => 4,
    }
  }
}

impl Display for MacroInstruction {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      MacroInstruction::Define => fmt.write_str("(.DEFINE)"),
      MacroInstruction::Expand => fmt.write_str("(.EXPAND)"),
      MacroInstruction::Index{frame, narg} => {
	match narg {
	  Some(narg) => fmt.write_str(format!("(.INDEX {} {})", frame, narg).as_str()),
	  None => fmt.write_str(format!("(.INDEX {})", frame).as_str()),
	}
      },
      MacroInstruction::Context{range} => {
	match range {
	  Some(range) => fmt.write_str(format!("(.CONTEXT {} {})", range.0, range.1).as_str()),
	  None => fmt.write_str(format!("(.CONTEXT)").as_str()),
	}
      },
      MacroInstruction::Return{range} => {
	match range {
	  Some(range) => {
	    match range {
	      ReturnInstData::Arg(arg) => fmt.write_str(format!("(.RETURN {})", arg).as_str()),
	      ReturnInstData::Range(arg0, arg1) => fmt.write_str(format!("(.RETURN {} {})", arg0, arg1).as_str()),
	    }
	  },
	  None => fmt.write_str("(.RETURN)"),
	}
      },
    }
  }
}

#[derive(Debug)]
struct MInstEncoding {
  inst : u32,
}

impl TryFrom<MacroInstruction> for MInstEncoding {
  type Error = EncodingError;

  fn try_from(inst_type : MacroInstruction) -> Result<Self, Self::Error> {
    let (inst_enc, inst_args) = {
      let arg_vec = match inst_type {
	MacroInstruction::Define => vec![],
	MacroInstruction::Expand => vec![],
	MacroInstruction::Index{frame, narg} => {
	  match narg {
	    Some(narg) => vec![frame, narg],
	    None => vec![frame],
	  }
	}
	MacroInstruction::Context{range} => {
	  match range {
	    Some((x0, x1)) => vec![x0, x1],
	    None => vec![],
	  }
	},
	MacroInstruction::Return{ref range} => {
	  match range {
	    Some(range) => {
	      match range {
		ReturnInstData::Arg(arg) => vec![*arg],
		ReturnInstData::Range(arg0, arg1) => vec![*arg0, *arg1],
	      }
	    },
	    None => vec![],
	  }
	},
      };

      (u32::from(&inst_type), arg_vec)
    };

    // let arg_bounds = (0xFFF as i32, -1 * 0x1000 as i32);
    let arg_mask = 0x3FFF;
    let inst_data = (||{
      match inst_args.len() {
	0 => Ok(0),
	_ => {
	  let failed_args = inst_args.iter().filter_map(|arg| {
	    if *arg & !arg_mask > 0 { Some(arg.clone()) } else { None }
	  }).collect::<Vec<i32>>();

	  if failed_args.len() > 0 {
	    Err(EncodingError::InvalidArg(failed_args[0]))
	  }
	  else {
	    Ok((inst_args[0] & arg_mask) << 3 | (inst_args[1] & arg_mask) << (3 + 14))
	  }
	},
      }
    })()?;
    // let inst_data = (||{
    //   if inst_args.len() == 1 {
    // 	let 
    // 	let x0 = inst_args[0];
    // 	if x0 > arg_bounds.0 || x0 < arg_bounds.1 { Err(EncodingError::InvalidArg(x0))? }
    // 	return Ok((x0 & 0xFFFF) << 3)
    //   }
    //   if inst_args.len() == 2 {
    // 	let (x0, x1) = (inst_args[0], inst_args[1]);
    // 	if x0 > 0xFFF || x0 < (-1 * 0x1000) { Err(EncodingError::InvalidArg(x0))? }
    // 	if x1 > 0xFFF || x1 < (-1 * 0x1000) { Err(EncodingError::InvalidArg(x1))? }
    // 	return Ok((x0 << 3) | (x1 << (13 + 3)))
    //   }
    //   return Ok(0)
    // })()?;

    Ok(MInstEncoding { inst : inst_enc | inst_data as u32})
  }
}
