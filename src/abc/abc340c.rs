/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

struct Helper {
  memo: HashMap<usize,usize>,
}

impl Helper {
  fn dfs(&mut self, v:usize) -> usize {
    if v < 2 {
      return 0
    } else if let Some(cv) = self.memo.get(&v) {
      *cv
    } else {
      let mut result = v;
      result += self.dfs(v/2);
      if v % 2 == 1 {
        result += self.dfs(v/2+1);
      } else {
        result += self.dfs(v/2);
      }

      self.memo.insert(v, result);
      result
    }
  }
}

fn main() {
  input! {
    n:usize,
  }

  let mut helper = Helper { memo: HashMap::new() };
  println!("{}", helper.dfs(n));
}