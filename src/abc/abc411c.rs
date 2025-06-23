/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    a:[Usize1;q]
  }

  let mut memo = vec![false;n];
  let mut count = 0;
  for v in a {
      let is_left = if 0 < v && memo[v-1] {
        true
      } else {
        false
      };

      let is_right = if v+1 < n && memo[v+1] {
        true
      } else {
        false
      };

    if memo[v] {
      memo[v] = false;
      // .ooo. => .oxo.
      if is_left && is_right {
        count += 1;
      }
      // .xox. => .xxx.
      else if !is_left && !is_right {
        count -= 1;
      }
    } else {
      memo[v] = true;
      // .oxo. => .ooo.
      if is_left && is_right {
        count -= 1;
      }
      // .xxx. => .xox.
      else if !is_left && !is_right {
        count += 1;
      }
    }

    println!("{count}");
  }
}