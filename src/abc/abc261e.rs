/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    mut c:usize,
    queries:[(usize,usize);n]
  }

  let limit = 31;
  let mut memo = vec![(0,1);limit];
  let top = 30;
  for (t, v) in queries {    
    if t == 1 {
      for i in 0..top {
        let v2 = (v >> i) & 1;
        memo[i] = (memo[i].0 & v2, memo[i].1 & v2);
      }

    } else if t == 2 {
      for i in 0..top {
        let v2 = (v >> i) & 1;
        memo[i] = (memo[i].0 | v2, memo[i].1 | v2);
      }
    } else {
      for i in 0..top {
        let v2 = (v >> i) & 1;
        memo[i] = (memo[i].0 ^ v2, memo[i].1 ^ v2);
      }
    }

    let mut temp = 0usize;
    for i in 0..top {
      let v = if (c >> i) & 1 == 1 {
        memo[i].1
      } else {
        memo[i].0
      };

      if v == 1 {
        temp += 1 << i;
      }
    }
    println!("{}", temp);
    c = temp;
  }
}