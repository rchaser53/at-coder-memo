/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    s:Chars,
    mut t:Chars,
  }

  if t[t.len()-1] == 'X' {
    t.pop();
  }
  t.reverse();

  for i in 0..t.len() {
    t[i] = t[i].to_lowercase().to_string().chars().next().unwrap();
  }

  for c in s {
    if t.is_empty() {
      break
    }
    if c == t[t.len()-1] {
      t.pop();
    }
  }
  
  if t.is_empty() {
    println!("Yes");
  } else {
    println!("No");
  }
}
