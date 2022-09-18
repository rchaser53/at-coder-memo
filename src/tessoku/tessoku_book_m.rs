/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;


fn main() {
  input! { 
    n:usize,
    k:usize,
    mut vs:[usize;n]
  }
  vs.reverse();

  let mut result = 0;
  let mut ri = 1;
  for li in 0..n-1 {
    while ri < n && vs[li] - vs[ri] <= k {
      ri += 1;
    }

    // li:2 ri:4
    // 2,4 3,4
    if ri == n {
      if vs[li] - vs[n-1] <= k {
        result += n - li - 1;
      } else {
        break
      }
    } else {
      result += ri - li - 1;
    }
  }

  println!("{}", result);
}