/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    s:Chars,
  }

  let mut result = vec![];
  for c in s {
    result.push(c);
    let li = result.len()-1;
    if li > 1 {
     if result[li] == 'C' && result[li-1] == 'B' && result[li-2] == 'A' {
      result.pop();
      result.pop();
      result.pop();
     }
    }
  }
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}