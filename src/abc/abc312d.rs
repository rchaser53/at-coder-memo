/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    s:Chars
  }

  let limit = 3000;
  let mut memo = vec![0;limit+10];
  memo[0] = 1;
  for c in s {
    let mut new_memo = vec![0;limit+10];
    if c == '(' {
      for i in 0..limit {
        new_memo[i+1] = memo[i];
      }
    } else if c == ')' {
      for i in 1..limit {
        new_memo[i-1] = memo[i];
      }
    } else {
      for i in 0..limit {
        new_memo[i+1] += memo[i];
      }
      for i in 1..limit {
        new_memo[i-1] += memo[i];
        new_memo[i-1] %= MOD;
      }
    }
    memo = new_memo;
  }

  println!("{}", memo[0]);
}