/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use num_complex::Complex;

fn helper(
  memo: &mut Vec<Vec<usize>>,
  que: &mut VecDeque<(usize,usize,usize,usize)>,
  x:usize, y:usize, v:usize,
  t:usize, a:usize) -> bool {
  let nv = if t == a { v } else { v + 1 };
  if nv < memo[x][y] {
    if t == a {
      que.push_front((x, y, nv, a));
    } else {
      que.push_back((x, y, nv, a));
    }
    memo[x][y] = nv;
    false
  } else if memo[x][y] < nv {
    return true
  } else if memo[x][y] == nv && t == a {
    return true
  } else {
    false
  }
}

fn main() {
  input! {
    n:usize,
    mut aa: (Usize1, Usize1),
    mut bb: (Usize1, Usize1),
    rows:[Chars;n]
  }

  let inf = 1_000_000_000_000usize;
  let mut memo = vec![vec![inf;n];n];
  memo[aa.0][aa.1] = 0;
 
  let mut que = VecDeque::new();
  que.push_back((aa.0, aa.1, 0, 100));
 
  while let Some((x, y, v, t)) = que.pop_front() {
    if 0 < x && 0 < y {
      let mut tx = x - 1;
      let mut ty = y - 1;
      while rows[tx][ty] == '.' {
        if helper(&mut memo, &mut que, tx, ty, v, t, 0) {
          break
        } else if tx == 0 || ty == 0 {
          break
        }
        tx -= 1;
        ty -= 1;
      }
    }
    if x < n - 1 && y < n - 1 {
      let mut tx = x + 1;
      let mut ty = y + 1;
      while rows[tx][ty] == '.' {
        if helper(&mut memo, &mut que, tx, ty, v, t, 0) {
          break
        } else if tx == n-1 || ty == n-1 {
          break
        }

        tx += 1;
        ty += 1;
      }
    }
 
    if 0 < x && y < n - 1 {
      let mut tx = x - 1;
      let mut ty = y + 1;
      while rows[tx][ty] == '.' {
        if helper(&mut memo, &mut que, tx, ty, v, t, 1) {
          break
        } else if tx == 0 || ty == n-1 {
          break
        }

        tx -= 1;
        ty += 1;
      }
    }
    if x < n-1 && 0 < y {
      let mut tx = x + 1;
      let mut ty = y - 1;
      while rows[tx][ty] == '.' {
        if helper(&mut memo, &mut que, tx, ty, v, t, 1) {
          break
        } else if tx == n-1 || ty == 0 {
          break
        }

        tx += 1;
        ty -= 1;
      }
    }
  }
 
  if memo[bb.0][bb.1] == inf {
    println!("-1");
  } else {
    println!("{}", memo[bb.0][bb.1]);
  }
}