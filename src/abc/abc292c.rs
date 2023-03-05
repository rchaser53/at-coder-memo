/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input!{
    n:usize
  }

  let mut memo = vec![0;n+1];
  for i in 1..n {
    let mut set = HashSet::new();

    let mut j = 1;
    while j * j <= i {
      if i % j == 0 {
        set.insert((j, i/j));
        set.insert((i/j, j));
      }
      j += 1;
    }

    if i % j == 0 {
      set.insert((j, i/j));
      set.insert((i/j, j));
    }
    memo[i] = set.len();
  }

  let mut result = 0;
  for i in 1..n {
    let v1 = memo[i];
    let v2 = memo[n-i];
    result += v1 * v2;
  }
  println!("{}", result);
}