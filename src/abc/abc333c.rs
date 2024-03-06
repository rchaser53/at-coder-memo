/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

struct Helper {
  arr: Vec<usize>,
  set: HashSet<usize>,
}

impl Helper {
  fn dfs(&mut self, ci:usize, num:usize, tot:usize) {
    if num == 3 {
      self.set.insert(tot);
      return
    }

    if self.arr.len() <= ci {
      return
    }

    self.dfs(ci, num+1, tot+self.arr[ci]);
    self.dfs(ci+1, num, tot);
  }
}

fn main() {
  input! {
    n:Usize1,
  }

  let mut helper = Helper {
    arr: vec![1usize,11,111,1111,11111,
    111111,1111111,11111111,111111111,1111111111
    ,11111111111, 111111111111],
    set: HashSet::new()
  };

  helper.dfs(0,0,0);
  let mut arr = helper.set.into_iter().collect::<Vec<usize>>();
  arr.sort();

  println!("{}", arr[n]);
}