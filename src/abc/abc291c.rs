/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    s:Chars
  }

  let mut set = HashSet::new();
  let mut x = 0;
  let mut y = 0;
  set.insert((x,y));
  for c in s {
    if c == 'R' {
      x += 1;
    } else if c == 'L' {
      x -= 1;
    } else if c == 'U' {
      y += 1;
    } else {
      y -= 1;
    }
    if set.contains(&(x,y)) {
      println!("Yes");
      return
    }
    set.insert((x,y));
  }

  println!("No");
}