/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;


fn main() {
  input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
  }

  if m != n {
    println!("No");
    return;
  }

  let mut g = vec![vec![];n];
  for (a,b) in ab {
    g[a].push(b);
    g[b].push(a);
  }

  for i in 0..n {
    if g[i].len() != 2 {
      println!("No");
      return;
    }
  }


  let mut seen = vec![false;n];
  let mut ci = 0;
  let mut li = 10usize.pow(9);
  while !seen[ci] {
    seen[ci] = true;
    for &ni in &g[ci] {
      if li != ni {
        li = ci;
        ci = ni;
        break;
      }
    }
  }

  for f in seen {
    if !f {
      println!("No");
      return;
    }
  }
  println!("Yes");
}