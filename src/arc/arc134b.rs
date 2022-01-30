/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut t = vec![0u8;n];
  let mut memo = vec![vec![];26];
  for i in 0..n {
    let ti = s[i] as u8 - 'a' as u8;
    t[i] = ti;
    memo[ti as usize].push(i);
  }

  let mut li = 0;
  let mut ri = n-1;
  while li < ri {
    let ci = t[li] as usize;
    for i in 0..ci {
      let mut success = false;
      while let Some(ti) = memo[i].pop() {
        if li < ti && ti <= ri {
          t.swap(li, ti);
          ri = ti - 1;
          success = true;
          break
        }
      }
      if success { break }
    }
    li += 1;
  }

  println!("{}", t.into_iter().map(|v| ((v + 'a' as u8) as char).to_string()).collect::<String>());
}