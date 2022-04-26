mod sym;

pub use sym::SymItem;

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

  if let Some(res) = SymItem::parse("hello") { println!("{}", res.as_str().unwrap_or("INVALID")) }
  if let Some(res) = SymItem::parse("hello") { println!("{:?}", res.as_list().unwrap_or(&vec![])) }
  if let Some(res) = SymItem::parse("(a b c)") { println!("{}", res.as_str().unwrap_or("INVALID")) }
  if let Some(res) = SymItem::parse("(a b c)") { println!("{:?}", res.as_list().unwrap_or(&vec![])) }
}
