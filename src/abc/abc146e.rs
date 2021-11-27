/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[Usize1;n]
  }

  if k == 1 {
    println!("0");
    return
  }

  let mut result = 0usize;
  let mut memo = vec![0usize;n+1];
  let mut map = HashMap::new();
  map.insert(0usize, 1usize);

  for (i, v) in vals.into_iter().enumerate() {
    memo[i+1] = (memo[i]+v) % k;
    match map.get(&memo[i+1]) {
      Some(&tv) => {
        result += tv;
        map.insert(memo[i+1], tv+1);
      }
      None => {
        map.insert(memo[i+1], 1);
      }
    }
    if k <= i+2 {
      map.insert(memo[i+2-k], map.get(&memo[i+2-k]).unwrap()-1);
    }
  }
  println!("{}", result);
}