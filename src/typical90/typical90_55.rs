/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

struct Helper {
  vals: Vec<usize>,
  count: usize,
  p: usize,
  q: usize
}

impl Helper {
  fn dfs(
    &mut self,
    v: usize,
    ci: usize,
    num: usize
  ) {
    if num == 5 {
      if v % self.p == self.q {
        self.count += 1;
      }
      return
    }

    for i in ci+1..self.vals.len() {
      self.dfs(
        (v * self.vals[i]) % self.p, i, num+1
      );
    }
  }
}

pub fn main(
) {
  input! {
    n:usize,
    p:usize,
    q:usize,
    vals:[usize;n]
  }
  let mut helper = Helper{ vals, p, q, count:0 };
  for i in 0..=n-5 {
    helper.dfs(helper.vals[i], i, 1);
  }
  println!("{}", helper.count);
}
