/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

pub trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T] where T: Ord {
  fn next_permutation(&mut self) -> bool {
    if self.len() < 2 { return false; }
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] >= self[i] {
        i -= 1;
      }

      if i == 0 {
        return false;
      }

      let mut j = self.len() - 1;
      while j >= i && self[j] <= self[i-1]  {
        j -= 1;
      }

      self.swap(j, i-1);

      self[i..].reverse();

      true
    }
}


fn main() {
  input! {
    n:usize,
    m:usize,
    uvw:[(Usize1,Usize1,usize);m]
  }

  let inf = 10usize.pow(20);
  let mut g = vec![vec![inf;n];n];
  for (u,v,w) in uvw {
    g[u][v] = w;
    g[v][u] = w;
  }

  let mut min = inf;
  let mut p = (1..n).into_iter().collect::<Vec<_>>();

  loop {
    let next: Vec<usize> = p.iter().map(|v| *v).collect();
    
    let mut success = true;
    let mut v = 0;
    let mut ci = 0;
    for &j in next.iter() {
      let nv = g[ci][j];
      if nv == inf {
        success = false;
        break;
      }
      v ^= nv;
      ci = j;
      if ci == n-1 {
        break
      }
    }
    
    if success {
      min = std::cmp::min(v, min);
    }
    
    if !p.next_permutation() {
      break
    }
  } 
  println!("{min}");
}