use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let ww = rows[0].len()+2;
  rows.insert(0,vec!['#';ww]);
  rows.push(vec!['#';ww]);

  for i in 1..=h {
    rows[i].insert(0, '#');
    rows[i].push('#');
  }
  
  for v in rows {
    println!("{}", v.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}
