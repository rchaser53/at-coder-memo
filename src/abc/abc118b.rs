#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main() {
  input! {
    n:usize,
    m:usize,
  }

  let mut memo = vec![false;m];
  input!{
    k:usize,
    vals:[Usize1;k]
  }
  for ti in vals {
    memo[ti] = true;
  }

  for _ in 1..n {
    let mut set = HashSet::new();
    input!{
      k:usize,
      vals:[Usize1;k]
    }
    for v in vals {
      set.insert(v);
    }

    for i in 0..m {
      if memo[i] {
        memo[i] = set.contains(&i);
      }
    }
  }

  let mut result = 0;
  for f in memo {
    if f {
      result += 1;
    }
  }
  println!("{}", result);
}
