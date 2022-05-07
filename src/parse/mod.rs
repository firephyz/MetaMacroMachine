mod sym;

pub use sym::{SymItem, SymAtom, SymList};


// enum ExprDisplayModeType {
//   Flat,
//   PrettyPrint,
// }

// static mut ExprDisplayMode: ExprDisplayModeType = ExprDisplayModeType::Flat;

// pub fn toggle_pretty_print() {
//   static was_enabled: bool = ExprDisplayMode == ExprDisplayModeType::PrettyPrint;
//   if !was_enabled {
    
// pub fn pretty_print() {
//   unsafe {
//     ExprDisplayMode = ExprDisplayModeType::PrettyPrint;
//   }
// }
// pub fn flat_print() {
//   unsafe {
//     ExprDisplayMode = ExprDisplayModeType::PrettyPrint;
//   }
// }

#[allow(dead_code)]
fn test_patterns() {
  if let Some(res) = SymItem::parse("abc") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse(")abc") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse("(abc") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse(" abc") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse("abc(") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse("abc)") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse("abc ") { println!("{:?}", res); }
  if let Some(res) = SymItem::parse("(ab)") { println!("{}", res); }
  if let Some(res) = SymItem::parse("(ab cd) ") { println!("{}", res); }
  if let Some(res) = SymItem::parse("()") { println!("{}", res); }
  if let Some(res) = SymItem::parse("(((a) bc (d e f) () (() g h) (i())) a)") { println!("{}", res); }

  unimplemented!("Test incomplete for unwrap_or arguments.");
  // if let Some(res) = SymItem::parse("hello") { println!("{}", res.as_str().unwrap_or("INVALID")) }
  // if let Some(res) = SymItem::parse("hello") { println!("{:?}", res.as_list().unwrap_or(&vec![])) }
  // if let Some(res) = SymItem::parse("(a b c)") { println!("{}", res.as_str().unwrap_or("INVALID")) }
  // if let Some(res) = SymItem::parse("(a b c)") { println!("{:?}", res.as_list().unwrap_or(&vec![])) }
}
