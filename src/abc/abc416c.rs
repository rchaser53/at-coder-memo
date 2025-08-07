/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

struct Helper {
  arr: Vec<String>,
  s: Vec<String>,
  k: usize
}

impl Helper {
  fn dfs(&mut self, arr: &mut Vec<String>) {
    if arr.len() == self.k {
      self.arr.push(arr.join(""));
      return;
    }

    for i in 0..self.s.len() {
      arr.push(self.s[i].clone());
      self.dfs(arr);
      arr.pop();
    }
  }
}


fn main() {
  input! {
    n:usize,
    k:usize,
    x:Usize1,
    s:[String;n]
  }

  let mut helper = Helper {
    arr: vec![],
    s,
    k
  };
  helper.dfs(&mut vec![]);

  helper.arr.sort();
  println!("{}", helper.arr[x]);
}