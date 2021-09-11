use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  
  let limit = 1 << n;
  let mut memo = vec![0usize;limit];
  memo[0] = 1;

  for i in 0..limit {
    for j in 0..n {
      if i >> j & 1 == 1 {
        continue
      }

      let mut flag = true;
      for &(a, b) in &vals {
        if (i >> a) & 1 != 1 && (i >> b) & 1 == 1 {
          flag = false;
          break
        }
      }
      if flag {
        memo[i | (1 << j)] += memo[i];
      }
    }
  }
  println!("{}", memo[limit-1]);
}