/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }

  let mut result = 0usize;
  let n = s.len();
  for i in 0..n-1 {
    if s[i] != s[i+1] && s[i] as usize - '0' as usize + 1 == s[i+1] as usize - '0' as usize {
      result += 1;
      let a = s[i];
      let b = s[i+1];
      let mut x = 1;
      while x <= i && i+1+x < n {
        if s[i-x] == a && s[i+1+x] == b {
          result += 1;
          x += 1;
        } else {
          break;
        }
      }
    }
  }
  println!("{}", result);
}