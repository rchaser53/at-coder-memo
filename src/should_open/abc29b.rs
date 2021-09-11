use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    ss:[Chars;12]
  }

  let mut count = 0;
  for i in 0..12 {
    let mut f = false;
    for &c in &ss[i] {
      if c == 'r' {
        f = true;
      }
    }
    if f {
      count += 1;
    }
  }

  println!("{}", count);
}