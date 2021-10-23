use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn helper(limit: usize) -> HashSet<usize> {
  let n = limit;
  let mut i = 1;
  let mut result = HashSet::new();
  while i * i <= limit {
    if n % i == 0 {
      result.insert(i);
      result.insert(n/i);
    }
    i += 1;
    
  }
  result.insert(n);
  result
}

fn main() {
  input!{
    x:usize,
    y:usize
  }

  let xv = helper(x).len();
  let yv = helper(y).len();
  
  if xv == yv {
    println!("Z");
  } else if xv < yv {
    println!("Y");
  } else {
    println!("X");
  }
}