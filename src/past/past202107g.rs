use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    mut n:isize,
  }
  let mut result = vec![];
  let mut now = 1usize;
  while 0 < n {
    if n % 3 == 1 {
      result.push(format!("{}", now));
      n -= 1;
    } else if n % 3 == 2 {
      result.push(format!("-{}", now));
      n += 1;
    }
    now *= 3;
    n /= 3;
  }
  

  println!("{}", result.len());
  println!("{}", result.join(" "));
}