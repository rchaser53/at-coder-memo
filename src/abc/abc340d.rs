/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    abx:[(usize,usize,Usize1);n-1]
  }

  let inf = 10usize.pow(15);
  let mut memo = vec![inf;n];
  memo[0] = 0;

  let mut g = vec![HashMap::new();n+1];
  for i in 0..n-1 {
    let (a,b,x) = abx[i];

    let e1 = g[i].entry(i+1).or_insert(inf);
    if a < *e1 {
      *e1 = a;
    }

    let e2 = g[i].entry(x).or_insert(inf);
    if b < *e2 {
      *e2 = b;
    }
  }

  let mut heap = BinaryHeap::new();
  heap.push(Reverse((0,0)));
  while let Some(Reverse((cv,ci))) = heap.pop() {
    for (&ni, &av) in &g[ci] {
      let nv = av + cv;
      if nv < memo[ni] {
        memo[ni] = nv;
        heap.push(Reverse((nv,ni)));
      }
    }
  }
  
  println!("{}", memo[n-1]);
}