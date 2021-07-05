/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars,
  }

  let inf = 1_000_000_000;
  let mut memo = vec![inf;n];

  let mut ai = inf;   // 'o'
  let mut bi = inf;   // 'x'
  if s[n-1] == 'o' {
    ai = n-1;
  } else {
    bi = n-1;
  }
  for i in (0..n-1).rev() {
    if s[i] == 'o' {
      memo[i] = bi;
      ai = i;
    } else {
      memo[i] = ai;
      bi = i;
    }
  }

  let mut result = 0usize;
  for i in 0..n {
    let v = memo[i];
    if v != inf {
      result += n - v;
    }
  }
  println!("{}", result);
}
