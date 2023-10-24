/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    t:Chars,
    ss:[Chars;n]
  }

  let m = t.len();
  let mut memo = vec![(0,0);n];
  let mut adds = vec![0;m+1];
  for i in 0..n {
    let s = &ss[i];
    let x = s.len();
    let mut ti = 0;
    let mut si = 0;
    while si < x && ti < m {
      if s[si] == t[ti] {
        ti += 1;
      }
      si += 1;
    }
    memo[i].0 = ti;

    let mut ti = m as isize - 1;
    let mut si = x as isize - 1;
    while 0 <= ti && 0 <= si {
      if s[si as usize] == t[ti as usize] {
        ti -= 1;
      }
      si -= 1;
    }

    adds[(ti+1) as usize] += 1;
    memo[i].1 = (ti + 1) as usize;
  }

  let mut memo2 = vec![0;m+1];
  memo2[0] = adds[0];
  for i in 1..=m {
    memo2[i] = memo2[i-1] + adds[i];
  }
  
  let mut result = 0usize;
  for (ti, _) in memo {
    result += memo2[ti];
  }

  println!("{}", result);
}