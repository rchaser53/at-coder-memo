use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(a:char, b:char, def:char) -> bool {
  a == def && b == def
}

fn main() {
  input!{
    n:usize,
    mut s:Chars
  }

  if n < 3 {
    println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
    return
  }

  for i in 0..s.len()-2 {
    let c1 = s[i];
    let c2 = s[i+1];
    let c3 = s[i+2];
    if c2 != 'x' { continue }

    let is_ok = helper(c1, c3, 'a')
    || helper(c1, c3, 'i')
    || helper(c1, c3, 'u')
    || helper(c1, c3, 'e')
    || helper(c1, c3, 'o');

    if is_ok {
      s[i] = '.';
      s[i+1] = '.';
      s[i+2] = '.';
    }
  }
  println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
}