/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:Chars,
    n:usize
  }

  let len = t.len();
  let inf = 1_000_000_000;
  let mut memo = vec![inf;len+1];
  memo[0] = 0;

  for _ in 0..n {
    input! {
      a:usize,
      s:[Chars;a]
    }

    let mut new_memo = memo.clone();
    for i in 0..len {
      if memo[i] == inf { continue }
      for ss in &s {
        let m = ss.len();
        if len < i + m { continue }
        let nv = memo[i]+1;
        if new_memo[i+m] <= nv { continue }

        let mut success = true;
        for j in 0..m {
          if ss[j] != t[i+j] {
            success = false;
            break
          }
        }

        if success {
          new_memo[i+m] = nv;
        }
      }
    }
    memo = new_memo;
  }

  if memo[len] != inf {
    println!("{}", memo[len]);
  } else {
    println!("-1");
  }
}