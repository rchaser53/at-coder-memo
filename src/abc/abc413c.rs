/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    q:usize
  }

  let mut que = VecDeque::new();
  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        c:isize,
        x:isize
      }

      que.push_back((x, c));

      continue 
    }

    input! {
      mut k:isize
    }

    let mut result = 0isize;
    while 0 < k {
      if let Some((x,mut c)) = que.pop_front() {
        if k < c {
          c -= k;
          que.push_front((x, c));
          result += x * k;
          k = 0;
        } else if k == c {
          result += x * k;
          k = 0;
        } else {
          result += x * c;
          k -= c;
        }
      }
    }

    println!("{}", result);
  }
}