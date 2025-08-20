/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut s:Chars,
    t:Chars,
    lr:[(Usize1,Usize1);m]
  }

  let mut memo = vec![(0,0);n];
  for (l,r) in lr {
    memo[l].0 += 1;
    memo[r].1 += 1;
  }

  let mut temp = 0;
  for i in 0..n {
    temp += memo[i].0;
    if temp % 2 == 1 {
      s[i] = t[i];
    }    
    temp -= memo[i].1;
  }

  println!("{}", s.iter().collect::<String>());
}