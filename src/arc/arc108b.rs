/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut result = vec![];

  for c in s {
    result.push(c);
    while 3 <= result.len() {
      let len = result.len();
      if result[len-3] == 'f' && result[len-2] == 'o' && result[len-1] == 'x' {
        result.pop();
        result.pop();
        result.pop();
      } else {
        break
      }
    }
  }

  println!("{}", result.len());
}