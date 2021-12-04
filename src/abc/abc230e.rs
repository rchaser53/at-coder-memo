/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }
  
  let mut result = 0usize;
  let mut i = 1;
  while i <= n {
    let a = n / i;
    let b = n % i;
    
    let next = b / a + 1;
    result += next * a;
    i += next;
  }
  println!("{}", result);
}