/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut now = 0isize;
  let mut out = 0isize;
  let mut que = VecDeque::new();
  for _ in 0..n {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        v:isize
      }
      que.push_back((v,now));
      now += v;
    } else if t == 2 {
      if let Some((len, _)) = que.pop_front() {
        out += len;
      }
    } else {
      input! {
        ti:Usize1
      }

      println!("{}", que[ti].1-out);
    }
  }
}