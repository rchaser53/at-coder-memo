use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:Chars
  }

  let mut map = HashMap::new();
  for c in s {
    *map.entry(c).or_insert(0) += 1;
  }

  for (_, v) in map {
    if v % 2 != 0 {
      println!("No");
      return
    }
  }
  println!("Yes");
}