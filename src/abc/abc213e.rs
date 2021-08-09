/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(
  memo: &mut Vec<Vec<usize>>,
  que: &mut VecDeque<(isize, isize, usize)>,
  r:isize,
  c:isize,
  v:usize
) { 
  let h = memo.len() as isize;
  let w = memo[0].len() as isize;

  if r < 0 || c < 0 || h <= r || w <= c {
    return
  }

  let ru = r as usize;
  let cu = c as usize;
  if memo[ru][cu] <= v {
    return
  }
  memo[ru][cu] = v;
  que.push_back((r,c,v));
}

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let inf = 1_000_000_000_000usize;
  let mut memo = vec![vec![inf;w];h];
  memo[0][0] = 0;
  
  let mut que = VecDeque::new();
  que.push_back((0,0,0));
  while let Some((yi, xi, v)) = que.pop_front() {
    let y = yi as usize;
    let x = xi as usize;
    let nv = v + 1;
    if 0 < y {
      if rows[y-1][x] == '.' && v < memo[y-1][x] {
        memo[y-1][x] = v;
        que.push_front((yi-1,xi,v));
      } else {
        for i in yi-2..yi {
          for j in xi-1..xi+2 {
            helper(&mut memo, &mut que, i, j, nv);
          }
        }
      }
    }

    if y < h-1 {
      if rows[y+1][x] == '.' && v < memo[y+1][x] {
        memo[y+1][x] = v;
        que.push_front((yi+1,xi,v));
      } else {
        for i in yi+1..yi+3 {
          for j in xi-1..xi+2 {
            helper(&mut memo, &mut que, i, j, nv);
          }
        }
      }
    }

    if 0 < x {
      if rows[y][x-1] == '.' && v < memo[y][x-1] {
        memo[y][x-1] = v;
        que.push_front((yi,xi-1,v));
      } else {
        for i in yi-1..yi+2 {
          for j in xi-2..xi {
            helper(&mut memo, &mut que, i, j, v+1);
          }
        }
      }
    }

    if x < w-1 {
      if rows[y][x+1] == '.' && v < memo[y][x+1] {
        memo[y][x+1] = v;
        que.push_front((yi,xi+1,v));
      } else {
        for i in yi-1..yi+2 {
          for j in xi+1..xi+3 {
            helper(&mut memo, &mut que, i, j, v+1);
          }
        }
      }
    }
  }

  println!("{}", memo[h-1][w-1]);
}