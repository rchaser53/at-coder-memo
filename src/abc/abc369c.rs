/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::*;
use std::collections::*;


fn main() {
  input! {
    n:usize,
    a:[isize;n]
  }

  let mut result = n + n - 1;
  let mut que = VecDeque::new();
  for v in a {
    que.push_back(v);
    let len = que.len();
    if 3 <= len {
      let lv = que[len-1];
      let lv2 = que[len-2];
      let lv3 = que[len-3];

      if lv-lv2 != lv2-lv3 {
        let num = len - 1;
        result += (1+num) * num / 2;
        result -= num + num - 1;
        que.clear();
        que.push_back(lv2);
        que.push_back(lv);
      }
    }
  }

  result += (1+ que.len()) * que.len() / 2;
  result -= que.len() + que.len() - 1;

  println!("{}", result);
}