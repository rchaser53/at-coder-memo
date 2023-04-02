/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize
  }

  if n < m / n {
    println!("-1");
    return
  }

  let mut a = 1;
  let mut result = usize::max_value();
  while a * a <= m && a <= n {
    let b = if m % a == 0 {
      m / a
    } else {
      m / a + 1
    };

    if b <= n {
      let x = a * b;
      result = result.min(x);
    }
    a += 1;
  }

  let b = if m % a == 0 {
    m / a
  } else {
    m / a + 1
  };
  if b <= n && a <= n {
    let x = a * b;
    result = result.min(x);
  }
  
  if result == usize::max_value() {
    println!("-1");
  } else {
    println!("{}", result);
  }

}