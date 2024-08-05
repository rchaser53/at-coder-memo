/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }
  
  let mut memo = vec![vec![0;3];n+1];
  let p = [
    // others, win, lose
    ((1,2),'S','P'),  // R
    ((0,2),'P','R'),  // S
    ((0,1),'R','S'),  // P
    
    ];
  for i in 0..n {
    let c1 = s[i];
    for j in 0..3 {
      let ((li1,li2), win_c, lose_c) = p[j];
      if c1 == lose_c { continue }
      memo[i+1][j] = memo[i][li1].max(memo[i][li2]);
      if c1 == win_c {
        memo[i+1][j] += 1;
      }

    }
  }

  println!("{}", memo[n].iter().max().unwrap());
}