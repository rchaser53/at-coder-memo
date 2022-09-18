/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

// ２次元いもす
// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_i
fn main() {
  input! { 
    h:usize,
    w:usize,
    n:usize,
    vals:[(Usize1,Usize1,Usize1,Usize1);n]
  }

  let mut memo = vec![vec![0isize;w+1];h+1];
  for (a,b,c,d) in vals {
    memo[a][b] += 1;
    memo[c+1][b] -= 1;
    memo[a][d+1] -= 1;
    memo[c+1][d+1] += 1;
  }

  for i in 0..h {
    for j in 0..w {
      memo[i][j+1] += memo[i][j];
    }
  }

  for i in 0..w {
    for j in 0..h {
      memo[j+1][i] += memo[j][i];
    }
  }

  for i in 0..h {
    println!("{}", memo[i][..w].iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  }
}