use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    vals:[usize;3]
  }

  let a = vals[0] + vals[1] == vals[2];
  let b = vals[0] - vals[1] == vals[2];

  if a && b {
    println!("?")
  } else if a {
    println!("+")
  } else if b {
    println!("-")
  } else {
    println!("!")
  }
}