/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const INF:isize = std::isize::MAX;

struct Helper {
  vals: Vec<isize>,
  dp: Vec<Vec<isize>>,
}

impl Helper {
  fn dfs(
    &mut self,
    left: usize,
    right: usize
  ) -> isize {

    if self.dp[left][right] != INF {
      return self.dp[left][right];
    }

    if left == right {
      self.dp[left][right] = self.vals[left];
      return self.dp[left][right];
    }

    let lv = self.vals[left] - self.dfs(left+1, right);
    let rv = self.vals[right] - self.dfs(left, right-1);
    self.dp[left][right] = std::cmp::max(lv, rv);
    self.dp[left][right]
  }
}

pub fn main(
) {
  input! {
    n:usize,
    vals:[isize;n]
  }
  let dp = vec![vec![INF;n];n];
  let mut helper = Helper { vals, dp };
  println!("{}", helper.dfs(0, n-1));
}
