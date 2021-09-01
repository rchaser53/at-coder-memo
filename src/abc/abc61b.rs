use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }

  let mut sets = vec![Vec::new();n];
  for (a,b) in vals {
    sets[a].push(b);
    sets[b].push(a);
  }

  for v in sets {
    println!("{}", v.len());
  }

}
