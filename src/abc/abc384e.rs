/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn helper(rows: &Vec<Vec<usize>>, heap:&mut BinaryHeap<(Reverse<usize>,usize,usize)>, memo: &mut Vec<Vec<bool>>, ci:usize, cj:usize) {
  let h = rows.len();
  let w = rows[0].len();
  
  if 0 < ci && !memo[ci-1][cj] {
    heap.push((Reverse(rows[ci-1][cj]), ci-1, cj));
    memo[ci-1][cj] = true;
  }

  if ci < h-1 && !memo[ci+1][cj] {
    heap.push((Reverse(rows[ci+1][cj]), ci+1, cj));
    memo[ci+1][cj] = true;
  }

  if 0 < cj && !memo[ci][cj-1] {
    heap.push((Reverse(rows[ci][cj-1]), ci, cj-1));
    memo[ci][cj-1] = true;
  }

  if cj < w-1 && !memo[ci][cj+1] {
    heap.push((Reverse(rows[ci][cj+1]), ci, cj+1));
    memo[ci][cj+1] = true;
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    x:usize,
    p:Usize1,
    q:Usize1,
    rows:[[usize;w];h]
  }

  let mut heap = BinaryHeap::new();
  let mut memo = vec![vec![false;w];h];
  memo[p][q] = true;  
  
  helper(&rows, &mut heap, &mut memo, p, q);
  let mut now = rows[p][q];
  let mut dirty = true;

  while dirty {
    dirty = false;
    if let Some((Reverse(nv),ni,nj)) = heap.pop() {
      let has_r = now % x > 0;
      if has_r {
        if nv <= now / x {
          now += nv;
          helper(&rows, &mut heap, &mut memo, ni, nj);
          dirty = true;
        }
      } else {
        if nv < now / x {
          now += nv;
          helper(&rows, &mut heap, &mut memo, ni, nj);
          dirty = true;
        }
      }
    }
  }
  
  println!("{}", now);
}