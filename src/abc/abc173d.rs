/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut vals: [usize;n]
  }

  vals.sort();
  vals.reverse();
  let mut result = vals[0];
  let mut que = VecDeque::new();
  que.push_back(vals[1]);

  let mut i = 2;
  let mut flag = false;
  while i < n {
    result += que[0];
    que.push_back(vals[i]);
    if flag {
      que.pop_front();
    }
    flag = !flag;
    i += 1;
  }

  println!("{}", result);
}