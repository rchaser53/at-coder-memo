/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    a:usize,
    b:usize
  }

  let mut ast = String::from("");
  let mut bst = String::from("");

  for _ in 0..b {
    ast = format!("{}{}", ast, a);
  }
  for _ in 0..a {
    bst = format!("{}{}", bst, b);
  }

  if ast < bst {
    println!("{}", ast);
  } else {
    println!("{}", bst);
  }
}
