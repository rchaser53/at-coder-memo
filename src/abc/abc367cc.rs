/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

struct Helper {
  k: i32,
  r: Vec<i32>,
  result: Vec<String>,
}
impl Helper {
  fn dfs(&mut self, arr: &mut Vec<i32>, ci:usize, cv:i32) {
    let n = self.r.len();
    if n <= ci {
      if cv % self.k == 0 {
        let temp = arr.clone();
        self.result.push(temp.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
      }

      return
    }

    let v = self.r[ci];
    for x in 1..=v {
      arr.push(x);
      self.dfs(arr, ci+1, cv+x);
      arr.pop();
    }
  }
}

fn main() {
  input! {
    n:usize,
    k:i32,
    r:[i32;n]
  }

  let mut helper = Helper { k, r, result:vec![] };
  helper.dfs(&mut vec![], 0, 0);
  helper.result.sort();
  for v in helper.result {
    println!("{}", v);
  }

}