use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    a:usize,
    b:usize
  }

  if n == 1 {
    if a != b {
      println!("0");
    } else {
      println!("1");
    }
    return
  }

  if b < a {
    println!("0");
    return
  } else if b == a {
    println!("1");
    return
  }

  let min = a * (n-1) + b;
  let max = b * (n-1) + a;

  println!("{}", max - min + 1);
}