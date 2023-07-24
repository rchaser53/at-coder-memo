/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    s:[Chars;n]
  }

  let mut seen = vec![vec![vec![false;4];m];n];
  let mut que:VecDeque<(usize,usize,usize)> = VecDeque::new();
  for i in 0..4 {
    que.push_back((1,1,i));
    seen[1][1][i] = true;
  }
  
  let dict: [(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];
  while let Some((i,j,di)) = que.pop_front() {
    let ni = (i as i32 + dict[di].0) as usize;
    let nj = (j as i32 + dict[di].1) as usize;

    seen[i][j][di] = true;
    if s[ni][nj] == '#' {
      for ndi in 0..4 {
        if !seen[i][j][ndi] {
          seen[i][j][ndi] = true;
          que.push_back((i,j,ndi));
        }
      }
    } else {
      que.push_front((ni,nj,di));
    }
  }

  let mut result = 0;
  for i in 0..n {
    for j in 0..m {
      for k in 0..4 {
        if seen[i][j][k] {
          result += 1;
          break
        }
      }
    }
  }
  println!("{}", result);
  
}