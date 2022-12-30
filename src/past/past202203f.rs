/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn helper(
  memo: &Vec<Vec<(usize,usize)>>,
  i:usize,
  j:usize,
  ti:usize,
  tj:usize
) -> bool {
  memo[i][j].0 != memo[ti][tj].0
    && memo[i][j].1 == memo[ti][tj].1
}

fn main() {
  input! {
    h:usize,
    w:usize,
    n:usize,
    a:[[Usize1;w];h],
    c:[usize;n]
  }

  let mut memo = vec![vec![(0,0);w];h];
  for i in 0..h {
    for j in 0..w {
      let ti = a[i][j];
      memo[i][j] = (a[i][j], c[ti]);
    }
  }

  for i in 0..h {
    for j in 0..w {
      let v1 = helper(&memo, i, j, i.saturating_sub(1), j);
      let v2 = helper(&memo, i, j, (h-1).min(i+1), j);
      let v3 = helper(&memo, i, j, i, j.saturating_sub(1));
      let v4 = helper(&memo, i, j, i, (w-1).min(j+1));
      if v1 || v2 || v3 || v4 {
        println!("No");
        return
      }
    }
  }

  println!("Yes");
}