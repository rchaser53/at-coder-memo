/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize
  }

  for _ in 0..t {
    input! {
      n:usize,
      A:[Usize1;2*n]
    }

    let mut result = HashSet::new();
    let mut pos = vec![vec![];n];
    (0..n*2).for_each(|i| pos[A[i]].push(i));
    for i in 0..n*2-1 {
      let a = A[i];
      let b = A[i+1];
      if pos[a][0] + 1 == pos[a][1] || pos[b][0]+1 == pos[b][1] {
        continue
      }
      let mut p = vec![pos[a][0], pos[a][1], pos[b][0], pos[b][1]];
      p.sort();
      if p[0] + 1 == p[1] && p[2]+1 == p[3] {
        if a < b {
          result.insert((a,b));
        } else {
          result.insert((b,a));
        }
      }
    }

    println!("{}", result.len());
  }
}