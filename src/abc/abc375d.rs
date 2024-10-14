/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }

  let n = s.len();
  let mut result = 0;
  let mut memo = vec![vec![];26];
  for i in 0..n {
    memo[s[i] as usize - 'A' as usize].push(i);
  }

  for i in 0..26 {
    let m = memo[i].len();
    if m <= 1 { continue }
    let mut temp = 0;
    let e1 = memo[i][0];
    for j in 1..m {
      temp += memo[i][j] - e1 - 1;
    }

    for j in 0..m-1 {
      result += temp;
      let left = m - j - 1;
      let diff = memo[i][j+1] - memo[i][j];
      temp += 1;
      temp -= left * diff;
    }
  }

  println!("{}", result);
}