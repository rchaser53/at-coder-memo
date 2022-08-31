/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

struct Helper {
  memo: Vec<Option<f64>>
}

impl Helper {
  fn dfs(&mut self, i:usize) -> f64 {
    if let Some(v) = self.memo[i] {
      v
    } else {
      let mut result = 0f64;
      for j in 1..=6 {
        let v1 = j as f64;
        let v2 = self.dfs(i-1);
        if v1 < v2 {
          result += v2 / 6f64;
        } else {
          result += v1 / 6f64;
        }
      }
      self.memo[i] = Some(result);
      result
    }
  }
}

fn main() {
  input! {
    n:usize
  }

  let mut helper = Helper { memo: vec![None;n] };
  helper.memo[0] = Some(3.5);
  helper.dfs(n-1);
  println!("{}", helper.memo[helper.memo.len()-1].unwrap());
}