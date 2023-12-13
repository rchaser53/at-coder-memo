/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    l:usize,
    a:[usize;n],
    b:[usize;m],
    cd:[(Usize1,Usize1);l]
  }

  let set = cd.into_iter().collect::<HashSet<(usize,usize)>>();

  let mut b = b.into_iter().enumerate().collect::<Vec<(usize,usize)>>();
  b.sort_by(|a,b| a.1.cmp(&b.1));

  let mut dict = vec![0;m];
  for i in 0..m {
    dict[i] = b[i].0;
  }

  let finished = 1_000_000_000;
  let mut memo = vec![m-1;n];
  let mut heap = BinaryHeap::new();
  for i in 0..n {
    heap.push((a[i] + b[m-1].1, i, b[m-1].0));
  }

  while let Some((v, i, j)) = heap.pop() {
    if set.contains(&(i,j)) {
      if memo[i] != finished {
        memo[i] -= 1;
        if i == 0 {
          memo[i] = finished;
        } else {
          let nj = dict[memo[i]];
          let nv = a[i] + b[memo[i]].1;
          heap.push((nv, i, nj));
        }
      }
    } else {
      println!("{}", v);
      return 
    }
  }
}
