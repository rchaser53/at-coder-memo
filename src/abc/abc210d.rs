/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(
  rows: &Vec<Vec<isize>>,
  h: usize,
  w: usize,
  c: isize
) -> isize {
  let inf = isize::max_value();
  let mut min = inf;
  let mut memo = vec![vec![inf;w];h];
  memo[0][0] = rows[0][0];
  for i in 1..w {
    let v = i as isize * c;
    min = std::cmp::min(rows[0][i] + memo[0][i-1] + v, min);
    memo[0][i] = std::cmp::min(rows[0][i] - v, memo[0][i-1]);
  }
  for i in 1..h {
    let v = i as isize * c;
    min = std::cmp::min(rows[i][0] + memo[i-1][0] + v, min);
    memo[i][0] = std::cmp::min(rows[i][0] - v, memo[i-1][0]);
  }

  for i in 1..h {
    for j in 1..w {
      let v = (i + j) as isize * c;
      let temp_min = std::cmp::min(memo[i-1][j], memo[i][j-1]);
      // println!("i:{} j:{} temp_min:{} v:{} cost:{}", i, j, temp_min, v, rows[i][j] + v + temp_min);
      min = std::cmp::min(rows[i][j] + v + temp_min, min);
      memo[i][j] = std::cmp::min(temp_min, rows[i][j] - v);
    }
  }

  min
}

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    c:isize,
    mut rows:[[isize;w];h]
  }

  let normal = helper(&rows, h, w, c);
  for i in 0..h {
    rows[i].reverse();
  }
  let reverse = helper(&rows, h, w, c);

  println!("{}", std::cmp::min(normal, reverse));
}
